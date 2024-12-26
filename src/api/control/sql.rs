use std::sync::{LazyLock, Mutex};

use crate::api::typedef::User;
use mysql::{Opts, Pool};
use mysql::prelude::*;

const DB_URL: &str = env!("DB_URL");

const POOL: LazyLock<Mutex<Option<Pool>>> = LazyLock::new(|| Mutex::new(None));

fn get_pool() -> Pool {
    let mut pool = POOL.lock().unwrap();

    if pool.is_none() {
        let new_pool = Pool::new(Opts::from_url(DB_URL).expect("Error creating opts.")).expect("Error initializing database pool.");

        *pool = Some(new_pool);
    };
    pool.clone().unwrap()
}

pub fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    let schema = include_bytes!("../../../schema.sql");
    let queries: Vec<&str> = String::from_utf8_lossy(schema).split(';').collect();
    
    Ok(())
}

pub fn query_unsafe(str: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    conn.query_drop(str)?;
    Ok(())
}

pub async fn get_player(uuid: String) -> User {

} 
