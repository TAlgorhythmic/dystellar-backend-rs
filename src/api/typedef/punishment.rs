use std::{cmp::Ordering, str::FromStr};

use chrono::{DateTime, Utc};
use json::{JsonValue, object};

use crate::api::typedef::{BackendError, jsonutils::SerializableJson};

pub struct Punishment {
    pub id: u64,
    pub title: Box<str>,
    pub creation_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub reason: Box<str>,
    pub alsoip: bool,
    pub allow_chat: bool,
    pub allow_ranked: bool,
    pub allow_unranked: bool,
    pub allow_join_minigames: bool
}

impl Punishment {
    pub fn get_priority(&self) -> u8 {
        !self.alsoip as u8 +
        !self.allow_chat as u8 +
        !self.allow_ranked as u8 +
        !self.allow_unranked as u8 +
        !self.allow_join_minigames as u8
    }
}

impl SerializableJson for Punishment {
    fn to_json(&self) -> json::JsonValue {
        object! {
            id: self.id,
            title: self.title.as_ref(),
            created_at: self.creation_date.to_string(),
            expiration_date: match self.expiration_date {
                Some(date) => JsonValue::String(date.to_string()),
                _ => JsonValue::Null
            },
            reason: self.reason.as_ref(),
            alsoip: self.alsoip,
            allow_chat: self.allow_chat,
            allow_ranked: self.allow_ranked,
            allow_unranked: self.allow_unranked,
            allow_join_minigames: self.allow_join_minigames,
        }
    }

    fn from_json(json: &JsonValue) -> Result<Self, BackendError> {
        let created_at = json["created_at"].as_str().map(|s| DateTime::from_str(s).unwrap_or(Utc::now())).unwrap_or(Utc::now());
        let expiration_date_opt = json["expiration_date"].clone();
        let expiration_date = match expiration_date_opt {
            JsonValue::Null => None,
            _ => expiration_date_opt.as_str().map(|s| DateTime::from_str(s).unwrap_or(Utc::now()))
        };

        Ok(Self {
            id: json["id"].as_u64().unwrap_or(800),
            title: json["title"].as_str().ok_or(BackendError::new("punishment.title missing", 400))?.into(),
            creation_date: created_at,
            expiration_date: expiration_date,
            reason: json["reason"].as_str().unwrap_or("Unspecified").into(),
            alsoip: json["alsoip"].as_bool().ok_or(BackendError::new("punishment.alsoip missing", 400))?,
            allow_chat: json["allow_chat"].as_bool().ok_or(BackendError::new("punishment.allow_chat missing", 400))?,
            allow_ranked: json["allow_ranked"].as_bool().ok_or(BackendError::new("punishment.allow_ranked missing", 400))?,
            allow_unranked: json["allow_unranked"].as_bool().ok_or(BackendError::new("punishment.allow_unranked missing", 400))?,
            allow_join_minigames: json["allow_join_minigames"].as_bool().ok_or(BackendError::new("punishment.allow_join_minigames missing", 400))?
        })
    }
}

impl Eq for Punishment {}

impl PartialEq for Punishment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.creation_date == other.creation_date && self.expiration_date == other.expiration_date && self.reason == other.reason && self.alsoip == other.alsoip && self.allow_chat == other.allow_chat && self.allow_ranked == other.allow_ranked && self.allow_unranked == other.allow_unranked && self.allow_join_minigames == other.allow_join_minigames
    }
}

impl PartialOrd for Punishment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Punishment {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_priority() != other.get_priority() {
            return self.get_priority().cmp(&other.get_priority());
        }

        if self.expiration_date.is_none() && other.expiration_date.is_some() {
            return Ordering::Less;
        } else if other.expiration_date.is_none() && self.expiration_date.is_some() {
            return Ordering::Greater;
        } else if self.expiration_date.is_none() {
            return Ordering::Equal;
        }

        let time = Utc::now().timestamp_millis() - self.expiration_date.unwrap().timestamp_millis();
        let otime = Utc::now().timestamp_millis() - other.expiration_date.unwrap().timestamp_millis();
        time.cmp(&otime)
    }
}
