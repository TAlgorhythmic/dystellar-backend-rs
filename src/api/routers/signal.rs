use std::{error::Error, sync::Arc};

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;
use tokio::sync::Mutex;

use crate::api::{typedef::{Method, Router}, utils::response};


/**
* A simple endpoint that returns an ok response, used to check the status of the backend if its
* running or not
*/
async fn status(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    Ok(response(object! { ok: true }))
}

pub async fn register(rout: &Arc<Mutex<Router>>) {
    let mut router = rout.lock().await;

    router.endpoint(Method::Get,
        "/api/signal/status",
        Box::new(|req| {Box::pin(status(req))})
    ).expect("Failed to register status endpoint");
}
