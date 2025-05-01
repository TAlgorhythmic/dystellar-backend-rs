use std::{collections::HashMap, error::Error};

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};

use crate::api::typedef::Method;

use super::router;

async fn on_reg(req: Request<Incoming>, args: HashMap<&str, &str>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    if req.headers().get("Content-Type").unwrap() != "application/json" {
        return Err("This kind of request requires json content.".into());
    }

    
}

pub fn register() {
    {
        let this = &mut router();
        let method = Method::Post;
        let split = "/api/auth/register".split('/').collect::<Vec<&str>>();

        if split.len() < 2 {
            return Err("An endpoint must contain at least the basename.".into());
        }

        if *split[1] != *this.base.name {
            return Err("Invalid base name in url.".into());
        }

        register_endpoint(1, &mut this.base, split, method, on_reg)
    };
}
