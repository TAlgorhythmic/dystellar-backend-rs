use std::{error::Error, str::from_utf8, sync::{Arc, LazyLock}};

use sled::Tree;

use crate::api::typedef::{permissions::Group, User};
use super::setup::get_client;

// Trees
static USERS: LazyLock<Arc<Tree>> = LazyLock::new(|| Arc::new(get_client().open_tree("users").expect("Failed to open 'users' tree")));
static GROUPS: LazyLock<Arc<Tree>> = LazyLock::new(|| Arc::new(get_client().open_tree("groups").expect("Failed to open 'groups' tree")));
static MAILS: LazyLock<Arc<Tree>> = LazyLock::new(|| Arc::new(get_client().open_tree("mails").expect("Failed to open 'mails' tree")));

pub fn create_new_player(uuid: &str, name: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = get_client();
    Ok(())
}

pub fn get_default_group_name() -> Result<Option<Box<str>>, Box<dyn Error + Send + Sync>> {
    let client = get_client();

    let group = client.get(b"default_group")?;
    if group.is_none() {
        return Ok(None);
    }

    Ok(Some(from_utf8(&group.unwrap()).unwrap().into()))
}

pub fn set_default_group(name: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    get_client().insert(b"default_group", name)?;

    Ok(())
}

pub fn get_group_full(name: &str) -> Result<Option<Group>, Box<dyn Error + Send + Sync>> {
    let group_opt = GROUPS.get_lt(name)?;
    
    Ok(group_opt.map(|json| json.into::<Group>()))
}

pub async fn get_player_from_uuid_full(uuid: &str) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
    
}
