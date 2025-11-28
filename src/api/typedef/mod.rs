mod user;
mod http;
mod microsoft;
pub mod mailing;
pub mod punishments;
pub mod permissions;
pub mod fs_json;
pub mod routing;

pub use user::User;
pub use microsoft::SigninState;
pub use microsoft::UserCredentials;
pub use microsoft::MinecraftData;
pub use microsoft::XboxLiveTokensData;
pub use microsoft::MicrosoftTokens;
pub use http::BackendError;

use json::JsonValue;
use std::path::Path;
use std::{error::Error, fs, io::Read};
use zip::ZipArchive;
use json::object;
use sha2::{Digest, Sha256};
use sha2::digest::DynDigest;

pub fn generate_sha256(file: &str) -> Result<Box<str>, Box<dyn Error + Send + Sync>> {
    let mut hasher = Sha256::new();

    let mut file = fs::File::open(file)?;
    let mut bytes = [0u8; 8192];

    loop {
        let rd = file.read(&mut bytes)?;
        if rd == 0 {
            break;
        }

        DynDigest::update(&mut hasher, &bytes[..rd]);
    }
    let result = hasher.finalize();
    Ok(format!("{:x}", result).into_boxed_str())
}

/**
* Trait that allows to easily (de)serialize from database/storage.
*/
pub trait Serializable {
    fn load(key: &str) -> Result<Option<Self>, Box<dyn Error + Send + Sync>> where Self: Sized;
    fn save(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[derive(Debug, Clone)]
pub struct ModMetadata {
    pub name: Box<str>,
    pub version: Box<str>,
    pub filename: Box<str>,
    pub authors: Box<str>,
    pub sha256: Box<str>
}

impl Into<JsonValue> for ModMetadata {
    fn into(self) -> JsonValue {
        return object! {
            name: self.name.as_ref(),
            version: self.version.as_ref(),
            filename: self.filename.as_ref(),
            authors: self.authors.as_ref(),
            sha256: self.sha256.as_ref()
        }
    }
}

fn extract_toml_entry(entry: &str) -> Result<Box<str>, Box<dyn Error + Send + Sync>> {
    Ok(unsafe {
        entry.get_unchecked((entry.find('"').ok_or::<Box<dyn Error + Send + Sync>>("Malformed toml file".into())? + 1)..entry.len() - 1).into()
    })
}

impl ModMetadata {
    pub fn from_path(path: &str) -> Result<ModMetadata, Box<dyn Error + Send + Sync>> {
        let path_struct = Path::new(path);
        let file = fs::File::open(path_struct)?;
        let mut archive = ZipArchive::new(file)?;
        let mut buf = String::new();

        let mut metadata = archive.by_name("META-INF/neoforge.mods.toml")?;
        metadata.read_to_string(&mut buf)?;

        let mut name: Option<Box<str>> = None;
        let mut version: Option<Box<str>> = None;
        let mut authors: Option<Box<str>> = None;

        let mut in_section = false;
        for line in buf.lines() {
            if line == "[[mods]]" {
                in_section = true;
            } else if line.starts_with('[') {
                break;
            }

            if in_section {
                if line.starts_with("modId") {
                    name = Some(extract_toml_entry(line)?);
                } else if line.starts_with("version") {
                    version = Some(extract_toml_entry(line)?);
                } else if line.starts_with("authors") {
                    authors = Some(extract_toml_entry(line)?);
                }
            }
        }

        if name.is_none() || version.is_none() || authors.is_none() {
            return Err("Incomplete mod metadata".into());
        }
        let sha256 = generate_sha256(path)?;

        Ok(ModMetadata { name: name.unwrap(), version: version.unwrap(), filename: path_struct.file_name().unwrap().to_str().unwrap().into(), authors: authors.unwrap(), sha256 })
    }
}
