use std::error::Error;
use std::future::Future;
use std::pin::Pin;

use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};
use http_body_util::Full;

type EndpointHandler = Box<dyn Fn(Request<Incoming>) -> Pin<Box<dyn Future<Output = Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>>> + Send + 'static>>>;

#[derive(PartialEq)]
pub enum Method {
    Get,
    Post,
    Delete,
    Patch,
    Put,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        if value == "POST" {return Self::Post;}
        else if value == "DELETE" {return Self::Delete;}
        else if value == "PATCH" {return Self::Patch;}
        else if value == "PUT" {return Self::Put}
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

unsafe impl Send for Endpoint {}
unsafe impl Send for Node {}
unsafe impl Send for Router {}

impl Endpoint {
    pub fn new(method: Method, name: &str, fun: EndpointHandler) -> Self {
        Self { name: name.into(), method, run: fun }
    }

    pub fn get_handler(&self) -> &EndpointHandler {
        &self.run
    }
}

impl Node {
    pub fn new(val: &str) -> Self {
        Self { name: val.into(), subnodes: vec![], endpoints: vec![] }
    }

    pub fn subnodes_search_mut(&mut self, val: &str) -> Option<&mut Node> {
        self.subnodes.iter_mut().find(|n| *n.name == *val)
    }

    pub fn endpoints_search_mut(&mut self, val: &str, method: &Method) -> Option<&mut Endpoint> {
        self.endpoints.iter_mut().find(|n| *n.name == *val && n.method == *method)
    }

    pub fn subnodes_search(&self, val: &str) -> Option<&Node> {
        self.subnodes.iter().find(|n| *n.name == *val)
    }

    pub fn endpoints_search(&self, val: &str, method: &Method) -> Option<&Endpoint> {
        self.endpoints.iter().find(|n| *n.name == *val && n.method == *method)
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

        if let Some(child) = node.subnodes_search_mut(split[i]) {
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

    pub fn get_endpoint(&self, path: &str, method: Method) -> Option<&Endpoint> {
        let split = path.split('/').collect::<Vec<&str>>();

        if split.len() < 2 || *split[1] != *self.base.name {
            return None;
        }

        let mut node = &self.base;
        for i in 2..split.len() {
            if i == split.len() - 1 {
                if let Some(endpoint) = node.endpoints_search(split[i], &method) {
                    return Some(endpoint);
                }
            }
            if let Some(subnode) = node.subnodes_search(split[i]) {
                node = subnode;
                continue;
            }
            return None;
        }
        None
    }

    pub fn endpoint(&mut self, method: Method, path: &str, func: EndpointHandler) -> Result<(), Box<dyn Error + Send + Sync>> {
        let split = path.split('/').collect::<Vec<&str>>();

        if split.len() < 2 {
            return Err("An endpoint must contain at least the basename.".into());
        }

        if *split[1] != *self.base.name {
            return Err("Invalid base name in url.".into());
        }
        register_endpoint(2, &mut self.base, split, method, func)
    }
}
