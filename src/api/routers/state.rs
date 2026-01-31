use std::{convert::Infallible, error::Error, sync::{Arc, Mutex}};

use http_body_util::{combinators::BoxBody};
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;

use crate::api::{control::inotify::DirWatcher, typedef::{BackendError, fs_json::{Config, state::State}, routing::{Method, nodes::Router}}, utils::response_json};

async fn launcher(_: Request<Incoming>, state: Arc<Mutex<State>>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    let secure = state.lock().unwrap();

    Ok(response_json(object! {
        launcher_url: secure.launcher_url.as_ref(),
        launcher_version: secure.launcher_version.as_ref(),
        minecraft_version: secure.minecraft_version.as_ref()
    }))
}

pub async fn register(router: &mut Router, watcher: &mut DirWatcher) -> Result<(), Box<dyn Error + Send + Sync>> {
    let state = State::open("state.json", watcher)?;

    router.endpoint(Method::Get, "/launcher", move |req| launcher(req, state.clone()))?;

    Ok(())
}
