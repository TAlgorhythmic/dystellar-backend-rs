use chrono::{DateTime, Utc};

use super::Mail;

static MESSAGE_SERIAL_ID: u8 = 0;

pub struct Message {
    id: u64,
    message: Box<[str]>,
    submission_date: DateTime<Utc>,
    sender: Box<str>,
    is_deleted: bool
}

impl Mail for Message {
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

    fn get_id(&self) -> &u64 {
        &self.id
    }
}
