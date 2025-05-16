static PMS_ENABLED: u8 = 0;
static PMS_ENABLED_WITH_IGNORELIST: u8 = 1;
static PMS_ENABLED_FRIENDS_ONLY: u8 = 2;
static PMS_DISABLED: u8 = 3;

pub struct User {
    uuid: Box<str>,
    name: Box<str>,
    email: Option<Box<str>>,
    chat: bool,
    messages: u8,
    suffix: Box<str>,
    lang: Box<str>,
    tabcompletion: bool,
    scoreboard: bool,
    friend_reqs: bool,
    send_pack_prompt: bool,
    tip_first_friend: bool
}

impl User {
    pub fn new(uuid: &str, name: &str) -> Self {
        Self { uuid: uuid.into(),
            name: name.into(),
            email: None,
            chat: true,
            messages: PMS_ENABLED_WITH_IGNORELIST,
            suffix: "".into(),
            lang: "en".into(),
            tabcompletion: false,
            scoreboard: true,
            friend_reqs: true,
            send_pack_prompt: true,
            tip_first_friend: false
        }
    }
    
    pub fn from_existing(uuid: Box<str>, name: Box<str>, email: Option<Box<str>>, chat: bool, messages: u8, suffix: Box<str>, lang: Box<str>, tabcompletion: bool, scoreboard: bool, friend_reqs: bool, send_pack_prompt: bool, tip_first_friend: bool) -> Self {
        Self { uuid, name, email, chat, messages, suffix, lang, tabcompletion, scoreboard, friend_reqs, send_pack_prompt, tip_first_friend }
    }

    pub fn set_name(&mut self, name: Box<str>) {
        self.name = name;
    }

    pub fn set_email(&mut self, email: Option<Box<str>>) {
        self.email = email;
    }

    pub fn set_chat_enabled(&mut self, chat: bool) {
        self.chat = chat;
    }

    pub fn set_dms_enabled(&mut self, dms: u8) {
        self.messages = dms;
    }

    pub fn set_suffix(&mut self, suffix: Box<str>) {
        self.suffix = suffix;
    }

    pub fn set_lang(&mut self, tabcompletion: bool) {
        self.tabcompletion = tabcompletion;
    }

    pub fn set_friend_reqs(&mut self, friend_reqs: bool) {
        self.friend_reqs = friend_reqs;
    }

    pub fn set_pack_prompt(&mut self, send_pack_prompt: bool) {
        self.send_pack_prompt = send_pack_prompt;
    }

    pub fn set_tip_first_friend(&mut self, tip_first_friend: bool) {
        self.tip_first_friend = tip_first_friend;
    }
}
