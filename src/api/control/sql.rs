use std::sync::{LazyLock, Mutex};

use crate::api::typedef::User;
use mysql::{Opts, Pool};
use mysql::prelude::*;

const DB_URL: &str = env!("DB_URL");

const POOL: LazyLock<Mutex<Pool>> = LazyLock::new(|| Mutex::new(Pool::new(Opts::from_url(DB_URL).expect("Error creating opts.")).expect("Error initializing database pool.")));

pub fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    let schema = include_bytes!("../../../schema.sql");
    let queries: Vec<&str> = String::from_utf8_lossy(schema).split(';').collect();
    
    Ok(())
}

pub fn query_unsafe(str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = POOL.get_conn()?;

    conn.query_drop(str)?;
    Ok(())
}

pub async fn get_player(uuid: String) -> User {

} 
