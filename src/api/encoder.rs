use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};

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
pub fn decode_datetime(time: &[u8; 7]) -> DateTime<Utc> {
    let year = ((time[0] as i16) << 8) | time[1] as i16;
    let month = time[2];
    let day = time[3];
    let hour = time[4];
    let minute = time[5];
    let second = time[6];

    let date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).expect("date is invalid for some reason");
    let time = NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32).expect("time is invalid for some reason");
    DateTime::from_utc(NaiveDateTime::new(date, time), Utc)
}
