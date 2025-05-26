pub struct SigninState {
    auth: bool,
    code: Option<Box<str>>
}

pub struct MinecraftData {
    pub uuid: Box<str>,
    pub token: Box<str>,
    pub expires: i64
}

pub struct UserCredentials {
    pub uuid: Box<str>,
    pub mc_token: Box<str>,
    pub access_token: Box<str>,
    pub refresh_token: Box<str>,
    pub expires: i64
}

pub struct MicrosoftTokens {
    pub access_token: Box<str>,
    pub refresh_token: Box<str>,
    pub expires: i64
}

pub struct XboxLiveTokensData {
    pub token: Box<str>,
    pub uhs: Box<str>
}

impl MinecraftData {
    pub fn new(uuid: &str, token: &str, expires: i64) -> Self {
        Self { uuid: uuid.into(), token: token.into(), expires }
    }

    pub fn get_uuid(&self) -> &Box<str> {
        &self.uuid
    }
    pub fn get_token(&self) -> &Box<str> {
        &self.token
    }
    pub fn get_expiration(&self) -> &i64 {
        &self.expires
    }
}

impl XboxLiveTokensData {
    pub fn new(token: Box<str>, uhs: Box<str>) -> Self {
        Self { token, uhs }
    }

    pub fn get_token(&self) -> &Box<str> {
        &self.token
    }
    pub fn get_uhs(&self) -> &Box<str> {
        &self.uhs
    }
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

impl MicrosoftTokens {
    pub fn new(access_token: Box<str>, refresh_token: Box<str>, expiration: i64) -> Self {
        Self { access_token, refresh_token, expires: expiration }
    }

    pub fn get_token(&self) -> &Box<str> {
        &self.access_token
    }
    pub fn get_refresh_token(&self) -> &Box<str> {
        &self.refresh_token
    }
    pub fn get_expiration(&self) -> &i64 {
        &self.expires
    }

    pub fn set_token(&mut self, token: Box<str>) {
        self.access_token = token;
    }
    pub fn set_refresh_token(&mut self, refresh_token: Box<str>) {
        self.refresh_token = refresh_token;
    }
    pub fn set_expiration(&mut self, expiration: i64) {
        self.expires = expiration;
    }
}

impl UserCredentials {
    pub fn new(uuid: Box<str>, mc_token: Box<str>, access_token: Box<str>, refresh_token: Box<str>, expiration: i64) -> UserCredentials {
        Self { uuid, mc_token, access_token, refresh_token, expires: expiration }
    }

    pub fn get_uuid(&self) -> &Box<str> {
        &self.uuid
    }
    pub fn get_token(&self) -> &Box<str> {
        &self.access_token
    }
    pub fn get_refresh_token(&self) -> &Box<str> {
        &self.refresh_token
    }
    pub fn get_expiration(&self) -> &i64 {
        &self.expires
    }

    pub fn set_uuid(&mut self, uuid: Box<str>) {
        self.uuid = uuid;
    }
    pub fn set_token(&mut self, token: Box<str>) {
        self.access_token = token;
    }
    pub fn set_refresh_token(&mut self, refresh_token: Box<str>) {
        self.refresh_token = refresh_token;
    }
    pub fn set_expiration(&mut self, expiration: i64) {
        self.expires = expiration;
    }
}
