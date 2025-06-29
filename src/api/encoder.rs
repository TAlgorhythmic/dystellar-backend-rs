use std::error::Error;

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};

pub fn encode_u64(num: u64) -> [u8; 8] {
    [
        (num >> 56) as u8,
        (num >> 48) as u8,
        (num >> 40) as u8,
        (num >> 32) as u8,
        (num >> 24) as u8,
        (num >> 16) as u8,
        (num >> 8) as u8,
        num as u8
    ]
}

pub fn decode_u64(data: &[u8]) -> Result<u64, Box<dyn Error + Send + Sync>> {
    if data.len() < 8 {
        return Err("Malformed data, expected at least size 8".into());
    }

    Ok(((data[0] as u64) << 56) |
    ((data[1] as u64) << 48) |
    ((data[2] as u64) << 40) |
    ((data[3] as u64) << 32) |
    ((data[4] as u64) << 24) |
    ((data[5] as u64) << 16) |
    ((data[6] as u64) << 8)  |
    (data[7] as u64))
}

pub fn encode_datetime(time: DateTime<Utc>) -> [u8; 7] {
    let year = time.year();
    let month = time.month();
    let day = time.day();
    let hour = time.hour();
    let minute = time.minute();
    let second = time.second();

    [(year >> 8) as u8, year as u8, month as u8, day as u8, hour as u8, minute as u8, second as u8]
}

#[allow(deprecated)]
pub fn decode_datetime(time: &[u8]) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    if time.len() < 7 {
        return Err("Malformed array for time management".into())
    }
    let year = ((time[0] as i16) << 8) | time[1] as i16;
    let month = time[2];
    let day = time[3];
    let hour = time[4];
    let minute = time[5];
    let second = time[6];

    let date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32);
    let time = NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32);
    if date.is_none() || time.is_none() {
        return Err("Invalid date or time values".into());
    }
    Ok(DateTime::from_utc(NaiveDateTime::new(date.unwrap(), time.unwrap()), Utc))
}
