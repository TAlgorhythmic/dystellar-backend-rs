pub mod state;
pub mod redirects;

use std::{error::Error, fs, sync::{Arc, Mutex}};

use json::JsonValue;

use crate::api::control::inotify::DirWatcher;

pub trait Config: Sized + Send + Sync + 'static {
    fn open(path: &str, watcher: &mut DirWatcher) -> Result<Arc<Mutex<Self>>, Box<dyn Error + Send + Sync>> {
        let mut conf = Self::default();

        if conf.load_async(path).is_err() {
            println!("{path} doesn't seem to exist, creating default config...");
            if let Err(err) = conf.save(path) {
                eprintln!("Failed to save file: {}", err.to_string());
            }
        }

        let res = Arc::new(Mutex::new(conf));
        let res_cl = res.clone();

        println!("Registering watcher for {path}...");
        watcher.watch(path, Box::new(move |path| {
            println!("[{path}] Updating cache...");
            let mut config = res_cl.lock().unwrap();
            let s = config.load(path);

            if s.is_err() {
                println!("Failed to update config from {path}");
            }
        }), None);

        Ok(res)
    }

    fn save(&self, path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        fs::write(path, json::stringify_pretty(self.to_json(), 4))?;
        Ok(())
    }

    fn default() -> Self;
    fn to_json(&self) -> JsonValue;
    fn load(&mut self, path: &str) -> Result<(), Box<dyn Error + Send + Sync>>;

    fn load_async(&mut self, path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        tokio::task::block_in_place(|| {
            self.load(path)
        })
    }
}
