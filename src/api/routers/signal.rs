use std::convert::Infallible;

use http_body_util::combinators::BoxBody;
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;

use crate::api::{routers::ROUTER, typedef::{routing::Method, BackendError}, utils::response_json};

/**
* A simple endpoint that returns an ok response, used to check the status of the backend if its
* running or not
*/
async fn status(_: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    Ok(response_json(object! { ok: true }))
}

pub async fn register() {
    let mut router = ROUTER.lock().await;

    router.endpoint(Method::Get,
        "/api/signal/status",
        Box::new(|req| {Box::pin(status(req))})
    ).expect("Failed to register status endpoint");
}
