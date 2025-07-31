use std::sync::{Arc, LazyLock};

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;
use tokio::sync::Mutex;

use crate::api::{typedef::{fs_json::{state::State, Config}, BackendError, Router}, utils::{response_json, temporary_redirection}};

static CONFIG: LazyLock<Arc<std::sync::Mutex<State>>> = LazyLock::new(|| State::open("downloads.json").expect("Failed to open downloads.json"));

async fn launcher(req: Request<Incoming>, config: Arc<std::sync::Mutex<State>>) -> Result<Response<Full<Bytes>>, BackendError> {
    todo!();
    Ok(response_json(object! { ok: true }))
}

pub async fn register(rout: &Arc<Mutex<Router>>, config: Arc<std::sync::Mutex<State>>) {
    let mut router = rout.lock().await;

    let launcher_clone = config.clone();
    router.endpoint(crate::api::typedef::Method::Get,
        "/launcher",
        Box::new(move |req| {Box::pin(launcher(req, launcher_clone.clone()))})
    ).expect("Failed to register status endpoint");
}
