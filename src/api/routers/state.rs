use std::{convert::Infallible, error::Error, sync::{Arc, Mutex}};

use http_body_util::{combinators::BoxBody};
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;

use crate::api::{control::inotify::DirWatcher, routers::ROUTER, typedef::{fs_json::{state::State, Config}, routing::Method, BackendError}, utils::response_json};

async fn launcher(_: Request<Incoming>, state: Arc<Mutex<State>>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    let secure = state.lock().unwrap();

    Ok(response_json(object! {
        launcher_url: secure.launcher_url.as_ref(),
        launcher_version: secure.launcher_version.as_ref(),
        minecraft_version: secure.minecraft_version.as_ref()
    }))
}

pub async fn register(watcher: &mut DirWatcher) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut router = ROUTER.lock().await;
    let state = State::open("state.json", watcher)?;

    router.endpoint(Method::Get,
        "/launcher",
        Box::new(move |req| {Box::pin(launcher(req, state.clone()))})
    ).expect("Failed to register status endpoint");

    Ok(())
}
