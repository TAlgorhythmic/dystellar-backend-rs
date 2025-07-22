use std::sync::{Arc, LazyLock};

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;
use tokio::sync::Mutex;

use crate::api::{typedef::{config::Config, BackendError, Router}, utils::{response_json, temporary_redirection}};

static CONFIG: LazyLock<Arc<std::sync::Mutex<Config>>> = LazyLock::new(|| Config::open("downloads.json").expect("Failed to open downloads.json"));

async fn launcher(req: Request<Incoming>, config: Arc<std::sync::Mutex<Config>>) -> Result<Response<Full<Bytes>>, BackendError> {
    Ok(response_json(object! { ok: true }))
}

async fn discord(req: Request<Incoming>, config: Arc<std::sync::Mutex<Config>>) -> Result<Response<Full<Bytes>>, BackendError> {
    let conf = config.lock().unwrap();
    
    Ok(temporary_redirection(&conf.discord_url))
}

pub async fn register(rout: &Arc<Mutex<Router>>, config: Arc<std::sync::Mutex<Config>>) {
    let mut router = rout.lock().await;

    let launcher_clone = config.clone();
    router.endpoint(crate::api::typedef::Method::Get,
        "/launcher",
        Box::new(move |req| {Box::pin(launcher(req, launcher_clone.clone()))})
    ).expect("Failed to register status endpoint");

    let discord_clone = config.clone();
    router.endpoint(crate::api::typedef::Method::Get,
        "/discord",
        Box::new(move |req| {Box::pin(discord(req, discord_clone.clone()))})
    ).expect("Failed to register status endpoint");
}
