use hyper::body::{Incoming, Bytes};
use hyper::{Request, Response};
use http_body_util::Full;
use json::object;
use super::routers::handle;

pub async fn srv(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
    let ct = req.headers().get("Content-Type");

    if ct.is_none() || ct.unwrap() != "application/json" {
        Err("This API is only accepting json requests.".into())
    } else {
        let res = handle(req);
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
}
