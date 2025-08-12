use std::error::Error;

use crate::api::{typedef::{fs_json::{redirects::Redirects, Config}}};

pub fn register() -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = Redirects::open("redirections.json")?;

    Ok(())
}
