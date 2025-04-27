pub mod auth;
pub mod microsoft;

use std::sync::{Arc, LazyLock};

use hyper::{Response, Request, body::{Bytes, Incoming}};
use http_body_util::Full;

use super::typedef::Router;

const ROUTER: LazyLock<Arc<Router>> = LazyLock::new(|| Arc::new(Router::new("/api")));

pub fn router() -> Arc<Router> {
    ROUTER.clone()
}

pub fn handle(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
    match req.uri().path() {
        _ => Err("Endpoint not valid.".into())
    }
}
