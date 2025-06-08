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

pub async fn get_player_from_uuid(uuid: &str) -> Result<User, Box<dyn Error + Send + Sync>> {
    let client = get_client();
    let conn = client.connect()?;
    
    let mut stmt = conn
        .query(
            "SELECT name, email, chat, messages, suffix, lang, tabcompletion, scoreboard, friendReqs, sendPackPrompt, tipFirstFriend FROM players WHERE uuid = ?1;",
            params!(uuid))
        .await?;

    if let Some(row) = stmt.next().await? {
        let name: &str = row.get_str(0).unwrap();
        let email: Option<String> = row.get(1).unwrap();
        let chat: bool = row.get(2).unwrap();
        let messages: i32 = row.get(3).unwrap();
        let suffix: &str = row.get_str(4).unwrap();
        let lang: &str = row.get_str(5).unwrap();
        let tabcompletion: bool = row.get(6).unwrap();
        let scoreboard: bool = row.get(7).unwrap();
        let friend_reqs: bool = row.get(8).unwrap();
        let pack_prompt: bool = row.get(9).unwrap();
        let tip_first_friend: bool = row.get(10).unwrap();

        return Ok({
            let uuid = uuid.into();
            let name = name.into();
            let email = email.map(|e| e.into());
            let messages = messages as u8;
            let suffix = suffix.into();
            let lang = lang.into();
            User { uuid, name, email, chat: chat, pms: messages, suffix, lang, tabcompletion: tabcompletion, scoreboard: scoreboard, friend_reqs: friend_reqs, send_pack_prompt: pack_prompt, tip_first_friend: tip_first_friend, private }
        }
            let uuid = uuid.into();
            let name = name.into();
            let email = email.map(|e| e.into());
            let messages = messages as u8;
            let suffix = suffix.into();
            let lang = lang.into();
            User { uuid, name, email, chat: chat, pms: messages, suffix, lang, tabcompletion: tabcompletion, scoreboard: scoreboard, friend_reqs: friend_reqs, send_pack_prompt: pack_prompt, tip_first_friend: tip_first_friend, private }
        });
    }
 
    Err("Account not found.".into())
}
