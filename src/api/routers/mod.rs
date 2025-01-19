pub mod auth;
pub mod microsoft;

use hyper::{Response, Request, body::Bytes, body::Incoming};
use http_body_util::Full;

pub fn handle(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
    match req.uri().path() {

        _ => Err("Endpoint not valid.".into())
    }
}
