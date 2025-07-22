pub mod microsoft;
pub mod signal;
pub mod privileged;
pub mod users;
pub mod state;

use std::sync::Arc;
use hyper::{Response, Request, body::{Bytes, Incoming}};
use http_body_util::Full;
use tokio::sync::Mutex;
use crate::api::typedef::BackendError;

use super::typedef::Router;

pub async fn handle(req: Request<Incoming>, router: Arc<Mutex<Router>>) -> Result<Response<Full<Bytes>>, BackendError> {
    if let Some(endpoint) = router.lock().await.get_endpoint(req.uri().path(), req.method().as_str().into()) {
        let fut = endpoint.get_handler()(req);
        return fut.await;
    }

    Err(BackendError::new("Path not found", 404))
}
