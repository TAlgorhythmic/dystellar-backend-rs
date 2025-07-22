use std::sync::{Arc, LazyLock};

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;
use tokio::sync::Mutex;

use crate::api::{typedef::{config::Config, BackendError, Router}, utils::response_json};

static CONFIG: LazyLock<Mutex<Config>> = LazyLock::new(|| Mutex::new(Config::open("downloads.json").expect("Failed to open downloads.json")));

async fn status(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, BackendError> {
    Ok(response_json(object! { ok: true }))
}

pub async fn launcher(rout: &Arc<Mutex<Router>>) {
    let mut router = rout.lock().await;

    router.endpoint(crate::api::typedef::Method::Get,
        "/api/downloads/launcher",
        Box::new(|req| {Box::pin(status(req))})
    ).expect("Failed to register status endpoint");
}
