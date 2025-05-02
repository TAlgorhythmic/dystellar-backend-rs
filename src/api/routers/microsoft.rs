use std::{collections::HashMap, error::Error, sync::{Arc, LazyLock, RwLock}};

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;

use crate::api::{typedef::{Method, SigninState}, utils::response};

use super::router;

const PENDING: LazyLock<Arc<RwLock<HashMap<String, SigninState>>>> = LazyLock::new(|| {Arc::new(RwLock::new(HashMap::new()))});

async fn login(_: Request<Incoming>, args: HashMap<String, String>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    let pend = PENDING;
    let arg = args.get("uuid");
    if arg.is_none() {
        return Err("Invalid params".into());
    }

    let guard = pend.read().unwrap();

    let uuid = arg.unwrap();
    let state = guard.get(uuid);
    if state.is_none() || !state.unwrap().is_authenticated() {
        return Ok(response(object! { ok: true, authenticated: false }));
    }

    Ok(response(object! { ok: true, authenticated: true }))
}

async fn callback(_: Request<Incoming>, args: HashMap<Box<str>, Box<str>>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    let arg0 = args.get("code");
    let arg1 = args.get("state");

    if arg0.is_none() || arg1.is_none() {
        return Err("Invalid url params.".into());
    }

    let pend = PENDING;
    let code = arg0.unwrap();
    let uuid = arg1.unwrap();
    
    let mut guard = pend.write().unwrap();
    let opt = guard.get_mut(uuid);
    if opt.is_none() {
        return Err("Invalid state.".into());
    }

    let signin_state: &mut SigninState = opt.unwrap();
    signin_state.set_authenticated(true);
    signin_state.set_code(String::from(*code).as_str());
    Ok(response(object! { ok: true, msg: "Login successful! You can now close this tab." }))
}

pub async fn register() {
    router().endpoint(Method::Get, "/api/microsoft/callback", Box::new(|req, args| {Box::new(callback(req, args))}));
    router().endpoint(Method::Get, "/api/microsoft/login", Box::new(|req, args| {Box::new(login(req, args))}));
}
