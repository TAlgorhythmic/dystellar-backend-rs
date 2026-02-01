use std::{error::Error, sync::Arc};

use tokio::sync::Mutex;

use crate::api::{control::inotify::DirWatcher, typedef::{fs_json::redirects::Redirects, routing::nodes::Router}};

pub fn register(watcher: &mut DirWatcher, router: Arc<Mutex<Router>>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = Redirects::open_redirs("redirections.json", watcher, router)?;

    Ok(())
}
