pub struct SigninState {
    auth: bool,
    code: Option<Box<str>>
}

pub struct MinecraftLoginData {
    uuid: Option<Box<str>>,
    access_token: Option<Box<str>>,
    refresh_token: Option<Box<str>>,
    expires: Option<i32>,
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

impl MinecraftLoginData {
    pub fn new() -> MinecraftLoginData {
        Self { uuid: None, access_token: None, refresh_token: None, expires: None }
    }

    pub fn get_uuid(&self) -> &Option<Box<str>> {
        &self.uuid
    }
    pub fn get_token(&self) -> &Option<Box<str>> {
        &self.access_token
    }
    pub fn get_refresh_token(&self) -> &Option<Box<str>> {
        &self.refresh_token
    }
    pub fn get_expiration(&self) -> &Option<i32> {
        &self.expires
    }

    pub fn set_uuid(&mut self, uuid: Box<str>) {
        self.uuid = Some(uuid);
    }
    pub fn set_token(&mut self, token: Box<str>) {
        self.access_token = Some(token);
    }
    pub fn set_refresh_token(&mut self, refresh_token: Box<str>) {
        self.refresh_token = Some(refresh_token);
    }
    pub fn set_expiration(&mut self, expiration: i32) {
        self.expires = Some(expiration);
    }
}
