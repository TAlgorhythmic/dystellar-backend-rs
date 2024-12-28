const PMS_ENABLED: u8 = 0;
const PMS_ENABLED_WITH_IGNORELIST: u8 = 1;
const PMS_ENABLED_FRIENDS_ONLY: u8 = 2;
const PMS_DISABLED: u8 = 3;

pub struct User {
    uuid: String,
    name: String,
    email: Option<String>,
    chat: bool,
    messages: u8,
    suffix: Option<String>,
    lang: String,
    tabcompletion: bool,
    scoreboard: bool,
    friend_reqs: bool,
    send_pack_prompt: bool,
    tip_first_friend: bool
}

impl User {
    pub fn new(uuid: String, name: String) -> Self {
        Self { uuid, name, email: None, chat: true, messages: PMS_ENABLED_WITH_IGNORELIST, suffix: None, lang: String::from("en"), tabcompletion: false, scoreboard: true, friend_reqs: true, send_pack_prompt: true, tip_first_friend: false }
    }
    
    pub fn from_existing(uuid: String, name: String, email: Option<String>, chat: bool, messages: u8, suffix: Option<String>, lang: String, tabcompletion: bool, scoreboard: bool, friend_reqs: bool, send_pack_prompt: bool, tip_first_friend: bool) -> Self {
        Self { uuid, name, email, chat, messages, suffix, lang, tabcompletion, scoreboard, friend_reqs, send_pack_prompt, tip_first_friend }
    }
}
