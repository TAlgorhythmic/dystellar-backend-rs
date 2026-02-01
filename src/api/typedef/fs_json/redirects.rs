use std::{error::Error, fs, sync::Arc};

use json::JsonValue;
use tokio::sync::Mutex;

use crate::api::{control::inotify::DirWatcher, typedef::routing::{Method, nodes::Router}, utils::temporary_redirection};

use super::Config;

pub struct Redirects {
    pub mappings: Arc<Mutex<Vec<(Box<str>, Arc<str>)>>>,
    pub router: Arc<Mutex<Router>>
}

impl Redirects {
    pub fn open_redirs(path: &str, watcher: &mut DirWatcher, router: Arc<Mutex<Router>>) -> Result<Arc<Mutex<Self>>, Box<dyn Error + Send + Sync>> {
        let mut conf = Self::new(router);

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
            let mut config = res_cl.blocking_lock();
            let s = config.load(path);

            if s.is_err() {
                println!("Failed to update config from {path}");
            }
        }), None);

        Ok(res)
    }

    pub fn new(router: Arc<Mutex<Router>>) -> Self {
        Self { mappings: Arc::new(Mutex::new(vec![])), router }
    }
}

impl Config for Redirects {
    fn default() -> Self {
        Self { mappings: Arc::new(Mutex::new(vec![])), router: Arc::new(Mutex::new(Router::new())) }
    }

    fn to_json(&self) -> json::JsonValue {
        let mut json = JsonValue::new_object();
        let mappings = self.mappings.blocking_lock();

        for (key, value) in &*mappings {
            json[key.as_ref()] = JsonValue::String(value.to_string());
        }
        json
    }

    fn load(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let path_cl: Box<str> = path.into();
        let mut router = self.router.blocking_lock();
        let mut mappings = self.mappings.blocking_lock();

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
