use std::error::Error;

use libsql::params;

use crate::api::typedef::User;
use super::setup::get_client;

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

        return Ok(User::from_existing(uuid.into(), name.into(), email.map(|e| e.into()), chat, messages as u8, suffix.into(), lang.into(), tabcompletion, scoreboard, friend_reqs, pack_prompt, tip_first_friend));
    }
 
    Err("Account not found.".into())
}
