use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use http_body_util::combinators::BoxBody;
use hyper::body::{Incoming, Bytes};
use hyper::{Request, Response};
use json::object;
use tokio::sync::Mutex;
use crate::api::typedef::routing::nodes::Router;
use crate::api::utils::response_status_json;

use super::routers::handle;

static ERROR_COLOR: &str = "\x1b[31m";
static SUCCESS_COLOR: &str = "\x1b[32m";
static RESET_COLOR: &str = "\x1b[0m";

pub async fn srv_api(req: Request<Incoming>, address: SocketAddr, router: Arc<Mutex<Router>>) -> Result<Response<BoxBody<Bytes, Infallible>>, Infallible> {
    let path: Box<str> = req.uri().path().into();
    let method = req.method().clone();

    let res = handle(req, router).await;
    if let Err(err) = &res {
        let value = object! {
            ok: false,
            error: err.get_msg()
        };
        println!("{ERROR_COLOR}-> [{}] {} {{ error: {}, path: {}, address: {} }}{RESET_COLOR}", err.get_status(), method.as_str(), err.get_msg(), path, address);
        return Ok(response_status_json(value, *err.get_status()));
    }
    let res = res.unwrap();
    println!("{SUCCESS_COLOR}[{}] {} {{ path: {}, address: {} }}{RESET_COLOR}", res.status().as_str(), method.as_str(), path, address);

    return Ok(res);
}
