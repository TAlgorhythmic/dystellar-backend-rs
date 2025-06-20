pub mod ban;
pub mod blacklist;
pub mod mute;
pub mod ranked_ban;
pub mod warn;

use std::cmp::Ordering;

use chrono::{DateTime, Utc};

pub trait Punishment:  Ord + Eq + PartialOrd + PartialEq {
    fn get_id(&self) -> &u64;
    fn allow_chat(&self) -> bool;
    fn allow_ranked(&self) -> bool;
    fn allow_unranked(&self) -> bool;
    fn allow_join_minigames(&self) -> bool;
    fn get_reason(&self) -> &str;
    fn get_creation_date(&self) -> &DateTime<Utc>;
    fn get_expiration_date(&self) -> &Option<DateTime<Utc>>;
    fn get_priority(&self) -> u8;
    fn get_type(&self) -> u8;
    fn is_also_ip(&self) -> &bool;

    fn compare(&self, other: &Self) -> Ordering {
        if self.get_priority() != other.get_priority() {
            return self.get_priority().cmp(&other.get_priority());
        }

        if self.get_expiration_date().is_none() && other.get_expiration_date().is_some() {
            return Ordering::Less;
        } else if other.get_expiration_date().is_none() && self.get_expiration_date().is_some() {
            return Ordering::Greater;
        } else if self.get_expiration_date().is_none() {
            return Ordering::Equal;
        }

        let time = Utc::now().timestamp_millis() - self.get_expiration_date().unwrap().timestamp_millis();
        let otime = Utc::now().timestamp_millis() - other.get_expiration_date().unwrap().timestamp_millis();
        time.cmp(&otime)
    }
}
