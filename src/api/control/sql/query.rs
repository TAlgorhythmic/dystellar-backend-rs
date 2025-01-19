use crate::api::typedef::User;
use super::setup::get_pool;
use mysql_async::prelude::*;

pub async fn get_player_from_uuid(uuid: String) -> Result<User, Box<dyn std::error::Error>> {
    let pool = get_pool();
    let mut conn = pool.get_conn().await?;
    
    //let stmt = conn
    //    .exec_first::<String, Option<String>, bool, u8, Option<String>, String, bool, bool, bool, bool, bool>
    //    (
    //        "SELECT name, email, chat, messages, suffix, lang, tabcompletion, scoreboard, friendReqs, sendPackPrompt, tipFirstFriend FROM players WHERE uuid = ?",
    //        (uuid,)
    //    ).await;
    
    Ok(User::new(String::from(""), String::from("")))
}
