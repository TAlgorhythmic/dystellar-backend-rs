pub mod coins;
pub mod message;

use std::error::Error;

use chrono::{DateTime, Utc};

use crate::api::typedef::User;

pub trait Mail {
    pub fn get_serial_id(&self) -> u8;
    pub fn get_sender(&self) -> &str;
    pub fn get_submission_date(&self) -> &DateTime<Utc>;
    pub fn is_deleted(&self) -> &bool;
    pub fn get_id(&self) -> &u64;
}

pub trait Claimable {
    pub fn is_claimed(&self) -> &bool;
    pub fn claim(&self, user: &mut User);
}
