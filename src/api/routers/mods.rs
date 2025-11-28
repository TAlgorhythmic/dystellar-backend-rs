use rayon::prelude::*;
use std::{convert::Infallible, error::Error, fs::{self, DirEntry}, sync::Arc, thread};

use http_body_util::{BodyExt, Full, combinators::BoxBody};
use hyper::{Request, Response, body::{Bytes, Incoming}, header::CONTENT_TYPE};
use json::{object, stringify};
use tokio::{sync::Mutex, task::spawn_blocking};

use crate::api::{control::inotify::DirWatcher, routers::ROUTER, typedef::{BackendError, ModMetadata, routing::Method}};

fn generate_mod_registry() -> Result<Box<str>, Box<dyn Error + Send + Sync>> {
    println!("Generating mod registry...");

    let files: Vec<DirEntry> = fs::read_dir("repository/mods")?
        .filter_map(|x| x.ok())
        .filter(|x| {
            if let Ok(metadata) = x.metadata() {
                return !metadata.is_dir();
            }
            false
        })
        .collect();
    let files_optional: Vec<DirEntry> = fs::read_dir("repository/mods/optional")?
        .filter_map(|x| x.ok())
        .filter(|x| {
            if let Ok(metadata) = x.metadata() {
                return !metadata.is_dir();
            }
            false
        })
        .collect();

    let mods: Vec<ModMetadata> = files.par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            ModMetadata::from_path(path.to_str()?).ok()
        })
        .collect();
    let mods_optional: Vec<ModMetadata> = files_optional.par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            ModMetadata::from_path(path.to_str()?).ok()
        })
        .collect();

    let json = object! {
        mods: mods,
        optional: mods_optional
    };

    println!("Mod registry created!");
    Ok(stringify(json).into_boxed_str())
}

async fn generate_mod_registry_async() -> Result<Box<str>, Box<dyn Error + Send + Sync>> {
    spawn_blocking(|| generate_mod_registry()).await?
}

async fn manifest(_: Request<Incoming>, registry: Arc<Mutex<Box<str>>>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    let lock = registry.lock().await;

    Ok(Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from((*lock).to_string())).boxed())
        .unwrap()
    )
}

pub async fn register() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut router = ROUTER.lock().await;
    
    fs::create_dir_all("repository/mods/optional")?;

    let registry = Arc::new(Mutex::new(generate_mod_registry_async().await?));

    let mut mods_dir = DirWatcher::create("repository/mods")?;
    let mut optional_mods_dir = DirWatcher::create("repository/mods/optional")?;

    mods_dir.watch_global({
        let registry = registry.clone();

        Box::new(move |_| {
            let mut reg = registry.blocking_lock();
            match generate_mod_registry() {
                Ok(r) => *reg = r,
                Err(err) => println!("Failed to generate mod registry: {}", err.to_string())
            }
        })
    });
    optional_mods_dir.watch_global({
        let registry = registry.clone();

        Box::new(move |_| {
            let mut reg = registry.blocking_lock();
            match generate_mod_registry() {
                Ok(r) => *reg = r,
                Err(err) => println!("Failed to generate mod registry: {}", err.to_string())
            }
        })
    });

    mods_dir.listen();
    optional_mods_dir.listen();

    router.endpoint(Method::Get,
        "/api/mods/manifest",
        Box::new(move |req| {Box::pin(manifest(req, registry.clone()))})
    ).expect("Failed to register status endpoint");

    Ok(())
}
