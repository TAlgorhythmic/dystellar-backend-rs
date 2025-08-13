use std::{error::Error, sync::{Arc, LazyLock, Mutex}};

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;

use crate::api::{control::inotify::DirWatcher, routers::ROUTER, typedef::{fs_json::{state::State, Config}, BackendError}, utils::response_json};

async fn launcher(req: Request<Incoming>, state: Arc<Mutex<State>>) -> Result<Response<Full<Bytes>>, BackendError> {
    todo!();
    Ok(response_json(object! { ok: true }))
}

pub async fn register(watcher: &mut DirWatcher) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut router = ROUTER.lock().await;
    let state = State::open("state.json", watcher)?;

    router.endpoint(crate::api::typedef::Method::Get,
        "/launcher",
        Box::new(move |req| {Box::pin(launcher(req, state.clone()))})
    ).expect("Failed to register status endpoint");

    Ok(())
}
