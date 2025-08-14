pub mod nodes;
pub mod endpoint;

use nodes::EndpointHandler;
use endpoint::Endpoint;

#[derive(PartialEq)]
pub enum Method {
    Get,
    Post,
    Delete,
    Patch,
    Put,
}

pub trait Node: Send + Sync {
    fn new(val: &str) -> Self where Self: Sized;
    fn remove_endpoint(&mut self, val: &str, method: &Method);
    fn subnodes_search_mut(&mut self, val: &str) -> Option<&mut Box<dyn Node>>;
    fn endpoints_search_mut(&mut self, val: &str, method: &Method) -> Option<&mut Endpoint>;
    fn subnodes_search(&self, val: &str) -> Option<&Box<dyn Node>>;
    fn endpoints_search(&self, val: &str, method: &Method) -> Option<&Endpoint>;
    fn get_name(&self) -> &str;
    fn is_modifiable(&self) -> bool;
}
