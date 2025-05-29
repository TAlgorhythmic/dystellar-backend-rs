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
