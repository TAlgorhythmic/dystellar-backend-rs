use std::sync::{Arc, LazyLock};

use libsql::Database;

const DB_VERSION: u16 = 0;
const DB_URL: &str = env!("DB_URL");

#[allow(deprecated)]
const CLIENT: LazyLock<Arc<Database>> = LazyLock::new(|| Arc::new(Database::open(DB_URL).expect("Failed to open database.")));

pub fn get_client() -> Arc<Database> {
    CLIENT.clone()
}

pub async fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    let schema = include_bytes!("../../../../schema.sql");
    let queries: std::borrow::Cow<'_, str> = String::from_utf8_lossy(schema);
    
    for query in queries.split('·').into_iter() {
        query_unsafe(query).await?;
    }

    // Init metadata if not exists, this is to keep track of changes into the database.
    query_unsafe("CREATE TABLE IF NOT EXISTS metadata(id INT PRIMARY KEY, version INT);").await?;
    query_unsafe(format!("INSERT OR IGNORE INTO metadata(id, version) VALUES(0, {DB_VERSION});").as_str()).await?;

    println!("libsql initialized correctly!");
    Ok(())
}

async fn query_unsafe(str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = CLIENT.connect()?;

    conn.execute(str, ()).await?;
    Ok(())
}
