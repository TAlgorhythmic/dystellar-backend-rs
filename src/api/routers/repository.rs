use std::{error::Error, fs, sync::{LazyLock, Mutex}};

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, Request, Response};

use crate::api::{control::inotify::register_file_watcher, routers::ROUTER, typedef::{BackendError, Method, Mod, Router}};

static MODS: LazyLock<Mutex<Vec<Mod>>> = LazyLock::new(|| Mutex::new(vec![]));

async fn mods(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, BackendError> {
    
}

pub async fn register() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut router = ROUTER.lock().await;

    // Create dirs if missing
    fs::create_dir_all("repository/mods")?;

    update_repos_r(&mut router, "repository");

    register_file_watcher("repository", || {
        let mut rout = ROUTER.blocking_lock();

        update_repos_r(&mut rout, "repository");
    });

    router.endpoint(Method::Get,
        "/api/signal/status",
        Box::new(|req| {Box::pin(mods(req))})
    ).expect("Failed to register status endpoint");

    Ok(())
}

fn update_repos_r(router: &mut Router, path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {

    for f in fs::read_dir(path)? {
        let entry = f?;

        if entry.file_type()?.is_dir() {
            update_repos_r(router, format!("{}/{}", path, entry.file_name().to_str().unwrap()).as_str())?;
        } else {

        }
    }

    Ok(())
}
