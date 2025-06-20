use chrono::{DateTime, Utc};

use crate::api::typedef::User;

use super::{Mail, Claimable};

static COINS_SERIAL_ID: u8 = 1;

pub struct Coins {
    id: u64,
    message: Box<[Box<str>]>,
    submission_date: DateTime<Utc>,
    sender: Box<str>,
    is_deleted: bool,
    coins: u64,
    is_claimed: bool
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

    fn get_id(&self) -> &u64 {
        &self.id
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
