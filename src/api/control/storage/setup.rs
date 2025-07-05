use std::{error::Error, sync::{Arc, LazyLock}};

use sled::{open, Db};

static DB_VERSION: u8 = 0;

static CLIENT: LazyLock<Arc<Db>> = LazyLock::new(|| Arc::new(open("data").expect("Failed to create database, likely a permissions problem")));

pub fn get_client() -> Arc<Db> {
    CLIENT.clone()
}

/**
* Change when there are data updates.
*/
fn update() {}

pub async fn init_db() -> Result<(), Box<dyn Error + Send + Sync>> {
    let ret = CLIENT.insert("db_version", &[DB_VERSION])?;

    if let Some(prev) = ret {
        if prev[0] != DB_VERSION {
            update();
        }
    }

    Ok(())
}
