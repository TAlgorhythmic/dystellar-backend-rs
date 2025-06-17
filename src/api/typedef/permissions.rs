use std::{collections::HashMap, sync::{Arc, LazyLock}};

use chrono::{DateTime, Utc};

static POOL: LazyLock<HashMap<i32, Arc<str>>> = LazyLock::new(|| HashMap::new());

pub struct Permission {
    pub permission: Arc<str>,
    pub value: bool
}

pub struct Group {
    id: i32,
    pub name: Box<str>,
    pub prefix: Box<str>,
    pub suffix: Box<str>,
    pub last_modification: DateTime<Utc>
}

impl Group {
    pub fn get_id(&self) -> &i32 {
        &self.id
    }
}
