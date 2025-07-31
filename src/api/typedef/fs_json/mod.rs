pub mod state;
pub mod redirects;

use std::{error::Error, fs, sync::{Arc, Mutex}};

use json::JsonValue;

use crate::api::control::inotify::register_file_watcher;

pub trait Config: Sized + Send + Sync + 'static {
    fn open(path: &str) -> Result<Arc<Mutex<Self>>, Box<dyn Error + Send + Sync>> {
        let mut conf = Self::default();
        let conf_opt = conf.load(path);

        if conf_opt.is_err() {
            println!("{path} doesn't seem to exist, creating default config...");
            conf.save(path)?;
        }

        let res = Arc::new(Mutex::new(conf));
        let path_cl: Box<str> = path.into();
        let res_cl = res.clone();

        println!("Registering watcher for {path}...");
        register_file_watcher(path, move || {
            println!("File {path_cl} modified. Updating cache...");
            let s = res_cl.lock().unwrap().load(&path_cl);

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
