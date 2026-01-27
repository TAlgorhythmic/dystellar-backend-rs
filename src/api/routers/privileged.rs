use std::convert::Infallible;

use chrono::DateTime;
use http_body_util::combinators::BoxBody;
use hyper::{body::{Bytes, Incoming}, header::AUTHORIZATION, Request, Response};
use json::{array, object};

use crate::api::{control::storage::query::{create_punishment, get_user, get_user_connected}, routers::ROUTER, typedef::{BackendError, jsonutils::SerializableJson, routing::Method}, utils::{HttpTransaction, get_body_json, get_body_url_args, response_json}};

static TOKEN: &str = env!("PRIVILEGE_TOKEN");
static ALLOWED_IP: &str = env!("PRIVILEGED_AUTHORIZED_IP");

fn check_token(transaction: &HttpTransaction) -> Result<(), BackendError> {
    let http = match transaction {
        HttpTransaction::Req(req) => req.headers().to_owned(),
        HttpTransaction::Res(res) => res.headers().to_owned()
    };

    let header = http.get(AUTHORIZATION);
    if let Some(h) = header && h.to_str().unwrap() == TOKEN {
        return Ok(());
    }

    Err(BackendError::new("Operation not permitted.", 401))
}

/**
* Punish a player, this creates a punishment, assigns it to the player and returns it.
*/
async fn punish(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if ALLOWED_IP == req.uri().host().unwrap() {
        return Err(BackendError::new("Operation not permitted.", 401));
    }
    let transaction = HttpTransaction::Req(req);
    check_token(&transaction)?;
    let json = get_body_json(transaction).await?;

    let user_uuid = json["user_uuid"].as_str().ok_or(BackendError::new("user_uuid missing", 400))?;
    let subject_addr = json["subject_addr"].as_str().ok_or(BackendError::new("subject_json missing", 400))?;
    let title = json["title"].as_str().ok_or(BackendError::new("title missing", 400))?;
    let creation_date = DateTime::from_timestamp_millis(
        json["creation_date"].as_i64().ok_or(BackendError::new("creation_date missing", 400))?
    ).ok_or(BackendError::new("creation date is invalid", 400))?;
    let expiration_date = match json["expiration_date"].as_i64() {
        Some(n) => Some(DateTime::from_timestamp_millis(n).ok_or(BackendError::new("expiration date invalid", 400))?),
        _ => None
    };
    let reason = json["reason"].as_str().ok_or(BackendError::new("reason missing", 400))?;
    let alsoip = json["alsoip"].as_bool().unwrap_or(false);
    let allow_chat = json["allow_chat"].as_bool().unwrap_or(false);
    let allow_ranked = json["allow_ranked"].as_bool().unwrap_or(false);
    let allow_unranked = json["allow_unranked"].as_bool().unwrap_or(false);
    let allow_join_minigames = json["allow_join_minigames"].as_bool().unwrap_or(false);

    let pun = create_punishment(user_uuid, subject_addr, title, creation_date, expiration_date, reason, alsoip, allow_chat, allow_ranked, allow_unranked, allow_join_minigames)?;
    Ok(response_json(pun.to_json()))
}

/**
* An endpoint used to get the full data of a user, requires a unique token and being from an
* authorized IP.
*/
async fn player_data(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if ALLOWED_IP == req.uri().host().unwrap() {
        return Err(BackendError::new("Operation not permitted.", 401));
    }

    let args = get_body_url_args(&req).await?;
    let uuid = args.get("uuid").ok_or(BackendError::new("Malformed url, uuid expected", 400))?;

    let transaction = HttpTransaction::Req(req);
    check_token(&transaction)?;
    
    let data = get_user(uuid)?.ok_or(BackendError::new("User not found", 404))?;

    Ok(response_json(data.to_json()))
}

async fn user_connected(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if ALLOWED_IP == req.uri().host().unwrap() {
        return Err(BackendError::new("Operation not permitted.", 401));
    }

    let args = get_body_url_args(&req).await?;
    check_token(&HttpTransaction::Req(req))?;

    let uuid = args.get("uuid").ok_or(BackendError::new("Falformed url, uuid expected", 400))?;
    let name = args.get("name").ok_or(BackendError::new("Falformed url, uuid expected", 400))?;
    let address = args.get("address").ok_or(BackendError::new("Falformed url, address expected", 400))?;

    let data = get_user_connected(uuid.as_ref(), name.as_ref(), address.as_ref())?;

    Ok(response_json(data.to_json()))
}

pub async fn register() {
    let mut router = ROUTER.lock().await;

    router.endpoint(Method::Get,
        "/api/privileged/player_data",
        Box::new(|req| {Box::pin(player_data(req))})
    ).expect("Failed to register status endpoint");
    router.endpoint(Method::Get,
        "/api/privileged/user_connected",
        Box::new(|req| {Box::pin(player_data(req))})
    ).expect("Failed to register status endpoint");
    router.endpoint(Method::Post,
        "/api/privileged/punish",
        Box::new(|req| {Box::pin(punish(req))})
    ).expect("Failed to register punish endpoint");
}
