use std::{error::Error, fs, sync::{Arc, Mutex}};

use json::JsonValue;

use crate::api::{control::inotify::DirWatcher, typedef::routing::Method, utils::temporary_redirection};

use super::Config;

pub struct Redirects {
    pub mappings: Arc<Mutex<Vec<(Box<str>, Arc<str>)>>>
}

impl Redirects {
    fn open_redirs(path: &str, watcher: &mut DirWatcher) -> Result<Arc<Mutex<Self>>, Box<dyn Error + Send + Sync>> {
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
}

impl Config for Redirects {
    fn default() -> Self {
        Self { mappings: Arc::new(Mutex::new(vec![])) }
    }

    fn to_json(&self) -> json::JsonValue {
        let mut json = JsonValue::new_object();
        let mappings = self.mappings.lock().unwrap();

        for (key, value) in &*mappings {
            json[key.as_ref()] = JsonValue::String(value.to_string());
        }
        json
    }

    fn load(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let path_cl: Box<str> = path.into();
        let mut router = ROUTER.blocking_lock();
        let mut mappings = self.mappings.lock().unwrap();

        for (key, _) in &*mappings {
            router.remove_endpoint(Method::Get, format!("/{key}").as_str());
        }
        mappings.clear();

        let str = fs::read_to_string(path_cl.to_string())?;

        let json_opt = json::parse(&str);
        if let Err(err) = &json_opt {
            return Err(format!("Error parsing json: {}", err.to_string()).into());
        }

        let json = json_opt.unwrap();

        for (key, value) in json.entries() {
            if let Some(val) = value.as_str() {
                mappings.push((key.into(), val.into()));
            }
        }

        for (key, value) in &*mappings {
            let val = value.clone();

            let _ = router.endpoint(
                Method::Get,
                format!("/{key}").as_str(),
                move |_| {
                    let url = val.clone();
                    async move {
                        Ok(temporary_redirection(&url))
                    }
                }
            );
        }

        Ok(())
    }
}
