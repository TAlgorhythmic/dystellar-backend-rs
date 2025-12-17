use hyper::header::{HeaderValue, AUTHORIZATION};
use json::{array, object};

use crate::api::{control::http::get_json, typedef::*, utils::{get_body_json, HttpTransaction}};

use super::http::{post_json, post_urlencoded};

static CLIENT_ID: &str = env!("CLIENT_ID");
static CLIENT_SECRET: &str = env!("CLIENT_SECRET");
static REDIRECT_URI: &str = env!("REDIRECT_URI");

/**
* Fetch microsoft oauth2 login token and refresh_token
*/
pub async fn get_microsoft_tokens(code: &str) -> Result<MicrosoftTokens, BackendError> {
    let auth_res = post_urlencoded(
        "https://login.microsoftonline.com/consumers/oauth2/v2.0/token",
        format!(
            "client_id={CLIENT_ID}&client_secret={CLIENT_SECRET}&code={code}&grant_type=authorization_code&redirect_uri={REDIRECT_URI}"
        )
    ).await;

    if let Err(auth_res) = &auth_res {
        return Err(BackendError::new(auth_res.to_string().as_ref(), 500));
    }

    let tokens_json = get_body_json(HttpTransaction::Res(auth_res.unwrap())).await?;

    let opt_expiration = tokens_json["expires_in"].as_i64();
    let opt_access_token = tokens_json["access_token"].as_str();
    let opt_refresh_token = tokens_json["refresh_token"].as_str();
    if opt_expiration.is_none() || opt_access_token.is_none() || opt_refresh_token.is_none() {
        return Err(BackendError::new("Failed to fetch microsoft tokens, either an internal error occurred or the code token expired", 400));
    }

    Ok(MicrosoftTokens::new(opt_access_token.unwrap().into(), opt_refresh_token.unwrap().into()))
}

/**
* Fetch xbox tokens to exchange later for xsts
*/
pub async fn get_xbox_live_data(access_token: &str) -> Result<XboxLiveTokensData, BackendError> {
    let xbox_res = post_json("https://user.auth.xboxlive.com/user/authenticate", object! {
        Properties: object! {
            AuthMethod: "RPS",
            SiteName: "user.auth.xboxlive.com",
            RpsTicket: format!("d={access_token}")
        },
        RelyingParty: "http://auth.xboxlive.com",
        TokenType: "JWT"
    }).await;

    if let Err(err) = &xbox_res {
        return Err(BackendError::new(err.to_string().as_ref(), 500));
    }

    let body = get_body_json(HttpTransaction::Res(xbox_res.unwrap())).await?;
    let opt_token = body["Token"].as_str();
    let opt_uhs = body["DisplayClaims"]["xui"][0]["uhs"].as_str();

    if opt_token.is_none() || opt_uhs.is_none() {
        return Err(BackendError::new("Xbox live data response is incomplete, probably an error occurred.", 400));
    }

    let token = opt_token.unwrap();
    let uhs = opt_uhs.unwrap();
    Ok(XboxLiveTokensData::new(token.into(), uhs.into()))
}

/**
* Get a new microsoft oauth2 access token from a refresh token.
*/
pub async fn refresh_access_token(refresh_token: &str) -> Result<MicrosoftTokens, BackendError> {
    let auth_res = post_urlencoded(
        "https://login.live.com/oauth20_token.srf",
        format!(
            "client_id={CLIENT_ID}&client_secret={CLIENT_SECRET}&refresh_token={refresh_token}&grant_type=refresh_token&redirect_uri={REDIRECT_URI}"
        )
    ).await;

    if let Err(err) = &auth_res {
        return Err(BackendError::new(err.to_string().as_ref(), 500));
    }

    let tokens_json = get_body_json(HttpTransaction::Res(auth_res.unwrap())).await?;

    let opt_expiration = tokens_json["expires_in"].as_i64();
    let opt_access_token = tokens_json["access_token"].as_str();
    let opt_refresh_token = tokens_json["refresh_token"].as_str();
    if opt_expiration.is_none() || opt_access_token.is_none() || opt_refresh_token.is_none() {
        return Err(BackendError::new("Failed to fetch microsoft tokens, either an internal error occurred or the code token expired", 400));
    }

    Ok(MicrosoftTokens::new(opt_access_token.unwrap().into(), opt_refresh_token.unwrap().into()))
}

pub async fn get_xbox_xts_data(xbox_live_token: &str) -> Result<XstsData, BackendError> {
    let xsts_res = post_json("https://xsts.auth.xboxlive.com/xsts/authorize", object! {
        Properties: object! {
            SandboxId: "RETAIL",
            UserTokens: array![ xbox_live_token ]
        },
        RelyingParty: "rp://api.minecraftservices.com/",
        TokenType: "JWT"
    }).await;

    if let Err(err) = &xsts_res {
        return Err(BackendError::new(err.to_string().as_ref(), 500));
    }

    let body = get_body_json(HttpTransaction::Res(xsts_res.unwrap())).await?;

    let token = body["Token"].as_str();
    let uhs = body["DisplayClaims"]["xui"][0]["uhs"].as_str();
    let xuid = body["DisplayClaims"]["xui"][0]["xid"].as_str();

    if token.is_none() || uhs.is_none() || xuid.is_none() {
        return Err(BackendError::new("Failed to get XSTS data", 400));
    }

    Ok(XstsData { token: token.unwrap().into(), uhs: uhs.unwrap().into(), xuid: xuid.unwrap().into() })
}

pub async fn get_minecraft_token(uhs: &str, xsts_token: &str) -> Result<MinecraftData, BackendError> {
    let token_res = post_json("https://api.minecraftservices.com/authentication/login_with_xbox", object! {
        identityToken: format!("XBL3.0 x={uhs};{xsts_token}"),
        ensureLegacyEnabled: true
    }).await;

    if let Err(err) = &token_res {
        return Err(BackendError::new(err.to_string().as_ref(), 500));
    }

    let body = get_body_json(HttpTransaction::Res(token_res.unwrap())).await?;

    let opt_username = body["username"].as_str();
    let opt_token = body["access_token"].as_str();
    let opt_expires = body["expires_in"].as_i64();

    if opt_username.is_none() || opt_token.is_none() || opt_expires.is_none() {
        return Err(BackendError::new("Failed to get minecraft data", 400));
    }

    Ok(MinecraftData::new(opt_username.unwrap(), opt_token.unwrap(), opt_expires.unwrap()))
}

/**
* Handle all minecraft login stuff and return relevant information
*/
pub async fn login_minecraft(code: &str) -> Result<UserCredentials, BackendError> {
    let tokens = get_microsoft_tokens(code).await?;
    let xbox_data = get_xbox_live_data(tokens.get_token()).await?;
    let xsts_data = get_xbox_xts_data(xbox_data.get_token()).await?;
    let minecraft_data = get_minecraft_token(xbox_data.get_uhs(), xsts_data.token.as_ref()).await?;
    let name = get_minecraft_username(minecraft_data.get_token(), &minecraft_data.uuid).await?;

    Ok(UserCredentials::new(
        minecraft_data.uuid,
        name,
        minecraft_data.token,
        tokens.access_token,
        tokens.refresh_token,
        xsts_data.uhs,
        xsts_data.xuid,
        minecraft_data.expires
    ))
}

pub async fn get_minecraft_username(mc_token: &str, uuid: &str) -> Result<Box<str>, BackendError> {
    let payload = get_json(
        format!("https://api.minecraftservices.com/entitlements/license?requestId={uuid}").as_str(),
        Some(&[(AUTHORIZATION, HeaderValue::from_str(format!("Bearer {mc_token}").as_str()).unwrap())])
    ).await;

    if payload.is_err() {
        return Err(BackendError::new("Failed to get entitlements", 500));
    }
    
    let payload = get_body_json(HttpTransaction::Res(payload.unwrap())).await?;
    if payload["items"].members().find(|p| {
        if let Some(name) = p["name"].as_str() {
            return name == "product_minecraft" || name == "game_minecraft";
        }
        false
    }).is_none() {
        return Err(BackendError::new("Buy minecraft at official site first.", 401));
    }

    let res = get_json(
        "https://api.minecraftservices.com/minecraft/profile",
        Some(&[(AUTHORIZATION, HeaderValue::from_str(format!("Bearer {mc_token}").as_str()).unwrap())])
    ).await;

    if res.is_err() {
        return Err(BackendError::new("Failed to get username", 400));
    }

    let json = get_body_json(HttpTransaction::Res(res.unwrap())).await?;
    let name = json["name"].as_str();
    
    if name.is_none() {
        return Err(BackendError::new("Failed to get username, json format differs from expected", 500));
    }

    Ok(name.unwrap().into())
}

/**
* Handle minecraft login stuff from a token/refresh_token
*/
pub async fn login_minecraft_existing(mut tokens: MicrosoftTokens) -> Result<UserCredentials, BackendError> {
    // Refresh tokens if access token is expired
    let xbox_data = {
        let xbox_data_res = get_xbox_live_data(tokens.get_refresh_token()).await;

        if xbox_data_res.is_ok() {
            xbox_data_res.unwrap()
        } else {
            let refresh = refresh_access_token(tokens.get_refresh_token()).await?;
            tokens.set_token(refresh.access_token);
            tokens.set_refresh_token(refresh.refresh_token);

            get_xbox_live_data(tokens.get_token()).await?
        }
    };

    let xsts_data = get_xbox_xts_data(xbox_data.get_token()).await?;
    let minecraft_data = get_minecraft_token(xbox_data.get_uhs(), xsts_data.token.as_ref()).await?;
    let name = get_minecraft_username(minecraft_data.get_token(), &minecraft_data.uuid).await?;

    Ok(UserCredentials::new(
        minecraft_data.uuid,
        name,
        minecraft_data.token,
        tokens.access_token,
        tokens.refresh_token,
        xsts_data.uhs,
        xsts_data.xuid,
        minecraft_data.expires
    ))
}
