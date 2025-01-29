use std::ops::Deref;

use hyper::body::Bytes;
use hyper::Response;
use http_body_util::Full;

type EndpointHandler = fn(Option<Box<str>>) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>>;

#[derive(PartialEq)]
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
    method: Method,
    name: Box<str>,
    run: EndpointHandler,
}

pub struct Node {
    name: Box<str>,
    subnodes: Vec<Node>,
    endpoints: Vec<Endpoint>,
}

pub struct Router {
    base: Node,
}

impl Endpoint {
    pub fn new(method: Method, name: &str, fun: EndpointHandler) -> Self {
        Self { name: name.into(), method, run: fun }
    }
}

impl Node {
    pub fn new(val: &str) -> Self {
        Self { name: val.into(), subnodes: vec![], endpoints: vec![] }
    }

    pub fn subnodes_search(&self, val: &str) -> Option<&Node> {
        for subnode in &self.subnodes {
            if *subnode.name == *val {
                return Some(subnode);
            }
        }
        None
    }

    pub fn endpoints_search(&self, val: &str, method: &Method) -> Option<&Endpoint> {
        for endpoint in &self.endpoints {
            if *endpoint.name == *val && endpoint.method == *method {
                return Some(endpoint);
            }
        }
        None
    }
}

impl Router {
    pub fn new(base: &str) -> Self {
        Self { base: Node::new(base) }
    }

    pub fn endpoint(&mut self, method: Method, path: &str, func: EndpointHandler) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let split = path.split('/').collect::<Vec<&str>>();
        let len = split.len();

        if len < 2 {
            return Err("An endpoint must contain at least the basename.".into());
        }

        if *split[1] != *self.base.name {
            return Err("Invalid base name in url.".into());
        }

        let mut act = &mut self.base;

        for i in 2..len {
            let nd_found = act.subnodes_search(split[i]);

            if nd_found.is_some() {
                act = nd_found.unwrap();
                continue;
            }
            let ed_found = act.endpoints_search(split[i], &method);
            if ed_found.is_some() {
                eprintln!("Warning! Endpoint {path} is duplicated in some way, this endpoint won't be registered.");
                break;
            }
            if i == (len - 1) {
                act.endpoints.push(Endpoint::new(method, split[i], func));
            }
        }
        Ok(())
    }
}
