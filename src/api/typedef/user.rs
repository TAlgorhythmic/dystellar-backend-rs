use std::str::from_utf8;

use chrono::{DateTime, Utc};
use json::{object, JsonValue};

use crate::api::control::storage::query::{get_default_group_name, get_group_full};
use crate::api::typedef::mailing::Mail;
use crate::api::typedef::permissions::{Group, Permission};
use crate::api::typedef::punishments::Punishment;

static PMS_ENABLED: u8 = 0;
static PMS_ENABLED_FRIENDS_ONLY: u8 = 1;
static PMS_DISABLED: u8 = 2;

pub struct User {
    pub uuid: Box<str>,
    pub name: Box<str>,
    pub email: Option<Box<str>>,
    pub chat: bool,
    pub pms: u8,
    pub suffix: Box<str>,
    pub lang: Box<str>,
    pub scoreboard: bool,
    pub coins: u64,
    pub friend_reqs: bool,
    pub created_at: DateTime<Utc>,
    pub friends: Vec<Box<str>>,
    pub ignores: Vec<Box<str>>,
    pub inbox: Vec<Box<dyn Mail>>,
    pub punishments: Vec<Box<dyn Punishment>>,
    pub perms: Vec<Permission>,
    pub group: Option<Group>
}

impl From<User> for JsonValue {
    fn from(value: User) -> Self {
        let res = object! {
            uuid: value.uuid.as_ref(),
            name: value.name.as_ref(),
            suffix: value.suffix.as_ref(),
            created_at: value.created_at.to_string(),
            punishments: ""sff
        };
        res
    }
}

impl User {
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
            coins: self.coins,
            friend_reqs: self.friend_reqs,
            created_at: self.created_at.to_string(),
            friends: JsonValue::Array(
                self.friends
                    .iter()
                    .map(|friend| friend.as_ref().into()).collect()
            ),
            ignores: JsonValue::Array(
                self.ignores
                    .iter()
                    .map(|ignore| ignore.as_ref().into()).collect()
            ),
            inbox: JsonValue::Array(
                self.inbox
                    .iter()
                    .map(|mail| mail.to_json()).collect()
            )
        }
    }

    pub fn new_default(uuid: &str, name: &str) -> Self {
        let group_default = match get_default_group_name() {
            Ok(Some(group_name)) => {
                let group_name_str = from_utf8(&group_name);
                match group_name_str {
                    Ok(name_str) => match get_group_full(name_str) {
                        Ok(group) => group,
                        Err(_) => None,
                    },
                    Err(_) => None,
                }
            }
            _ => None,
        };
        

        Self {
            uuid: uuid.into(), name: name.into(), email: None, chat: true, pms: PMS_ENABLED,
            suffix: "".into(), lang: "en".into(), scoreboard: true, coins: 0, friend_reqs: true,
            created_at: Utc::now(), friends: vec![], ignores: vec![], inbox: vec![], perms: vec![], group: group_default
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

    pub fn set_coins(&mut self, coins: u64) {
        self.coins = coins;
    }
}
