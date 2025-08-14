use super::{Method, EndpointHandler};

pub struct Endpoint {
    pub method: Method,
    pub name: Box<str>,
    pub run: EndpointHandler,
}

impl Endpoint {
    pub fn new(method: Method, name: &str, fun: EndpointHandler) -> Self {
        Self { name: name.into(), method, run: fun }
    }

    pub fn get_handler(&self) -> &EndpointHandler {
        &self.run
    }
}
