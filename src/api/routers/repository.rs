use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};

use crate::api::{routers::ROUTER, typedef::{BackendError, Method}};

async fn mods(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, BackendError> {
    
}

pub async fn register() {
    let mut router = ROUTER.lock().await;

    router.endpoint(Method::Get,
        "/api/signal/status",
        Box::new(|req| {Box::pin(mods(req))})
    ).expect("Failed to register status endpoint");
}
