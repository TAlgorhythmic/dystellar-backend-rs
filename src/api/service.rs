use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::body::{Incoming, Bytes};
use hyper::{Request, Response};
use http_body_util::Full;
use json::object;
use crate::api::utils::response_status_json;

use super::routers::handle;

static ERROR_COLOR: &str = "\x1b[31m";
static SUCCESS_COLOR: &str = "\x1b[32m";
static RESET_COLOR: &str = "\x1b[0m";

pub async fn srv(req: Request<Incoming>, address: SocketAddr) -> Result<Response<Full<Bytes>>, Infallible> {
    let path: Box<str> = req.uri().path().into();
    
    let res = handle(req).await;
    if let Err(err) = &res {
        let value = object! {
            ok: false,
            error: err.get_msg()
        };
        println!("{ERROR_COLOR}-> Bad Return Error: {}, code: {}, path: {}, address: {}{RESET_COLOR}", err.get_msg(), err.get_status(), path, address);
        return Ok(response_status_json(value, *err.get_status()));
    }
    println!("{SUCCESS_COLOR} Successful Connection: path: {}, address: {}{RESET_COLOR}", path, address);

    return Ok(res.unwrap());
}
