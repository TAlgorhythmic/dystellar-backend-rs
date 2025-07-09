pub mod ban;
pub mod blacklist;
pub mod mute;
pub mod ranked_ban;
pub mod warn;

use std::{cmp::Ordering, error::Error};

use chrono::{DateTime, Utc};
use json::JsonValue;

use crate::api::typedef::punishments::{ban::{Ban, BAN_SERIE_ID}, blacklist::{Blacklist, BLACKLIST_SERIE_ID}, mute::{Mute, MUTE_SERIE_ID}, ranked_ban::{RankedBan, RANKED_SERIE_ID}, warn::{Warn, WARN_SERIE_ID}};

pub fn compare(pun: &Box<dyn Punishment>, other: &Box<dyn Punishment>) -> Ordering {
        if pun.get_priority() != other.get_priority() {
            return pun.get_priority().cmp(&other.get_priority());
        }

        if pun.get_expiration_date().is_none() && other.get_expiration_date().is_some() {
            return Ordering::Less;
        } else if other.get_expiration_date().is_none() && pun.get_expiration_date().is_some() {
            return Ordering::Greater;
        } else if pun.get_expiration_date().is_none() {
            return Ordering::Equal;
        }

        let time = Utc::now().timestamp_millis() - pun.get_expiration_date().unwrap().timestamp_millis();
        let otime = Utc::now().timestamp_millis() - other.get_expiration_date().unwrap().timestamp_millis();
        time.cmp(&otime)
    }

pub fn get_punishment_from_json(json: JsonValue) -> Result<Box<dyn Punishment>, Box<dyn Error + Send + Sync>> {
    let pun_type_opt = json["pun_type"].as_u8();

    if pun_type_opt.is_none() {
        return Err("Malformed json value".into());
    }

    let pun_type = pun_type_opt.unwrap();
    match pun_type {
        BAN_SERIE_ID => Ok(Box::new(Ban::from_json(json))),
        BLACKLIST_SERIE_ID => Ok(Box::new(Blacklist::from_json(json))),
        MUTE_SERIE_ID => Ok(Box::new(Mute::from_json(json))),
        RANKED_SERIE_ID => Ok(Box::new(RankedBan::from_json(json))),
        WARN_SERIE_ID => Ok(Box::new(Warn::from_json(json))),
        _ => Err("Unknown or malformed punishment type".into())
    }
}

pub trait Punishment {
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
    fn to_json(&self) -> JsonValue;
    fn from_json(json: JsonValue) -> Self where Self: Sized;
}
