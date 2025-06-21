use std::error::Error;

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::api::typedef::User;
use super::setup::get_client;

pub async fn create_new_player(uuid: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = get_client();

    client.inse
    conn.query("INSERT OR IGNORE INTO players (uuid) VALUES (?1);", params!(uuid)).await?;

    Ok(())
}

pub async fn get_player_from_uuid_full(uuid: &str) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
    let client = get_client();
    let conn = client.connect()?;
    
    let mut stmt = conn
        .query(
            "SELECT * FROM players WHERE uuid = ?1;",
            params!(uuid))
        .await?;

    if let Some(row) = stmt.next().await? {
        let name = row.get_str(2)?;
        let email: Option<&str> = row.get(3)?;
        let chat: bool = row.get(6)?;
        let pms: u8 = row.get::<i32>(7)? as u8;
        let suffix = row.get_str(8)?;
        let lang = row.get_str(9)?;
        let scoreboard: bool = row.get(10)?;
        let friend_reqs: bool = row.get(11)?;
        let pack_prompt: bool = row.get(12)?;
        let tip_first_friend: bool = row.get(13)?;
        let naive = NaiveDateTime::parse_from_str(row.get_str(14)?, "%Y-%m-%d %H:%M:%S")?;
        let creation_date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
        let coins: u64 = row.get::<i64>(15)? as u64;
        let group: u64 = row.get::<i64>(16)? as u64;
        return Ok(Some(User {
            uuid: uuid.into(),
            name: name.into(),
            email: email.map(|v| v.into()),
            chat,
            pms,
            suffix: suffix.into(),
            lang: lang.into(),
            scoreboard,
            coins,
            friend_reqs,
            send_pack_prompt: pack_prompt,
            tip_first_friend,
            friends: vec![],
            ignores: vec![],
            inbox: vec![]
        }));
    }
    Ok(None)
}
