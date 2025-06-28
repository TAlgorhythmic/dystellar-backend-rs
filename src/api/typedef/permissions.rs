use chrono::{DateTime, Utc};

use crate::api::{control::storage::query::get_group_full, typedef::Serializable};

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

impl Serializable for Group {
    fn load(key: &str) -> Result<Option<Self>, Box<dyn std::error::Error + Send + Sync>> where Self: Sized {
        get_group_full(key)
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        todo!()
    }
}
