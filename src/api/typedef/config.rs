use std::{error::Error, fs, sync::{Arc, Mutex}};

use json::{object, JsonValue};

use crate::api::control::inotify::register_file_watcher;

pub struct Config {
    pub launcher_url: Box<str>,
    pub launcher_version: Box<str>,
    pub discord_url: Box<str>
}

impl Config {
    pub fn default() -> Config {
        Config { launcher_url: "launcher_url".into(), launcher_version: "0.0".into(), discord_url: "discord_url".into() }
    }

    pub fn open(path: &str) -> Result<Arc<Mutex<Config>>, Box<dyn Error + Send + Sync>> {
        let mut conf = Config::default();
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

    pub fn to_json(&self) -> JsonValue {
        object! {
            launcher_url: self.launcher_url.as_ref(),
            launcher_version: self.launcher_version.as_ref(),
            discord_url: self.discord_url.as_ref()
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        fs::write(path, json::stringify_pretty(self.to_json(), 4))?;
        Ok(())
    }

    pub fn load(&mut self, path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let str = fs::read_to_string(path)?;

        let json = json::parse(&str)?;
        self.launcher_url = json["launcher_url"].as_str().unwrap_or("Failed to fetch launcher url").into();
        self.launcher_version = json["launcher_version"].as_str().unwrap_or("Failed to fetch launcher version").into();
        self.discord_url = json["discord_url"].as_str().unwrap_or("Failed to fetch discord url").into();

        Ok(())
    }
}
