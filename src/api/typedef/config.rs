use std::{error::Error, fs, sync::{Arc, Mutex}};

use json::{object, JsonValue};

use crate::api::control::inotify::register_file_watcher;

pub struct Config {
    launcher_url: Box<str>,
    launcher_version: Box<str>,
}

impl Config {
    pub fn default() -> Config {
        Config { launcher_url: "launcher_url".into(), launcher_version: "0.0".into() }
    }

    pub fn open(path: &str) -> Result<Arc<Mutex<Config>>, Box<dyn Error + Send + Sync>> {
        let conf_opt = fs::read_to_string(path);
        if conf_opt.is_err() {
            println!("{path} doesn't seem to exist, creating default config...");
            let res = Arc::new(Mutex::new(Config::default().save(path)?));

            println!("Registering watcher for {path}...");
            let path_cl: Box<str> = path.into();
            let res_cl = res.clone();

            register_file_watcher(path, move || {
                let s = res_cl.lock().unwrap().load(&path_cl);

                if s.is_err() {
                    println!("Failed to update config from {path_cl}");
                }
            });
            return Ok(res);
        }

        let json = json::parse(&conf_opt.unwrap())?;
        Ok(Arc::new(Mutex::new(Config {
            launcher_url: json["launcher_url"].as_str().unwrap_or("Failed to fetch launcher url").into(),
            launcher_version: json["launcher_version"].as_str().unwrap_or("Failed to fetch launcher version").into()
        })))
    }

    pub fn to_json(&self) -> JsonValue {
        object! {
            launcher_url: self.launcher_url.as_ref(),
            launcher_version: self.launcher_version.as_ref()
        }
    }

    pub fn save(self, path: &str) -> Result<Config, Box<dyn Error + Send + Sync>> {
        fs::write(path, json::stringify(self.to_json()))?;
        Ok(self)
    }

    pub fn load(&mut self, path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let str = fs::read_to_string(path)?;

        let json = json::parse(&str)?;
        self.launcher_url = json["launcher_url"].as_str().unwrap_or("Failed to fetch launcher url").into();
        self.launcher_version = json["launcher_version"].as_str().unwrap_or("Failed to fetch launcher version").into();

        Ok(())
    }
}
