use std::{convert::Infallible, error::Error};

use http_body_util::combinators::BoxBody;
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;

use crate::api::{typedef::{BackendError, routing::{Method, nodes::{Node, Router}}}, utils::response_json};

/**
* A simple endpoint that returns an ok response, used to check the status of the backend if its
* running or not
*/
async fn status(_: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    Ok(response_json(object! { ok: true }))
}

pub async fn register(node: &mut Node) -> Result<(), Box<dyn Error + Send + Sync>> {
    node.subnode("/signal")?
        .endpoint("/status", Method::Get, status)?;

    Ok(())
}
