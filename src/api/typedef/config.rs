use std::{default, error::Error, fs};

use json::{object, JsonValue};

struct Config {
    launcher_url: Box<str>,
    launcher_version: Box<str>,
}

impl Config {
    pub fn default() -> Config {
        Config { launcher_url: "launcher_url".into(), launcher_version: "0.0".into() }
    }

    pub fn open(path: &str) -> Result<Config, Box<dyn Error + Send + Sync>> {
        let conf_opt = fs::read_to_string(path);
        if conf_opt.is_err() {
            return Config::default().save(path)
        }

        let json = json::parse(&conf_opt.unwrap())?;
        Ok(Config {
            launcher_url: json["launcher_url"].as_str().unwrap_or("Failed to fetch launcher url").into(),
            launcher_version: json["launcher_version"].as_str().unwrap_or("Failed to fetch launcher version").into()
        })
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
}
