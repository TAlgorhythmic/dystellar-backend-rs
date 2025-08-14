use std::error::Error;
use std::future::Future;
use std::pin::Pin;

use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};
use http_body_util::Full;

use super::{Method, Node, Endpoint};
use crate::api::typedef::BackendError;

pub type EndpointHandler = Box<dyn Fn(Request<Incoming>) -> Pin<Box<dyn Future<Output = Result<Response<Full<Bytes>>, BackendError>> + Send + 'static>> + Send + Sync + 'static>;

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        if value == "POST" {Self::Post}
        else if value == "DELETE" {Self::Delete}
        else if value == "PATCH" {Self::Patch}
        else if value == "PUT" {Self::Put}
        else {Self::Get}
    }
}

pub struct RouterNode {
    name: Box<str>,
    subnodes: Vec<Box<dyn Node>>,
    endpoints: Vec<Endpoint>,
}

pub struct Router {
    base: RouterNode,
}

impl Node for RouterNode {
    fn new(val: &str) -> Self {
        Self { name: val.into(), subnodes: vec![], endpoints: vec![] }
    }

    fn remove_endpoint(&mut self, val: &str, method: &Method) {
        self.endpoints.retain(|endpoint| &*endpoint.name != val || endpoint.method != *method);
    }

    fn subnodes_search_mut(&mut self, val: &str) -> Option<&mut Box<dyn Node>> {
        self.subnodes.iter_mut().find(|n| n.get_name() == val)
    }

    fn endpoints_search_mut(&mut self, val: &str, method: &Method) -> Option<&mut Endpoint> {
        self.endpoints.iter_mut().find(|n| *n.name == *val && n.method == *method)
    }

    fn subnodes_search(&self, val: &str) -> Option<&Box<dyn Node>> {
        self.subnodes.iter().find(|n| n.get_name() == val)
    }

    fn endpoints_search(&self, val: &str, method: &Method) -> Option<&Endpoint> {
        self.endpoints.iter().find(|n| *n.name == *val && n.method == *method)
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn is_modifiable(&self) -> bool {
        true
    }
}

impl RouterNode {
    pub fn empty() -> Self {
        Self::new("")
    }
}

fn register_endpoint(i: usize, node: &mut RouterNode, split: Vec<&str>, method: Method, func: EndpointHandler)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    if i == split.len() - 1 {
        node.endpoints.push(Endpoint::new(method, split[i], func));
        return Ok(());
    } else {
        let next: Box<dyn Node>;

        if let Some(child) = node.subnodes_search_mut(split[i]) {
            next = child;
        } else {
            let new = RouterNode::new(split[i]);
            node.subnodes.push(new);
            next = node.subnodes.last_mut().unwrap();
        }

        register_endpoint(i + 1, next, split, method, func)
    }
}

impl Router {
    pub fn new() -> Self {
        Self { base: RouterNode::empty() }
    }

    pub fn get_endpoint(&self, path: &str, method: Method) -> Option<&Endpoint> {
        let split = path.split('/').collect::<Vec<&str>>();

        if split.len() < 1 {
            return None;
        }

        let mut node = &self.base;
        for i in 1..split.len() {
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

    pub fn remove_endpoint(&mut self, method: Method, path: &str) {
        let split = path.split('/').collect::<Vec<&str>>();

        if split.len() < 1 {
            return;
        }

        let mut node = &mut self.base;
        for i in 1..split.len() {
            if i == split.len() - 1 {
                node.remove_endpoint(split[i], &method);
                return;
            }
            if let Some(subnode) = node.subnodes_search_mut(split[i]) {
                node = subnode;
                continue;
            }
            return;
        }
    }

    pub fn endpoint(&mut self, method: Method, path: &str, func: EndpointHandler) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !path.starts_with('/') {
            return Err("Invalid path name".into());
        }

        let split = path.split('/').collect::<Vec<&str>>();

        if split.len() < 1 {
            return Err("Not an endpoint".into());
        }

        register_endpoint(1, &mut self.base, split, method, func)
    }
}
