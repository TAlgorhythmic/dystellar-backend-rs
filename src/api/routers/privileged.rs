use std::{convert::Infallible, error::Error, str::from_utf8};

use chrono::DateTime;
use http_body_util::combinators::BoxBody;
use hyper::{body::{Bytes, Incoming}, header::AUTHORIZATION, Request, Response};
use json::{JsonValue, object};

use crate::api::{control::storage::query::{create_punishment, get_all_groups_full, get_default_group_name, get_user, get_user_connected, put_user}, typedef::{BackendError, User, jsonutils::SerializableJson, routing::{Method, nodes::Router}}, utils::{HttpTransaction, get_body_json, get_body_url_args, response_json}};

static TOKEN: &str = env!("PRIVILEGE_TOKEN");
static ALLOWED_IP: &str = env!("PRIVILEGED_AUTHORIZED_IP");

fn check_token(req: &Request<Incoming>) -> Result<(), BackendError> {
    let http = req.headers().to_owned();

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
    check_token(&req)?;
    let json = get_body_json(HttpTransaction::Req(req)).await?;

    let user_uuid = json["user_uuid"].as_str().ok_or(BackendError::new("user_uuid missing", 400))?;
    let r#type = json["type"].as_str().ok_or(BackendError::new("type missing", 400))?;
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

    let pun = create_punishment(user_uuid, title, r#type, creation_date, expiration_date, reason, alsoip, allow_chat, allow_ranked, allow_unranked, allow_join_minigames)?;
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
    check_token(&req)?;
    
    let data = get_user(uuid)?.ok_or(BackendError::new("User not found", 404))?;

    Ok(response_json(data.to_json()))
}

async fn user_connected(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if ALLOWED_IP == req.uri().host().unwrap() {
        return Err(BackendError::new("Operation not permitted.", 401));
    }

    let args = get_body_url_args(&req).await?;
    check_token(&req)?;

    let uuid = args.get("uuid").ok_or(BackendError::new("Falformed url, uuid expected", 400))?;
    let name = args.get("name").ok_or(BackendError::new("Falformed url, uuid expected", 400))?;
    let address = args.get("address").ok_or(BackendError::new("Falformed url, address expected", 400))?;

    let data = get_user_connected(uuid.as_ref(), name.as_ref(), address.as_ref())?;

    Ok(response_json(data.to_json()))
}

async fn user_save(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if ALLOWED_IP == req.uri().host().unwrap() {
        return Err(BackendError::new("Operation not permitted.", 401));
    }
    check_token(&req)?;
    let json = get_body_json(HttpTransaction::Req(req)).await?;

    put_user(&User::from_json(&json)?)?;

    Ok(response_json(object! { ok: true }))
}

async fn get_groups(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if ALLOWED_IP == req.uri().host().unwrap() {
        return Err(BackendError::new("Operation not permitted.", 401));
    }
    check_token(&req)?;

    let default_group = get_default_group_name()?;

    if let Some(g) = default_group {
        Ok(response_json(object! {
            default_group: from_utf8(&g)?,
            groups: JsonValue::Array(get_all_groups_full()?.iter().map(|g| g.to_json()).collect())
        }))
    } else {
        Ok(response_json(object! { groups: JsonValue::Array(get_all_groups_full()?.iter().map(|g| g.to_json()).collect()) }))
    }
}

pub async fn register(router: &mut Router) -> Result<(), Box<dyn Error + Send + Sync>> {
    router.endpoint(Method::Get, "/api/privileged/player_data", player_data)?;
    router.endpoint(Method::Get, "/api/privileged/user_connected", user_connected)?;
    router.endpoint(Method::Post, "/api/privileged/punish", punish)?;
    router.endpoint(Method::Put, "/api/privileged/user_save", user_save)?;
    router.endpoint(Method::Put, "/api/privileged/get_groups", get_groups)?;

    Ok(())
}
