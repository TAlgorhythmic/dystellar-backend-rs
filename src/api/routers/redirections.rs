use std::error::Error;

use crate::api::{control::inotify::DirWatcher, typedef::fs_json::{redirects::Redirects, Config}};

pub fn register(watcher: &mut DirWatcher) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = Redirects::open("redirections.json", watcher)?;

    Ok(())
}
