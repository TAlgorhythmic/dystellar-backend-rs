use std::convert::Infallible;
use std::sync::Arc;

use hyper::body::{Incoming, Bytes};
use hyper::{Request, Response};
use http_body_util::Full;
use json::object;
use tokio::sync::Mutex;
use crate::api::utils::response_status_json;

use super::routers::handle;
use super::typedef::Router;

pub async fn srv(req: Request<Incoming>, router: Arc<Mutex<Router>>) -> Result<Response<Full<Bytes>>, Infallible> {
    let res = handle(req, router).await;
    if let Err(err) = &res {
        let value = object! {
            ok: false,
            error: err.get_msg()
        };
        return Ok(response_status_json(value, *err.get_status()));
    }
    return Ok(res.unwrap());
}
