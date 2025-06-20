use std::sync::Arc;

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, header::{AUTHORIZATION, CONTENT_TYPE}, Request, Response};
use json::{array, object};
use tokio::sync::Mutex;

use crate::api::{control::sql::query::get_player_from_uuid, typedef::{BackendError, Method, Router}, utils::{get_body_json, get_body_url_args, response_json, HttpTransaction}};

static TOKEN: &str = env!("PRIVILEGE_TOKEN");

fn check_token(transaction: HttpTransaction) -> Result<(), BackendError> {
    let http = match transaction {
        HttpTransaction::Req(req) => req.headers().to_owned(),
        HttpTransaction::Res(res) => res.headers().to_owned()
    };


    if let Some(h) = header && h.to_str().unwrap() == TOKEN {
        return Ok(());
    }

    Err(BackendError::new("Operation not permitted.", 401))
}

/**
* A simple endpoint that returns an ok response, used to check the status of the backend if its
* running or not
*/
async fn player_data(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, BackendError> {
    let transaction = HttpTransaction::Req(req);
    check_token(transaction)?;
    
    let args = get_body_url_args(&req).await?;
    let uuid = args.get("uuid").ok_or_else(|| BackendError::new("Malformed url, uuid expected", 400))?;
    
    let data_res = get_player_from_uuid(uuid).await;
    if let Err(err) = &data_res {
        return Err(BackendError::new(err.to_string().as_str(), 500));
    }

    let data = data_res.unwrap();
    let obj = object! {
        ok: true,
        data: data_res.map(|v| array![ v.to_json_complete() ]).or(array![])
    };

    Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, TOKEN)
        .body(Full::new(Bytes::from(stringify(obj))))
        .unwrap()
}

pub async fn register(rout: &Arc<Mutex<Router>>) {
    let mut router = rout.lock().await;

    router.endpoint(Method::Get,
        "/api/privileged/player_data",
        Box::new(|req| {Box::pin(player_data(req))})
    ).expect("Failed to register status endpoint");
}
