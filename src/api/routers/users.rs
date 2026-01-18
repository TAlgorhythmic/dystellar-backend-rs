use std::{collections::HashMap, convert::Infallible, sync::{Arc, LazyLock}};

use chrono::{DateTime, Utc};
use http_body_util::combinators::BoxBody;
use hyper::{body::{Bytes, Incoming}, header::AUTHORIZATION, Request, Response};
use tokio::sync::Mutex;

use crate::api::{control::storage::query::get_user, routers::ROUTER, typedef::{BackendError, jsonutils::SerializableJson, routing::Method}, utils::{get_body_url_args, response_json}};

pub static TOKENS: LazyLock<Arc<Mutex<HashMap<Box<str>, (Box<str>, DateTime<Utc>)>>>> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

/**
* Get user information, if a valid token is provided it returns full user information,
* otherwise only publicly available information is returned.
* Returns an error if an invalid token is provided.
*/
async fn get(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    let args = get_body_url_args(&req).await?;
    let uuid = args.get("uuid").ok_or(BackendError::new("Malformed url, uuid param is required", 400))?;
    let user = get_user(uuid.as_ref()).map_err(|_| BackendError::new("Failed to get user", 500))?
        .ok_or(BackendError::new("This user does not exist", 404))?;

    let token_header = req.headers().get(AUTHORIZATION);

    if token_header.is_none() {
        return Ok(response_json(user.to_json_reduced()));
    } else {
        let token = token_header.unwrap();
        let token_str = token.to_str().map_err(|_| BackendError::new("Failed to parse header", 500))?;

        let tokens = TOKENS.clone();
        let mut tokens_map = tokens.lock().await;

        if let Some(tupl) = tokens_map.get(token_str) {
            let (saved_uuid, expires_at) = tupl;

            if saved_uuid == uuid {
                if *expires_at < Utc::now() {
                    tokens_map.remove(token_str);
                    return Err(BackendError::new("This token has expired or does not exist", 401));
                }
                return Ok(response_json(user.to_json()));
            }
        }
        return Err(BackendError::new("This token has expired or does not exist", 401));
    }
}

pub async fn register() {
    let mut router = ROUTER.lock().await;

    router.endpoint(Method::Get,
        "/api/users",
        Box::new(|req| {Box::pin(get(req))})
    ).expect("Failed to register status endpoint");
}
