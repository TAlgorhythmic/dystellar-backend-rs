use std::error::Error;

use json::object;

use crate::api::{typedef::{MicrosoftTokens, MinecraftLoginData, XboxLiveTokensData}, utils::{get_body_json, HttpTransaction}};

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
* Handle all minecraft login stuff and return relevant information
*/
pub async fn login_minecraft(code: &str) -> Result<MinecraftLoginData, Box<dyn Error + Send + Sync>> {
    let tokens_json = get_microsoft_tokens(code).await?;

    let expiration = opt_expiration.unwrap();
    let access_token = opt_access_token.unwrap();
    let refresh_token = opt_refresh_token.unwrap();

    let xsts_res = post_json("https://xsts.auth.xboxlive.com/xsts/authorize", object! {
        Properties: object! {
            SandboxId: "RETAIL",
            UserTokens
        },
        RelyingParty: "rp://api.minecraftservices.com/",
        TokenType: "JWT"
    })
}

/**
* Handle minecraft login stuff from a token/refresh_token
*/
pub async fn login_minecraft_existing(access_token: &str, refresh_token: &str) -> Result<MinecraftLoginData, Box<dyn Error + Send + Sync>> {
    
}
