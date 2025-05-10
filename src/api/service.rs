use std::error::Error;
use std::sync::Arc;

use hyper::body::{Incoming, Bytes};
use hyper::{Request, Response};
use http_body_util::Full;
use json::object;
use tokio::sync::Mutex;
use super::routers::handle;
use super::typedef::Router;

pub async fn srv(req: Request<Incoming>, router: Arc<Mutex<Router>>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    println!("{}", req.uri().path());
    let res = handle(req, router).await;
    if res.is_err() {
        let err = res.err().unwrap();
        println!("{}", err.to_string());
        let value = object! {
            ok: false,
            error: err.to_string()
        };
        return Ok(Response::new(Full::new(Bytes::from(json::stringify(value)))));
    } else {
        return res;
    }
}
