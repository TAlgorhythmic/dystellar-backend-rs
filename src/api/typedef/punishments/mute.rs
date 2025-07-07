use chrono::{DateTime, Utc};

use super::Punishment;

pub static MUTE_SERIE_ID: u8 = 2;

#[derive(Eq)]
pub struct Mute {
    id: u64,
    creation_date: DateTime<Utc>,
    expiration_date: Option<DateTime<Utc>>,
    reason: Box<str>,
    alsoip: bool
}

impl Punishment for Mute {
    fn get_id(&self) -> &u64 {
        &self.id
    }

    fn allow_chat(&self) -> bool {
        false
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
        3
    }

    fn get_type(&self) -> u8 {
        MUTE_SERIE_ID
    }

    fn is_also_ip(&self) -> &bool {
        &self.alsoip
    }

    fn to_json(&self) -> json::JsonValue {
        todo!()
    }
}

impl PartialOrd for Mute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Mute {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.compare(other)
    }
}

impl PartialEq for Mute {
    fn eq(&self, other: &Self) -> bool {
        *self.get_id() == *other.get_id()
    }
}
