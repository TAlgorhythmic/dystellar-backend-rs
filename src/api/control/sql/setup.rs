use std::sync::{LazyLock, Mutex};

use mysql_async::{Opts, Pool};
use mysql_async::prelude::*;

const DB_VERSION: i16 = 0;
const DB_URL: &str = env!("DB_URL");

const POOL: LazyLock<Mutex<Option<Pool>>> = LazyLock::new(|| Mutex::new(None));

pub fn get_pool() -> Pool {
    let binding = POOL;
    let mut pool = binding.lock().unwrap();

    if pool.is_none() {
        let new_pool = Pool::new(DB_URL);

        *pool = Some(new_pool);
    };
    pool.clone().unwrap()
}

pub async fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    let schema = include_bytes!("../../../../schema.sql");
    let queries: std::borrow::Cow<'_, str> = String::from_utf8_lossy(schema);
    
    for query in queries.split('·').into_iter() {
        query_unsafe(query).await?;
    }

    // Init metadata if not exists, this is to keep track of changes into the database.
    query_unsafe("CREATE TABLE IF NOT EXISTS metadata(id INT PRIMARY KEY, version INT);").await?;
    query_unsafe(format!("INSERT IGNORE INTO metadata(id, version) VALUES(0, {DB_VERSION});").as_str()).await?;
    Ok(())
}

async fn query_unsafe(str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool();
    let mut conn = pool.get_conn().await?;

    conn.query_drop(str).await?;
    Ok(())
}
