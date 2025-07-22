use std::sync::{Arc, LazyLock};

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;
use tokio::sync::Mutex;

use crate::api::{typedef::{config::Config, BackendError, Router}, utils::response_json};

static CONFIG: LazyLock<Arc<std::sync::Mutex<Config>>> = LazyLock::new(|| Config::open("downloads.json").expect("Failed to open downloads.json"));

async fn launcher(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, BackendError> {
    Ok(response_json(object! { ok: true }))
}

pub async fn register(rout: &Arc<Mutex<Router>>) {
    let mut router = rout.lock().await;

    router.endpoint(crate::api::typedef::Method::Get,
        "/launcher",
        Box::new(|req| {Box::pin(launcher(req))})
    ).expect("Failed to register status endpoint");
}
