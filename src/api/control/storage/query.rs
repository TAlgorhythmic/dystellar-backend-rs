use std::{error::Error, str::from_utf8, sync::{Arc, LazyLock}};

use json::parse;
use sled::{IVec, Tree};

use crate::api::{encoder::{decode_datetime, decode_u64}, typedef::{mailing::{get_mail_from_json, get_mails_from_json, Mail}, permissions::{Group, Permission}, punishments::{get_punishments_from_json, Punishment}, User}};
use super::setup::get_client;

// Trees
static USERS: LazyLock<Arc<Tree>> = LazyLock::new(|| Arc::new(get_client().open_tree("users").expect("Failed to open 'users' tree")));
static GROUPS: LazyLock<Arc<Tree>> = LazyLock::new(|| Arc::new(get_client().open_tree("groups").expect("Failed to open 'groups' tree")));
static MAILS: LazyLock<Arc<Tree>> = LazyLock::new(|| Arc::new(get_client().open_tree("mails").expect("Failed to open 'mails' tree")));

pub fn create_new_player(uuid: &str, name: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = get_client();
    Ok(())
}

pub fn get_default_group_name() -> Result<Option<IVec>, Box<dyn Error + Send + Sync>> {
    let client = get_client();

    let group = client.get(b"default_group")?;
    if group.is_none() {
        return Ok(None);
    }

    Ok(Some(group.unwrap()))
}

pub fn set_default_group(name: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    get_client().insert(b"default_group", name)?;

    Ok(())
}

pub fn get_group_full(name: &str) -> Result<Option<Group>, Box<dyn Error + Send + Sync>> {
    let tree = GROUPS.clone();

    let modification_raw_opt = tree.get(format!("{name}:modified_at"))?;
    if modification_raw_opt.is_none() {
        return Ok(None);
    }

    let prefix = tree.get(format!("{name}:prefix"))?.unwrap_or("".into());
    let suffix = tree.get(format!("{name}:suffix"))?.unwrap_or("".into());
    let last_modification = decode_datetime(&*modification_raw_opt.unwrap())?;
    let mut perms = vec![];

    for key in tree.scan_prefix(format!("{name}:permissions:")) {
        let (perm, value) = key?;
        perms.push(Permission { permission: from_utf8(&perm)?.into(), value: value[0] != 0 });
    }

    Ok(Some(Group {
        name: name.into(),
        prefix: from_utf8(&prefix)?.into(),
        suffix: from_utf8(&suffix)?.into(),
        perms,
        last_modification
    }))
}

fn get_friends(uuid: &str, tree: &Arc<Tree>) -> Result<Vec<Box<str>>, Box<dyn Error + Send + Sync>> {
    let mut friends: Vec<Box<str>> = vec![];

    for friend in tree.scan_prefix(format!("{uuid}:friends:")) {
        let (key, value) = friend?;
        if value[0] != 0 {
            if let Ok(f) = from_utf8(&key) {
                friends.push(f.into());
            }
        }
    }

    Ok(friends)
}

fn get_group_from_opt(mut name_opt: Option<IVec>) -> Result<Option<Group>, Box<dyn Error + Send + Sync>> {
    if name_opt.is_none() {
        name_opt = get_default_group_name()?;
        if name_opt.is_none() {
            return Ok(None);
        }
    }

    let name = unsafe {name_opt.unwrap_unchecked()};

    get_group_full(from_utf8(&name)?)
}

fn get_ignores(uuid: &str, tree: &Arc<Tree>) -> Result<Vec<Box<str>>, Box<dyn Error + Send + Sync>> {
    let mut ignores: Vec<Box<str>> = vec![];

    for ignore in tree.scan_prefix(format!("{uuid}:ignores:")) {
        let (key, value) = ignore?;
        if value[0] != 0 {
            if let Ok(ig) = from_utf8(&key) {
                ignores.push(ig.into());
            }
        }
    }

    Ok(ignores)
}

fn get_user_mails(uuid: &str, tree: &Arc<Tree>) -> Result<Vec<Box<dyn Mail>>, Box<dyn Error + Send + Sync>> {
    let opt = tree.get(format!("{uuid}:mails"))?;
    if opt.is_none() {
        return Ok(vec![]);
    }

    let json = parse(from_utf8(&opt.unwrap())?)?;

    Ok(get_mails_from_json(json))
}

fn get_user_permissions(uuid: &str, tree: &Arc<Tree>) -> Result<Vec<Permission>, Box<dyn Error + Send + Sync>> {
    let mut perms = vec![];

    for perm in tree.scan_prefix(format!("{uuid}:permissions:")) {
        let (key, value) = perm?;
        perms.push(Permission { permission: from_utf8(&key)?.into(), value: value[0] != 0 });
    }

    Ok(perms)
}

fn get_user_punishments(uuid: &str, tree: &Arc<Tree>) -> Result<Vec<Box<dyn Punishment>>, Box<dyn Error + Send + Sync>> {
    let serie = tree.get(format!("{uuid}:punishments"))?;
    if serie.is_none() {
        return Ok(vec![]);
    }

    Ok(get_punishments_from_json(parse(from_utf8(&serie.unwrap())?)?))
}

pub fn get_player_from_uuid_full(uuid: &str) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
    let tree = USERS.clone();

    let name_opt = tree.get(format!("{uuid}:name"))?;
    if name_opt.is_none() {
        return Ok(None);
    }

    let name_binding = name_opt.unwrap();
    let suffix_binding = tree.get(format!("{uuid}:suffix"))?.unwrap_or("".into());
    let lang_binding = tree.get(format!("{uuid}:lang"))?.unwrap_or("en".into());

    let name = from_utf8(&name_binding)?;
    let email = tree.get(format!("{uuid}:email"))?;
    let chat = tree.get(format!("{uuid}:chat"))?.unwrap_or("1".into())[0] != 0;
    let pms = tree.get(format!("{uuid}:pms"))?.unwrap_or("1".into())[0];
    let suffix = from_utf8(&suffix_binding)?;
    let lang = from_utf8(&lang_binding)?;
    let scoreboard = tree.get(format!("{uuid}:scoreboard"))?.unwrap_or("1".into())[0] != 0;
    let coins = decode_u64(&*tree.get(format!("{uuid}:coins"))?.unwrap_or("\0\0\0\0\0\0\0\0".into()))?;
    let friend_reqs = tree.get(format!("{uuid}:friend_reqs"))?.unwrap_or("1".into())[0] != 0;
    let created_at = decode_datetime(&*tree.get(format!("{uuid}:created_at"))?.unwrap())?;
    let friends: Vec<Box<str>> = get_friends(uuid, &tree)?;
    let ignores: Vec<Box<str>> = get_ignores(uuid, &tree)?;
    let inbox: Vec<Box<dyn Mail>> = get_user_mails(uuid, &tree)?;
    let punishments = get_user_punishments(uuid, &tree)?;
    let perms: Vec<Permission> = get_user_permissions(uuid, &tree)?;
    let group = get_group_from_opt(tree.get(format!("{uuid}:group"))?)?;


    let user = User {
        uuid: uuid.into(),
        name: name.into(),
        email: email.map(|em| from_utf8(&em).unwrap().into()),
        chat, pms,
        suffix: suffix.into(),
        lang: lang.into(),
        scoreboard, coins, friend_reqs,
        created_at, friends, ignores,
        inbox, punishments, perms, group
    };
    Ok(Some(user))
}
