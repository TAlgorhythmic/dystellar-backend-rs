pub mod ban;
pub mod blacklist;
pub mod mute;
pub mod ranked_ban;
pub mod warn;

use std::{cmp::Ordering, time::Duration};

use chrono::{DateTime, Utc};

pub trait Punishment {
    pub fn get_id(&self) -> &u64;
    pub fn allow_chat(&self) -> &bool;
    pub fn allow_ranked(&self) -> &bool;
    pub fn allow_unranked(&self) -> &bool;
    pub fn allow_join_minigames(&self) -> &bool;
    pub fn get_message(&self) -> &str;
    pub fn get_reason(&self) -> &str;
    pub fn get_creation_date(&self) -> &DateTime<Utc>;
    pub fn get_expiration_date(&self) -> &Option<DateTime<Utc>>;
    pub fn get_priority(&self) -> &u8;

    pub fn compare(&self, other: &Self) -> Ordering {
        if *self.get_priority() != *other.get_priority() {
            return self.get_priority().cmp(other.get_priority());
        }

        if self.get_expiration_date().is_none() && other.get_expiration_date().is_some() {
            return Ordering::Less;
        } else if other.get_expiration_date().is_none() && self.get_expiration_date().is_some() {
            return Ordering::Greater;
        } else if self.get_expiration_date().is_none() {
            return Ordering::Equal;
        }

        let time = Utc::now().timestamp_millis() - self.get_expiration_date().unwrap().timestamp_millis();
        let otime = Duration::from_millis(Utc::now().timestamp_millis() - other.get_expiration_date().unwrap().timestamp_millis());
        time.cmp(otime)
    }
}
