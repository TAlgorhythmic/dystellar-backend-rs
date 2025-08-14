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
