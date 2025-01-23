use std::ops::Deref;

use hyper::body::Bytes;
use hyper::Response;
use http_body_util::Full;

pub enum Method {
    Get,
    Post,
    Delete,
    Patch,
}

pub struct Endpoint {
    param: Option<Box<str>>,
    method: Method,
    name: Box<str>,
    run: fn() -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>>,
}

pub struct Node {
    subnodes: Vec<Node>,
    endpoints: Vec<Endpoint>,
}

pub struct Router {
    base: Box<str>,
    nodes: Vec<Node>,
}

impl Router {
    pub fn new(base: &str) -> Self {
        Self { base: base.into(), nodes: vec![] }
    }

    pub fn endpoint(&self, method: Method, path: &str) {
        let mut split = path.split('/');

        let mut actual = split.next();
        while actual.is_some() && actual.unwrap().eq(self.base.deref()) {
            actual = split.next();
        }
        
    }
}
