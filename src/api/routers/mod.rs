pub mod microsoft;

use std::{error::Error, sync::{Arc, LazyLock, Mutex}};

use hyper::{Response, Request, body::{Bytes, Incoming}};
use http_body_util::Full;

use super::typedef::Router;

const ROUTER: LazyLock<Arc<Mutex<Router>>> = LazyLock::new(|| Arc::new(Mutex::new(Router::new("/api"))));

pub fn router() -> Arc<Mutex<Router>> {
    ROUTER.clone()
}

pub async fn handle(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    match req.uri().path() {
        _ => Err("Endpoint not valid.".into())
    }
}
