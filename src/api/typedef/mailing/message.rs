use std::str::FromStr;

use chrono::{DateTime, Utc};
use json::{array, object, JsonValue};

use crate::api::encoder::decode_datetime;

use super::Mail;

pub const MESSAGE_SERIAL_ID: u8 = 0;

pub struct Message {
    message: Box<str>,
    submission_date: DateTime<Utc>,
    sender: Box<str>,
    is_deleted: bool
}

impl Mail for Message {
    fn from_json(value: &JsonValue) -> Self {
        let submission_date_opt = value["submission_date"].as_str();
        let submission_date = if let Some(str) = submission_date_opt {DateTime::from_str(str).unwrap_or(Utc::now())} else {Utc::now()};

        Self {
            message: value["msg"].as_str().unwrap_or("Message not provided.").into(),
            submission_date,
            sender: value["sender"].as_str().unwrap_or("Unknown sender").into(),
            is_deleted: value["deleted"].as_bool().unwrap_or(false)
        }
    }

    fn get_serial_id(&self) -> u8 {
        MESSAGE_SERIAL_ID
    }

    fn get_sender(&self) -> &str {
        self.sender.as_ref()
    }

    fn get_submission_date(&self) -> &DateTime<Utc> {
        &self.submission_date
    }

    fn is_deleted(&self) -> &bool {
        &self.is_deleted
    }

    fn to_json(&self) -> JsonValue {
        object! {
            "type": MESSAGE_SERIAL_ID,
            "msg": self.message.as_ref(),
            "submission_date": self.submission_date.to_string(),
            "sender": self.sender.as_ref(),
            "deleted": self.is_deleted
        }
    }
}
