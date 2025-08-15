use std::{fs, sync::{Arc, Mutex}};

use json::JsonValue;

use crate::api::{routers::ROUTER, typedef::routing::Method, utils::temporary_redirection};

use super::Config;

pub struct Redirects {
    pub mappings: Arc<Mutex<Vec<(Box<str>, Arc<str>)>>>
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

        let str_opt = fs::read_to_string(path_cl.to_string());
        if let Err(err) = &str_opt {
            eprintln!("Error reading file: {}", err.to_string());
        }

        let str = str_opt.unwrap();
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
                Box::new(move |_| {
                    Box::pin({
                        let url = val.clone();
                        async move {
                            Ok(temporary_redirection(&url))
                        }
                    })
                })
            );
        }

        Ok(())
    }
}
