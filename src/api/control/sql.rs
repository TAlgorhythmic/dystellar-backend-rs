use std::sync::{LazyLock, Mutex};

use crate::api::typedef::User;
use mysql::{Opts, Pool};
use mysql::prelude::*;

const DB_VERSION: i16 = 0;
const DB_URL: &str = env!("DB_URL");

const POOL: LazyLock<Mutex<Option<Pool>>> = LazyLock::new(|| Mutex::new(None));

fn get_pool() -> Pool {
    let binding = POOL;
    let mut pool = binding.lock().unwrap();

    if pool.is_none() {
        let new_pool = Pool::new(Opts::from_url(DB_URL).expect("Error creating opts.")).expect("Error initializing database pool.");

        *pool = Some(new_pool);
    };
    pool.clone().unwrap()
}

pub fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    let schema = include_bytes!("../../../schema.sql");
    let queries: std::borrow::Cow<'_, str> = String::from_utf8_lossy(schema);
    
    for query in queries.split('·').into_iter() {
        query_unsafe(query)?;
    }

    // Init metadata if not exists, this is to keep track of changes into the database.
    query_unsafe("CREATE TABLE IF NOT EXISTS metadata(id INT PRIMARY KEY, version INT);")?;
    query_unsafe(format!("INSERT INTO metadata(id, version) VALUES(0, {DB_VERSION}) WHERE NOT EXISTS (SELECT * FROM metadata WHERE id = 0);").as_str());
    Ok(())
}

fn query_unsafe(str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool();
    let mut conn = pool.get_conn()?;

    conn.query_drop(str)?;
    Ok(())
}

pub async fn get_player(uuid: String) -> User {

}
