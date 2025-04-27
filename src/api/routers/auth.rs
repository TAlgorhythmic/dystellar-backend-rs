use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};

use crate::api::typedef::Method;

use super::router;

fn on_reg(req: Request<Incoming>, args: Option<&str>) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
    
}

pub fn register() {
    router().endpoint(Method::Post, "/api/auth/register", on_reg);
}
