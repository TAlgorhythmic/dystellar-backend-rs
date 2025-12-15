use std::{error::Error, fs};

use json::{object, JsonValue};

use super::Config;

pub struct State {
    pub launcher_url: Box<str>,
    pub launcher_version: Box<str>,
    pub minecraft_version: Box<str>
}

impl Config for State {
    fn default() -> Self {
        State { launcher_url: "launcher_url".into(), launcher_version: "0.0".into(), minecraft_version: "1.20.1".into() }
    }

    fn to_json(&self) -> JsonValue {
        object! {
            launcher_url: self.launcher_url.as_ref(),
            launcher_version: self.launcher_version.as_ref(),
        }
    }

    fn load(&mut self, path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let str = fs::read_to_string(path)?;

        let json = json::parse(&str)?;
        self.launcher_url = json["launcher_url"].as_str().ok_or("Failed to fetch launcher url")?.into();
        self.launcher_version = json["launcher_version"].as_str().ok_or("Failed to fetch launcher version")?.into();
        self.minecraft_version = json["minecraft_version"].as_str().ok_or("Failed to fetch minecraft version")?.into();

        Ok(())
    }
}
