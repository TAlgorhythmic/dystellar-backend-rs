use chrono::{DateTime, Utc};

pub struct Permission {
    pub permission: Box<str>,
    pub value: bool
}

pub struct Group {
    pub name: Box<str>,
    pub prefix: Box<str>,
    pub suffix: Box<str>,
    pub perms: Vec<Permission>,
    pub last_modification: DateTime<Utc>
}

impl Group {
    pub fn new(name: &str) -> Self {
        Self { name: name.into(), prefix: "".into(), suffix: "".into(), perms: vec![], last_modification: Utc::now() }
    }
}
