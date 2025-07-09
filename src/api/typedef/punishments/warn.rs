use std::str::FromStr;

use chrono::{DateTime, Utc};
use json::{object, JsonValue};

use super::Punishment;

pub const WARN_SERIE_ID: u8 = 4;

pub struct Warn {
    id: u64,
    creation_date: DateTime<Utc>,
    expiration_date: Option<DateTime<Utc>>,
    reason: Box<str>,
    alsoip: bool
}

impl Punishment for Warn {
    fn get_id(&self) -> &u64 {
        &self.id
    }

    fn allow_chat(&self) -> bool {
        true
    }

    fn allow_ranked(&self) -> bool {
        true
    }

    fn allow_unranked(&self) -> bool {
        true
    }

    fn allow_join_minigames(&self) -> bool {
        true
    }

    fn get_reason(&self) -> &str {
        self.reason.as_ref()
    }

    fn get_creation_date(&self) -> &DateTime<Utc> {
        &self.creation_date
    }

    fn get_expiration_date(&self) -> &Option<DateTime<Utc>> {
        &self.expiration_date
    }

    fn get_priority(&self) -> u8 {
        5
    }

    fn get_type(&self) -> u8 {
        WARN_SERIE_ID
    }

    fn is_also_ip(&self) -> &bool {
        &self.alsoip
    }

    fn to_json(&self) -> json::JsonValue {
        object! {
            id: self.id,
            created_at: self.creation_date.to_string(),
            expiration_date: match self.expiration_date {
                Some(date) => JsonValue::String(date.to_string()),
                _ => JsonValue::Null
            },
            reason: self.reason.as_ref(),
            alsoip: self.alsoip,
            pun_type: WARN_SERIE_ID,
        }
    }

    fn from_json(json: &JsonValue) -> Self where Self: Sized {
        let created_at = json["created_at"].as_str().map(|s| DateTime::from_str(s).unwrap_or(Utc::now())).unwrap_or(Utc::now());
        let expiration_date_opt = json["expiration_date"].clone();
        let expiration_date = match expiration_date_opt {
            JsonValue::Null => None,
            _ => expiration_date_opt.as_str().map(|s| DateTime::from_str(s).unwrap_or(Utc::now()))
        };

        Self {
            id: json["id"].as_u64().unwrap_or(800),
            creation_date: created_at,
            expiration_date: expiration_date,
            reason: json["reason"].as_str().unwrap_or("Unspecified").into(),
            alsoip: json["alsoip"].as_bool().unwrap_or(true)
        }
    }
}
