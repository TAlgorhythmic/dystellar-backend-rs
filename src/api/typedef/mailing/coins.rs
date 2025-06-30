use chrono::{DateTime, Utc};
use json::{object, JsonValue};

use crate::api::{encoder::decode_datetime, typedef::User};

use super::{Mail, Claimable};

static COINS_SERIAL_ID: u8 = 1;

pub struct Coins {
    message: Box<str>,
    submission_date: DateTime<Utc>,
    sender: Box<str>,
    is_deleted: bool,
    coins: u64,
    is_claimed: bool
}

impl From<JsonValue> for Coins {
    fn from(value: JsonValue) -> Self {
        let submission_date_opt = value["submission_date"].as_str();
        let submission_date = if let Some(str) = submission_date_opt {decode_datetime(str.as_bytes()).unwrap_or(Utc::now())} else {Utc::now()};

        Self {
            message: value["msg"].as_str().unwrap_or("Message not provided.").into(),
            submission_date,
            sender: value["sender"].as_str().unwrap_or("Unknown sender").into(),
            is_deleted: value["deleted"].as_bool().unwrap_or(false),
            coins: value["coins"].as_u64().unwrap_or(0),
            is_claimed: value["claimed"].as_bool().unwrap_or(true)
        }
    }
}

impl Mail for Coins {
    fn get_serial_id(&self) -> u8 {
        COINS_SERIAL_ID
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
            "type": COINS_SERIAL_ID,
            "msg": self.message.as_ref(),
            "submission_date": self.submission_date.to_string(),
            "sender": self.sender.as_ref(),
            "deleted": self.is_deleted,
            "coins": self.coins,
            "claimed": self.is_claimed
        }
    }
}

impl Claimable for Coins {
    fn is_claimed(&self) -> &bool {
        &self.is_claimed
    }

    fn claim(&mut self, user: &mut User) {
        user.set_coins(*user.get_coins() + self.coins);
        self.is_claimed = true;
    }
}
