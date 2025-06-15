use std::{collections::HashMap, sync::{Arc, LazyLock}};

use chrono::{DateTime, Utc};

static POOL: LazyLock<HashMap<i32, Arc<str>>> = LazyLock::new(|| HashMap::new());

pub struct Permission {
    permission: Arc<str>,
    value: bool
}

pub struct Group {
    id: i32,
    name: Box<str>,
    prefix: Box<str>,
    suffix: Box<str>,
    last_modification: DateTime<Utc>
}
