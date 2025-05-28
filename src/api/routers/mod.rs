pub mod microsoft;
pub mod signal;

use std::{error::Error, sync::Arc};
use hyper::{Response, Request, body::{Bytes, Incoming}};
use http_body_util::Full;
use tokio::sync::Mutex;

use super::typedef::Router;

pub async fn handle(req: Request<Incoming>, router: Arc<Mutex<Router>>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    if let Some(endpoint) = router.lock().await.get_endpoint(req.uri().path(), req.method().as_str().into()) {
        let fut = endpoint.get_handler()(req);
        return fut.await;
    }

    Err("Path not found".into())
}
