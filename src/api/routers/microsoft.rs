use std::{collections::HashMap, error::Error, sync::{Arc, LazyLock}};

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;

use crate::api::{typedef::Method, utils::response};

use super::router;

const PENDING: LazyLock<Arc<HashMap<&str, bool>>> = LazyLock::new(|| {Arc::new(HashMap::new())});

async fn login(_: Request<Incoming>, args: HashMap<&str, &str>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    let pend = PENDING;
    let arg = args.get("uuid");
    if arg.is_none() {
        return Err("Invalid params".into());
    }

    let uuid = arg.unwrap();
    let state = pend.get(uuid);
    if state.is_none() || !state.unwrap() {
        return Ok(response(object! { ok: true, authenticated: false }));
    }

    Ok(response(object! { ok: true, authenticated: true }))
}

async fn callback(req: Request<Incoming>, args: HashMap<&str, &str>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>>{
    
}

pub async fn register() {
    router().endpoint(Method::Get, "/api/microsoft/callback", Box::new(|req, args| {Box::new(callback(req, args))}));
    router().endpoint(Method::Get, "/api/microsoft/login", Box::new(|req, args| {Box::new(login(req, args))}));
}
