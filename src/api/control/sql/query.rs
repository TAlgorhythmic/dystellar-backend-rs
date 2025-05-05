use std::error::Error;

use libsql::params;

use crate::api::typedef::User;
use super::setup::get_client;

pub async fn get_player_from_uuid(uuid: &str) -> Result<User, Box<dyn Error + Send + Sync>> {
    let client = get_client();
    let conn = client.connect()?;
    
    let stmt = conn
        .query(
            "SELECT name, email, chat, messages, suffix, lang, tabcompletion, scoreboard, friendReqs, sendPackPrompt, tipFirstFriend FROM players WHERE uuid = ?1;",
            params!(uuid))
        .await?;

    if let Some(row) = stmt.next().await? {
        return Ok(User::from_existing(uuid.into(), row.get_str(0).unwrap().into(), row.get(1).unwrap(),
            row.get(2).unwrap(), row.get(3).unwrap(), row.get(4).unwrap(), , tabcompletion, scoreboard, 
            friend_reqs, send_pack_prompt, tip_first_friend)
        );
    }
 
    Err("Account not found.".into())
}
