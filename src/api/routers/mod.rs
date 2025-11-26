pub mod microsoft;
pub mod signal;
pub mod privileged;
pub mod users;
pub mod state;
pub mod stream;
pub mod redirections;

use std::{convert::Infallible, sync::{Arc, LazyLock}};
use hyper::{body::{Bytes, Incoming}, Request, Response};
use http_body_util::combinators::BoxBody;
use tokio::sync::Mutex;
use crate::api::typedef::{routing::nodes::Router, BackendError};

pub static ROUTER: LazyLock<Arc<Mutex<Router>>> = LazyLock::new(|| Arc::new(Mutex::new(Router::new())));

pub async fn handle(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    let router = ROUTER.lock().await;

    if let Some(endpoint) = router.get_endpoint(req.uri().path(), req.method().as_str().into()) {
        let fut = endpoint.get_handler()(req);
        return fut.await;
    } else if req.method().as_str() == "GET" {
        if let Some(map) = router.get_mapper(req.uri().path()) {
            let (mapper, file_path) = map;

            let fut = mapper.get_handler()(req, file_path);
            return fut.await;
        }
    }

    Err(BackendError::new("Path not found", 404))
}
