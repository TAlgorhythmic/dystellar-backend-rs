use std::collections::HashMap;
use std::error::Error;
use std::future::Future;

use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};
use http_body_util::Full;

type EndpointHandler = Box<dyn Fn(Request<Incoming>, HashMap<Box<str>, Box<str>>) -> Box<dyn Future<Output = Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>>> + Send + 'static>>;

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

    pub fn subnodes_search(&mut self, val: &str) -> Option<&mut Node> {
        self.subnodes.iter_mut().find(|n| *n.name == *val)
    }

    pub fn endpoints_search(&mut self, val: &str, method: &Method) -> Option<&mut Endpoint> {
        self.endpoints.iter_mut().find(|n| *n.name == *val && n.method == *method)
    }
}

fn register_endpoint(i: usize, node: &mut Node, split: Vec<&str>, method: Method, func: EndpointHandler)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    if i == split.len() - 1 {
        node.endpoints.push(Endpoint::new(method, split[i], func));
        return Ok(());
    } else {
        let next;

        if let Some(child) = node.subnodes_search(split[i]) {
            next = child;
        } else {
            let new = Node::new(split[i]);
            node.subnodes.push(new);
            next = node.subnodes.last_mut().unwrap();
        }


        register_endpoint(i + 1, next, split, method, func)
    }
}

impl Router {
    pub fn new(base: &str) -> Self {
        Self { base: Node::new(base) }
    }

    pub fn endpoint(&mut self, method: Method, path: &str, func: EndpointHandler) -> Result<(), Box<dyn Error + Send + Sync>> {
        let split = path.split('/').collect::<Vec<&str>>();

        if split.len() < 2 {
            return Err("An endpoint must contain at least the basename.".into());
        }

        if *split[1] != *self.base.name {
            return Err("Invalid base name in url.".into());
        }

        register_endpoint(1, &mut self.base, split, method, func)
    }
}
