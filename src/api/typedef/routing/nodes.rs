use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};
use http_body_util::Full;

use super::{Method, Endpoint};
use crate::api::typedef::BackendError;

pub type EndpointHandler = Box<dyn Fn(Request<Incoming>) -> Pin<Box<dyn Future<Output = Result<Response<Full<Bytes>>, BackendError>> + Send + 'static>> + Send + Sync + 'static>;
pub type FsEndpointHandler = Box<dyn Fn(Request<Incoming>, &str) -> Pin<Box<dyn Future<Output = Result<Response<Full<Bytes>>, BackendError>> + Send + 'static>> + Send + Sync + 'static>;

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        if value == "POST" {Self::Post}
        else if value == "DELETE" {Self::Delete}
        else if value == "PATCH" {Self::Patch}
        else if value == "PUT" {Self::Put}
        else {Self::Get}
    }
}

pub struct FsNodeMapper {
    name: Box<str>,
    endpoint: FsEndpointHandler
}

pub struct Node {
    name: Box<str>,
    subnodes: Vec<Node>,
    endpoints: Vec<Endpoint>,
}

pub struct Router {
    base: Node,
    fs_mappers: Vec<FsNodeMapper>
}

impl FsNodeMapper {
    pub fn new(path: &str, func: FsEndpointHandler) -> Self {
        Self { name: path.into(), endpoint: func }
    }

    pub fn get_name(&self) -> &str {
        todo!()
    }
}

impl Node {
    fn new(val: &str) -> Self {
        Self { name: val.into(), subnodes: vec![], endpoints: vec![] }
    }

    pub fn empty() -> Self {
        Self::new("")
    }

}

impl Node {
    pub fn remove_endpoint(&mut self, val: &str, method: &Method) {
        self.endpoints.retain(|endpoint| &*endpoint.name != val || endpoint.method != *method);
    }

    pub fn subnodes_search_mut(&mut self, val: &str) -> Option<&mut Node> {
        self.subnodes.iter_mut().find(|n| n.get_name() == val)
    }

    pub fn subnodes_search(&self, val: &str) -> Option<&Node> {
        self.subnodes.iter().find(|n| n.get_name() == val)
    }

    pub fn endpoints_search(&self, val: &str, method: &Method) -> Option<&Endpoint> {
        self.endpoints.iter().find(|n| *n.name == *val && n.method == *method)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_endpoint_recursive(&self, segments: &[&str], method: &Method) -> Option<&Endpoint> {
        if segments.len() == 1 {
            return self.endpoints_search(segments[0], method);
        }

        if let Some(child) = self.subnodes_search(segments[0]) {
            child.get_endpoint_recursive(&segments[1..], method)
        } else {
            None
        }
    }

    fn get_endpoint(&self, path: &str, method: &Method) -> Option<&Endpoint> {
        let it: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        if it.is_empty() {
            return None;
        }
        self.get_endpoint_recursive(&it, method)
    }
}

fn register_endpoint(i: usize, node: &mut Node, split: Vec<&str>, method: Method, func: EndpointHandler)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    if i == split.len() - 1 {
        node.endpoints.push(Endpoint::new(method, split[i], func));
        return Ok(());
    } else {
        let next: &mut Node;

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
    pub fn new() -> Self {
        Self { base: Node::empty(), fs_mappers: vec![] }
    }

    pub fn get_endpoint(&self, path: &str, method: Method) -> Option<&Endpoint> {
        self.base.get_endpoint(path, &method)
    }

    pub fn get_mapper(&self, path: &str) -> Option<(&FsNodeMapper, Box<str>)> {
        if let Some(map) = self.fs_mappers.iter().find(|m| path.starts_with(m.name))
        None
    }

    pub fn remove_endpoint(&mut self, method: Method, path: &str) {
        let split = path.split('/').collect::<Vec<&str>>();

        if split.len() < 1 {
            return;
        }

        if split.len() == 2 {
            self.base.remove_endpoint(split[1], &method);
            return;
        }

        let node_opt = self.base.subnodes_search_mut(split[1]);
        if node_opt.is_none() {
            return;
        }

        let mut node = node_opt.unwrap();
        for i in 2..split.len() {
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

    pub fn map(&self, )

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
