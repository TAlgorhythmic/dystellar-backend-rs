use std::{fs, sync::Arc};

use json::JsonValue;

use super::Config;

pub struct Redirects {
    pub mappings: Vec<(Box<str>, Arc<str>)>
}

impl Config for Redirects {
    fn default() -> Self {
        Self { mappings: vec![] }
    }

    fn to_json(&self) -> json::JsonValue {
        let mut json = JsonValue::new_object();

        for (key, value) in &self.mappings {
            json[key.as_ref()] = JsonValue::String(value.to_string());
        }
        json
    }

    fn load(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let str = fs::read_to_string(path)?;
        let json = json::parse(&str)?;

        for (key, value) in json.entries() {
            if let Some(val) = value.as_str() {
                self.mappings.push((key.into(), val.into()));
            }
        }

        Ok(())
    }
}
