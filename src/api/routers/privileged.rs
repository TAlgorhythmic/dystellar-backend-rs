use std::convert::Infallible;

use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{body::{Bytes, Incoming}, header::{AUTHORIZATION, CONTENT_TYPE}, Request, Response};
use json::{array, object, stringify};

use crate::api::{control::storage::query::get_user, routers::ROUTER, typedef::{BackendError, jsonutils::SerializableJson, routing::Method}, utils::{HttpTransaction, get_body_url_args}};

static TOKEN: &str = env!("PRIVILEGE_TOKEN");
static ALLOWED_IP: &str = env!("PRIVILEGED_AUTHORIZED_IP");

fn check_token(transaction: HttpTransaction) -> Result<(), BackendError> {
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
    check_token(transaction)?;
    
    let data_res = get_user(uuid);
    if let Err(err) = &data_res {
        return Err(BackendError::new(err.to_string().as_str(), 500));
    }

    let data = data_res.unwrap();

    let obj = object! {
        ok: true,
        data: data.map(|v| array![ v.to_json() ]).unwrap_or(array![])
    };

    Ok(Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, TOKEN)
        .body(Full::new(Bytes::from(stringify(obj))).boxed())
        .unwrap())
}

pub async fn register() {
    let mut router = ROUTER.lock().await;

    router.endpoint(Method::Get,
        "/api/privileged/player_data",
        Box::new(|req| {Box::pin(player_data(req))})
    ).expect("Failed to register status endpoint");
}
