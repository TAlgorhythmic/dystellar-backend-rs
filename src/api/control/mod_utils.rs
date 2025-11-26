use std::{error::Error, fs::File, io::Read};

use zip::ZipArchive;

use crate::api::typedef::Mod;

fn extract_toml_entry(entry: &str) -> Result<Box<str>, Box<dyn Error + Send + Sync>> {
    Ok(unsafe {
        entry.get_unchecked(entry.find('"').ok_or::<Box<dyn Error + Send + Sync>>("Malformed toml file".into())?..entry.len() - 1).into()
    })
}

pub fn read_mod_metadata(path: &str) -> Result<Mod, Box<dyn Error + Send + Sync>> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut buf = String::new();

    let mut metadata = archive.by_name("META-INF/neoforge.mods.toml")?;
    metadata.read_to_string(&mut buf)?;

    let mut name: Option<Box<str>> = None;
    let mut version: Option<Box<str>> = None;
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
            }
        }
    }

    if name.is_none() || version.is_none() {
        return Err("Incomplete mod metadata".into());
    }

    Ok(Mod { name: name.unwrap(), version: version.unwrap(), filename: path.into() })
}
