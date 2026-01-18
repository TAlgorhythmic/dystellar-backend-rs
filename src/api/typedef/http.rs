use std::{error::Error, fmt::Display};

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
