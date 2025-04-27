use crate::api::typedef::User;
use super::setup::get_client;

pub async fn get_player_from_uuid(uuid: String) -> Result<User, Box<dyn std::error::Error>> {
    let pool = get_client();
    let mut conn = pool.connect();
    
    //let stmt = conn
    //    .exec_first::<String, Option<String>, bool, u8, Option<String>, String, bool, bool, bool, bool, bool>
    //    (
    //        "SELECT name, email, chat, messages, suffix, lang, tabcompletion, scoreboard, friendReqs, sendPackPrompt, tipFirstFriend FROM players WHERE uuid = ?",
    //        (uuid,)
    //    ).await;
    
    Ok(User::new(String::from(""), String::from("")))
}
