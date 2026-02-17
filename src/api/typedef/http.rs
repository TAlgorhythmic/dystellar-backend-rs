use std::{error::Error, fmt::Display, num::ParseIntError, str::Utf8Error, string::FromUtf8Error};

use sled::transaction::TransactionError;
use tungstenite::error::ProtocolError;

#[derive(Debug)]
pub struct BackendError {
    msg: Box<str>,
    status: u16
}

impl BackendError {
    pub fn new(msg: &str, status: u16) -> Self {
        Self { msg: msg.into(), status }
    }

    pub fn get_status(&self) -> &u16 {
        &self.status
    }
    pub fn get_msg(&self) -> &str {
        self.msg.as_ref()
    }
}

impl Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for BackendError {}

impl From<std::io::Error> for BackendError {
    fn from(value: std::io::Error) -> Self {
        let string = value.to_string();
        println!("IO Error: {}", string);

        Self { msg: string.into_boxed_str(), status: 500 }
    }
}

impl From<&str> for BackendError {
    fn from(value: &str) -> Self {
        println!("Error: {}", value);

        Self { msg: value.into(), status: 500 }
    }
}

impl From<FromUtf8Error> for BackendError {
    fn from(value: FromUtf8Error) -> Self {
        let string = value.to_string();
        println!("FromUtf8Error: {}", string);

        Self { msg: string.into_boxed_str(), status: 500 }
    }
}

impl From<sled::Error> for BackendError {
    fn from(value: sled::Error) -> Self {
        println!("Error from sled: {}", value.to_string());

        Self { msg: "Internal Error".into(), status: 500 }
    }
}

impl From<ParseIntError> for BackendError {
    fn from(value: ParseIntError) -> Self {
        println!("Error parsing int: {}", value.to_string());

        Self { msg: "Internal Error".into(), status: 500 }
    }
}

impl From<json::JsonError> for BackendError {
    fn from(value: json::JsonError) -> Self {
        println!("Error from json: {}", value.to_string());

        Self { msg: "Internal Error".into(), status: 500 }
    }
}

impl<T> From<TransactionError<T>> for BackendError where T: Display {
    fn from(value: TransactionError<T>) -> Self {
        println!("Error from sled transaction: {}", value.to_string());

        Self { msg: "Internal Error".into(), status: 500 }
    }
}

impl From<Box<dyn Error + Send + Sync>> for BackendError {
    fn from(value: Box<dyn Error + Send + Sync>) -> Self {
        Self { msg: value.to_string().into_boxed_str(), status: 500 }
    }
}

impl From<Utf8Error> for BackendError {
    fn from(value: Utf8Error) -> Self {
        println!("Error from utf8 parsing: {}", value.to_string());

        Self { msg: "Internal Error".into(), status: 500 }
    }
}

impl From<ProtocolError> for BackendError {
    fn from(value: ProtocolError) -> Self {
        let string = value.to_string();
        println!("ProtocolError: {}", string);

        Self { msg: string.into_boxed_str(), status: 500 }
    }
}

impl From<tungstenite::Error> for BackendError {
    fn from(value: tungstenite::Error) -> Self {
        let string = value.to_string();
        println!("Tungstenite error: {}", value.to_string());

        Self { msg: string.into_boxed_str(), status: 500 }
    }
}
