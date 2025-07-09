use std::str::FromStr;

use chrono::{DateTime, Utc};
use json::{object, JsonValue};

use super::Punishment;

pub const BLACKLIST_SERIE_ID: u8 = 1;

pub struct Blacklist {
    id: u64,
    creation_date: DateTime<Utc>,
    reason: Box<str>,
    alsoip: bool
}

impl Punishment for Blacklist {
    fn get_id(&self) -> &u64 {
        &self.id
    }

    fn allow_chat(&self) -> bool {
        false
    }

    fn allow_ranked(&self) -> bool {
        false
    }

    fn allow_unranked(&self) -> bool {
        false
    }

    fn allow_join_minigames(&self) -> bool {
        false
    }

    fn get_reason(&self) -> &str {
        self.reason.as_ref()
    }

    fn get_creation_date(&self) -> &DateTime<Utc> {
        &self.creation_date
    }

    fn get_expiration_date(&self) -> &Option<DateTime<Utc>> {
        &None
    }

    fn get_priority(&self) -> u8 {
        0
    }

    fn get_type(&self) -> u8 {
        BLACKLIST_SERIE_ID
    }

    fn is_also_ip(&self) -> &bool {
        &self.alsoip
    }

    fn to_json(&self) -> json::JsonValue {
        object! {
            id: self.id,
            created_at: self.creation_date.to_string(),
            expiration_date: JsonValue::Null,
            reason: self.reason.as_ref(),
            alsoip: self.alsoip,
            pun_type: BLACKLIST_SERIE_ID,
        }
    }

    fn from_json(json: &JsonValue) -> Self where Self: Sized {
        let created_at = json["created_at"].as_str().map(|s| DateTime::from_str(s).unwrap_or(Utc::now())).unwrap_or(Utc::now());

        Self {
            id: json["id"].as_u64().unwrap_or(800),
            creation_date: created_at,
            reason: json["reason"].as_str().unwrap_or("Unspecified").into(),
            alsoip: json["alsoip"].as_bool().unwrap_or(true)
        }
    }
}
