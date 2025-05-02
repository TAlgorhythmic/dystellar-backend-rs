pub struct SigninState {
    auth: bool,
    code: Option<Box<str>>
}

impl SigninState {
    pub fn new() -> Self {
        Self {auth: false, code: None}
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth
    }

    pub fn get_code(&self) -> &Option<Box<str>> {
        &self.code
    }

    pub fn set_code(&mut self, code: &str) {
        self.code = Some(code.into());
    }

    pub fn set_authenticated(&mut self, auth: bool) {
        self.auth = auth;
    }
}
