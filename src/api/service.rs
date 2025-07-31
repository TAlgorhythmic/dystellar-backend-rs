use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::body::{Incoming, Bytes};
use hyper::{Request, Response};
use http_body_util::Full;
use json::object;
use tokio::sync::Mutex;
use crate::api::utils::response_status_json;

use super::routers::handle;
use super::typedef::Router;

pub async fn srv(req: Request<Incoming>, address: SocketAddr) -> Result<Response<Full<Bytes>>, Infallible> {
    let path: Box<str> = req.uri().path().into();
    
    let res = handle(req).await;
    if let Err(err) = &res {
        let value = object! {
            ok: false,
            error: err.get_msg()
        };
        println!("-> Bad Return Error: {}, code: {}, path: {}, address: {}", err.get_msg(), err.get_status(), path, address);
        return Ok(response_status_json(value, *err.get_status()));
    }
    return Ok(res.unwrap());
}
