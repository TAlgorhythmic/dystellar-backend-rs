use std::{collections::HashMap, error::Error, ops::Deref, sync::{Arc, LazyLock}, time::Duration};

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;
use tokio::sync::Mutex;

use crate::api::{typedef::{Method, Router, SigninState}, utils::{get_body, response}};

const PENDING: LazyLock<Arc<Mutex<HashMap<Box<str>, SigninState>>>> = LazyLock::new(|| {Arc::new(Mutex::new(HashMap::new()))});

async fn loginsession(req: Request<Incoming>, _: HashMap<Box<str>, Box<str>>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    let body = get_body(req).await?;
    let uuidopt = body["uuid"].as_str();
    if uuidopt.is_none() {
        return Err("Malformed body".into());
    }

    let uuid = uuidopt.unwrap().to_owned();

    let pend = PENDING.clone();

    let mut guard = pend.lock().await;

    guard.insert(uuid.clone().into(), SigninState::new());

    let pend_cl = pend.clone();
    let uuid_clone = uuid.clone();

    tokio::task::spawn(async move {
        tokio::time::sleep(Duration::from_secs(120)).await;
        
        let mut guard = pend_cl.lock().await;
        guard.remove(uuid_clone.as_str());
    });
    Ok(response(object! { ok: true }))
}

async fn login(_: Request<Incoming>, args: HashMap<Box<str>, Box<str>>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    let pend = PENDING;
    let arg = args.get("uuid");
    if arg.is_none() {
        return Err("Invalid params".into());
    }

    let guard = pend.lock().await;

    let uuid = arg.unwrap();
    let state = guard.get(uuid);

    if state.is_none() {
        return Err("Login session expired.".into());
    }

    if !state.unwrap().is_authenticated() {
        return Ok(response(object! { ok: true, authenticated: false }));
    }

    let res = state.unwrap();

    Ok(response(object! { ok: true, authenticated: true, code: res.get_code().as_ref().unwrap().deref() }))
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
    
    let mut guard = pend.lock().await;
    let opt = guard.get_mut(uuid);
    if opt.is_none() {
        return Err("Invalid state.".into());
    }

    let signin_state: &mut SigninState = opt.unwrap();
    signin_state.set_authenticated(true);
    signin_state.set_code(code);
    Ok(response(object! { ok: true, msg: "Login successful! You can now close this tab." }))
}

pub async fn register(rout: &Arc<Mutex<Router>>) {
    let mut router = rout.lock().await;

    router.endpoint(
        Method::Get, 
        "/api/microsoft/callback",
        Box::new(|req, args| {Box::pin(callback(req, args))})
    ).expect("Failed to register microsoft callback endpoint");

    router.endpoint(
        Method::Get,
        "/api/microsoft/login",
        Box::new(|req, args| {Box::pin(login(req, args))})
    ).expect("Failed to register microsoft login endpoint");

    router.endpoint(
        Method::Post,
        "/api/microsoft/loginsession",
        Box::new(|req, args| {Box::pin(loginsession(req, args))})
    ).expect("Failed to register microsoft login session endpoint");
}
