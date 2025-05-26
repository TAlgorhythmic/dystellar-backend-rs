use std::error::Error;

use json::{array, object};

use crate::api::{typedef::{MicrosoftTokens, MinecraftData, UserCredentials, XboxLiveTokensData}, utils::{get_body_json, HttpTransaction}};

use super::http::{post_json, post_urlencoded};

static CLIENT_ID: &str = env!("CLIENT_ID");
static CLIENT_SECRET: &str = env!("CLIENT_SECRET");
static REDIRECT_URI: &str = env!("REDIRECT_URI");

/**
* Fetch microsoft oauth2 login token and refresh_token
*/
pub async fn get_microsoft_tokens(code: &str) -> Result<MicrosoftTokens, Box<dyn Error + Send + Sync>> {
    let auth_res = post_urlencoded(
        "https://login.live.com/oauth20_token.srf",
        format!(
            "client_id={CLIENT_ID}&client_secret={CLIENT_SECRET}&code={code}&grant_type=authorization_code&redirect_uri={REDIRECT_URI}"
        )
    ).await?;

    let tokens_json = get_body_json(HttpTransaction::Res(auth_res)).await?;

    let opt_expiration = tokens_json["expires_in"].as_i64();
    let opt_access_token = tokens_json["access_token"].as_str();
    let opt_refresh_token = tokens_json["refresh_token"].as_str();
    if opt_expiration.is_none() || opt_access_token.is_none() || opt_refresh_token.is_none() {
        return Err("Failed to fetch microsoft tokens, either an internal error occurred or the code token expired".into());
    }

    Ok(MicrosoftTokens::new(opt_access_token.unwrap().into(), opt_refresh_token.unwrap().into(), opt_expiration.unwrap()))
}

/**
* Fetch xbox tokens to exchange later for xsts
*/
pub async fn get_xbox_live_data(access_token: &str) -> Result<XboxLiveTokensData, Box<dyn Error + Send + Sync>> {
    let xbox_res = post_json("https://user.auth.xboxlive.com/user/authenticate", object! {
        Properties: object! {
            AuthMethod: "RPS",
            SiteName: "user.auth.xboxlive.com",
            RpsTicket: access_token
        },
        RelyingParty: "http://auth.xboxlive.com",
        TokenType: "JWT"
    }).await?;

    let body = get_body_json(HttpTransaction::Res(xbox_res)).await?;
    let opt_token = body["Token"].as_str();
    let opt_uhs = body["DisplayClaims"]["xui"][0]["uhs"].as_str();

    if opt_token.is_none() || opt_uhs.is_none() {
        return Err("Xbox live data response is incomplete, probably an error occurred.".into());
    }

    let token = opt_token.unwrap();
    let uhs = opt_uhs.unwrap();
    Ok(XboxLiveTokensData::new(token.into(), uhs.into()))
}

/**
* Get a new microsoft oauth2 access token from a refresh token.
*/
pub async fn refresh_access_token(refresh_token: &str) -> Result<MicrosoftTokens, Box<dyn Error + Send + Sync>> {
    let auth_res = post_urlencoded(
        "https://login.live.com/oauth20_token.srf",
        format!(
            "client_id={CLIENT_ID}&client_secret={CLIENT_SECRET}&refresh_token={refresh_token}&grant_type=refresh_token&redirect_uri={REDIRECT_URI}"
        )
    ).await?;

    let tokens_json = get_body_json(HttpTransaction::Res(auth_res)).await?;

    let opt_expiration = tokens_json["expires_in"].as_i64();
    let opt_access_token = tokens_json["access_token"].as_str();
    let opt_refresh_token = tokens_json["refresh_token"].as_str();
    if opt_expiration.is_none() || opt_access_token.is_none() || opt_refresh_token.is_none() {
        return Err("Failed to fetch microsoft tokens, either an internal error occurred or the code token expired".into());
    }

    Ok(MicrosoftTokens::new(opt_access_token.unwrap().into(), opt_refresh_token.unwrap().into(), opt_expiration.unwrap()))
}

pub async fn get_xbox_xts_token(xbox_live_token: &str) -> Result<Box<str>, Box<dyn Error + Send + Sync>> {
    let xsts_res = post_json("https://xsts.auth.xboxlive.com/xsts/authorize", object! {
        Properties: object! {
            SandboxId: "RETAIL",
            UserTokens: array![ xbox_live_token ]
        },
        RelyingParty: "rp://api.minecraftservices.com/",
        TokenType: "JWT"
    }).await?;

    let body = get_body_json(HttpTransaction::Res(xsts_res)).await?;

    let opt = body["Token"].as_str();
    if opt.is_none() {
        return Err("Failed to get xsts token from microsoft".into());
    }

    Ok(opt.unwrap().into())
}

pub async fn get_minecraft_token(uhs: &str, xsts_token: &str) -> Result<MinecraftData, Box<dyn Error + Send + Sync>> {
    let token_res = post_json("https://api.minecraftservices.com/authentication/login_with_xbox", object! {
        identityToken: format!("XBL3.0 x={uhs};{xsts_token}"),
        ensureLegacyEnabled: true
    }).await?;

    let body = get_body_json(HttpTransaction::Res(token_res)).await?;

    let opt_username = body["username"].as_str();
    let opt_token = body["access_token"].as_str();
    let opt_expires = body["expires_in"].as_i64();

    if opt_username.is_none() || opt_token.is_none() || opt_expires.is_none() {
        return Err("Failed to get minecraft data".into());
    }

    Ok(MinecraftData::new(opt_username.unwrap(), opt_token.unwrap(), opt_expires.unwrap()))
}

/**
* Handle all minecraft login stuff and return relevant information
*/
pub async fn login_minecraft(code: &str) -> Result<UserCredentials, Box<dyn Error + Send + Sync>> {
    let tokens = get_microsoft_tokens(code).await?;
    let xbox_data = get_xbox_live_data(tokens.get_token()).await?;
    let xsts_token = get_xbox_xts_token(xbox_data.get_token()).await?;
    let minecraft_data = get_minecraft_token(xbox_data.get_uhs(), xsts_token.as_ref()).await?;

    Ok(UserCredentials::new(
        minecraft_data.uuid,
        minecraft_data.token,
        tokens.access_token,
        tokens.refresh_token,
        minecraft_data.expires
    ))
}

/**
* Handle minecraft login stuff from a token/refresh_token
*/
pub async fn login_minecraft_existing(mut tokens: MicrosoftTokens) -> Result<UserCredentials, Box<dyn Error + Send + Sync>> {
    // Refresh tokens if access token is expired
    let xbox_data = {
        let xbox_data_res = get_xbox_live_data(tokens.get_refresh_token()).await;

        if xbox_data_res.is_ok() {
            xbox_data_res.unwrap()
        } else {
            let refresh = refresh_access_token(tokens.get_refresh_token()).await?;
            tokens.set_token(refresh.access_token);
            tokens.set_refresh_token(refresh.refresh_token);
            tokens.set_expiration(refresh.expires);

            get_xbox_live_data(tokens.get_token()).await?
        }
    };

    let xsts_token = get_xbox_xts_token(xbox_data.get_token()).await?;
    let minecraft_data = get_minecraft_token(xbox_data.get_uhs(), xsts_token.as_ref()).await?;

    Ok(UserCredentials::new(
        minecraft_data.uuid,
        minecraft_data.token,
        tokens.access_token,
        tokens.refresh_token,
        minecraft_data.expires
    ))
}
