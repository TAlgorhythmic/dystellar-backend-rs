use chrono::{DateTime, Utc};
use json;
use json::{object, JsonValue};

use crate::api::control::storage::query::get_default_group_name;
use crate::api::typedef::mailing::Mail;
use crate::api::typedef::permissions::{Group, Permission};

static PMS_ENABLED: u8 = 0;
static PMS_ENABLED_FRIENDS_ONLY: u8 = 1;
static PMS_DISABLED: u8 = 2;

pub struct User {
    uuid: Box<str>,
    name: Box<str>,
    email: Option<Box<str>>,
    chat: bool,
    pms: u8,
    suffix: Box<str>,
    lang: Box<str>,
    scoreboard: bool,
    coins: u64,
    friend_reqs: bool,
    created_at: DateTime<Utc>,
    friends: Vec<Box<str>>,
    ignores: Vec<Box<str>>,
    inbox: Vec<Box<dyn Mail>>,
    perms: Vec<Permission>,
    group: Option<Group>
}

impl From<User> for JsonValue {
    fn from(value: User) -> Self {
        let res = object! {
            uuid: value.uuid.as_ref(),
            name: value.name.as_ref(),
            suffix: value.suffix.as_ref(),
        };
        res
    }
}

impl User {
    pub fn new_default(uuid: &str, name: &str) -> Self {
        Self {
            uuid: uuid.into(), name: name.into(), email: None, chat: true, pms: PMS_ENABLED,
            suffix: "".into(), lang: "en".into(), scoreboard: true, coins: 0, friend_reqs: true,
            created_at: Utc::now(), friends: vec![], ignores: vec![], inbox: vec![], group: get_default_group_name()
        }
    }

    pub fn get_coins(&self) -> &u64 {
        &self.coins
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
        self.pms = dms;
    }

    pub fn set_suffix(&mut self, suffix: Box<str>) {
        self.suffix = suffix;
    }

    pub fn set_lang(&mut self, lang: &str) {
        self.lang = lang.into();
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

    pub fn set_coins(&mut self, coins: u64) {
        self.coins = coins;
    }

    pub fn to_json_complete(&self) -> JsonValue {
        object! {
            uuid: self.uuid.as_ref(),
            name: self.name.as_ref(),
            email: match &self.email {
                Some(email) => email.as_ref().into(),
                None => JsonValue::Null
            },
            chat: self.chat,
            pms: self.pms,
            suffix: self.suffix.as_ref(),
            lang: self.lang.as_ref(),
            scoreboard: self.scoreboard,
            friend_reqs: self.friend_reqs,
            send_pack_prompt: self.send_pack_prompt,
            tip_first_friend: self.tip_first_friend
        }
    }
}
