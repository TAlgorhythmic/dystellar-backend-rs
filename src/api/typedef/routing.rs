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

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        if value == "POST" {return Self::Post;}
        else if value == "DELETE" {return Self::Delete;}
        else if value == "PATCH" {return Self::Patch;}
        else {Self::Get}
    }
}

pub struct Endpoint {
    param: Option<Box<str>>,
    method: Method,
    name: Box<str>,
    run: fn(Option<Box<str>>) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>>,
}

pub struct Node {
    name: Box<str>,
    subnodes: Vec<Node>,
    endpoints: Vec<Endpoint>,
}

pub struct Router {
    base: Node,
}

impl Router {
    pub fn new(base: &str) -> Self {
        Self { base: base.into()} }
    }

    pub fn endpoint<T>(&self, method: Method, path: &str, func: T) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        T: Fn(Option<Box<str>>) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>>
    {
        let split = path.split('/').collect::<Vec<&str>>();
        let len = split.len();

        if len < 2 {
            Err("An endpoint must contain at least the basename.")
        }

        for i in 2..len {
            if i == (len - 1) {

            }
        }
        Ok(())
    }
}
