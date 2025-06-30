pub mod coins;
pub mod message;

use chrono::{DateTime, Utc};
use json::JsonValue;

use crate::api::typedef::User;

pub trait Mail: From<JsonValue> {
    fn get_serial_id(&self) -> u8;
    fn get_sender(&self) -> &str;
    fn get_submission_date(&self) -> &DateTime<Utc>;
    fn is_deleted(&self) -> &bool;
}

pub trait Claimable {
    fn is_claimed(&self) -> &bool;
    fn claim(&mut self, user: &mut User);
}
