pub mod state;
pub mod redirects;

use std::{error::Error, fs, sync::{Arc, Mutex}};

use json::JsonValue;

use crate::api::control::inotify::register_file_watcher;

pub trait Config: Sized + Send + Sync + 'static {
    fn open(path: &str) -> Result<Arc<Mutex<Self>>, Box<dyn Error + Send + Sync>> {
        let mut conf = Self::default();
        let mut load_res = conf.load(path);

        if load_res.is_err() {
            println!("{path} doesn't seem to exist, creating default config...");
            if let Err(err) = conf.save(path) {
                eprintln!("Failed to save file: {}", err.to_string());
            } else {
                load_res = conf.load(path);
            }
        }

        let res = Arc::new(Mutex::new(conf));
        let path_cl: Box<str> = path.into();
        let res_cl = res.clone();

        println!("Registering watcher for {path}...");
        register_file_watcher(path, move || {
            println!("[{path_cl}] Updating cache...");
            let mut config = res_cl.lock().unwrap();
            let s = config.load(&path_cl);

            if s.is_err() {
                println!("Failed to update config from {path_cl}");
            }
        })?;

        Ok(res)
    }

    fn save(&self, path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        fs::write(path, json::stringify_pretty(self.to_json(), 4))?;
        Ok(())
    }

    fn default() -> Self;
    fn to_json(&self) -> JsonValue;
    fn load(&mut self, path: &str) -> Result<(), Box<dyn Error + Send + Sync>>;
}
