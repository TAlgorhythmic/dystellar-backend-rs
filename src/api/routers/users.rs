use std::{collections::HashMap, sync::{Arc, LazyLock}};

use chrono::{DateTime, Utc};
use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};
use tokio::sync::Mutex;

use crate::api::typedef::{BackendError, Method, Router};

pub static TOKENS: LazyLock<Arc<HashMap<&str, (&str, DateTime<Utc>)>>> = LazyLock::new(|| Arc::new(HashMap::new()));

/**
* Get user information, if a valid token is provided it returns full user information,
* otherwise only publicly available information is returned.
*/
async fn get(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, BackendError> {
    
}

pub async fn register(rout: &Arc<Mutex<Router>>) {
    let mut router = rout.lock().await;

    router.endpoint(Method::Get,
        "/api/users",
        Box::new(|req| {Box::pin(get(req))})
    ).expect("Failed to register status endpoint");
}
