use std::error::Error;

use hyper::body::{Incoming, Bytes};
use hyper::{Request, Response};
use http_body_util::Full;
use json::object;
use super::routers::handle;

pub async fn srv(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    let res = handle(req).await;
    if res.is_err() {
        let value = object! {
            ok: false,
            error: res.err().unwrap().to_string()
        };
        return Ok(Response::new(Full::new(Bytes::from(json::stringify(value)))));
    } else {
        return res;
    }
}
