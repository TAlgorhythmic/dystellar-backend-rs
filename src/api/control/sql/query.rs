use std::error::Error;

use libsql::params;

use crate::api::typedef::User;
use super::setup::get_client;

pub async fn create_new_player(uuid: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = get_client();
    let conn = client.connect()?;

    conn.query("INSERT OR IGNORE INTO players (uuid) VALUES (?1);", params!(uuid)).await?;

    Ok(())
}

pub async fn get_player_from_uuid(uuid: &str) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
    let client = get_client();
    let conn = client.connect()?;
    
    let mut stmt = conn
        .query(
            "SELECT * FROM players WHERE uuid = ?1;",
            params!(uuid))
        .await?;

    if let Some(row) = stmt.next().await? {

        return Ok(Some())
    }
    Ok(None)
}
