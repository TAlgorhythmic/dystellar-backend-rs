use chrono::{DateTime, Utc};

use super::Punishment;

pub static RANKED_SERIE_ID: u8 = 3;

pub struct RankedBan {
    id: u64,
    creation_date: DateTime<Utc>,
    expiration_date: Option<DateTime<Utc>>,
    reason: Box<str>,
    alsoip: bool
}

impl Punishment for RankedBan {
    fn get_id(&self) -> &u64 {
        &self.id
    }

    fn allow_chat(&self) -> bool {
        true
    }

    fn allow_ranked(&self) -> bool {
        false
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
        4
    }

    fn get_type(&self) -> u8 {
        RANKED_SERIE_ID
    }

    fn is_also_ip(&self) -> &bool {
        &self.alsoip
    }
}

impl PartialOrd for RankedBan {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for RankedBan {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.compare(other)
    }
}

impl PartialEq for RankedBan {
    fn eq(&self, other: &Self) -> bool {
        *self.get_id() == *other.get_id()
    }
}
