use hyper::body::{Incoming, Bytes};
use hyper::{Request, Response, Error};
use http_body_util::Full;
use super::routers::

pub async fn srv(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Box<str>> {
    let ct = req.headers().get("Content-Type");

    match ct {
        Some(val) => ,
        None => Err("asd".into())
    }
}
