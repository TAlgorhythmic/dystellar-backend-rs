pub mod coins;
pub mod message;

use std::error::Error;

use chrono::{DateTime, Utc};
use json::JsonValue;

use crate::api::typedef::User;

pub trait Mail: Send + Sync {
    fn from_json(json: &JsonValue) -> Self where Self: Sized;
    fn get_serial_id(&self) -> u8;
    fn get_sender(&self) -> &str;
    fn get_submission_date(&self) -> &DateTime<Utc>;
    fn is_deleted(&self) -> &bool;
    fn to_json(&self) -> JsonValue;
}

pub trait Claimable: Send + Sync {
    fn is_claimed(&self) -> &bool;
    fn claim(&mut self, user: &mut User);
}

pub fn get_mail_from_json(json: &JsonValue) -> Result<Box<dyn Mail>, Box<dyn Error + Send + Sync>> {
    let type_opt = json["type"].as_u8();

    if type_opt.is_none() {
        return Err("Malformed mail, a type field is required".into());
    }

    let serial = unsafe {type_opt.unwrap_unchecked()};

    match serial {
        coins::COINS_SERIAL_ID => Ok(Box::new(coins::Coins::from_json(json))),
        message::MESSAGE_SERIAL_ID => Ok(Box::new(message::Message::from_json(json))),
        _ => Err("This type does not exist".into())
    }
}

pub fn get_mails_from_json(json: &JsonValue) -> Vec<Box<dyn Mail>> {
    let mut res: Vec<Box<dyn Mail>> = vec![];

    for member in json.members() {
        if let Ok(mail) = get_mail_from_json(member) {
            res.push(mail);
        }
    }

    res
}
