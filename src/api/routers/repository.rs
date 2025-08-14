use std::{error::Error, fs, sync::{LazyLock, Mutex}};

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};

use crate::api::{routers::ROUTER, typedef::{BackendError, Mod, routing::{Method, nodes::Router}}};

static MODS: LazyLock<Mutex<Vec<Mod>>> = LazyLock::new(|| Mutex::new(vec![]));

pub async fn register() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut router = ROUTER.lock().await;

    // Create dirs if missing
    let _ = fs::create_dir("repository");

    Ok(())
}
