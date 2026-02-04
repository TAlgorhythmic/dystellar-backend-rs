use std::{str::from_utf8, sync::{Arc, LazyLock}};

use chrono::{DateTime, Utc};
use json::stringify;
use sled::{IVec, Tree, transaction::ConflictableTransactionError};

use crate::api::{encoder::{decode_datetime, encode_datetime}, typedef::{BackendError, User, jsonutils::SerializableJson, mailing::{Mail, get_json_from_mails, get_mails_from_json}, permissions::{Group, Permission}, punishment::Punishment}};
use super::setup::get_client;

// Trees
static USERS: LazyLock<Arc<Tree>> = LazyLock::new(|| Arc::new(get_client().open_tree("users").expect("Failed to open 'users' tree")));
static NAME_INDEXES: LazyLock<Arc<Tree>> = LazyLock::new(|| Arc::new(get_client().open_tree("nindex").expect("Failed to open 'nindex' tree")));
static IP_INDEXES: LazyLock<Arc<Tree>> = LazyLock::new(|| Arc::new(get_client().open_tree("iindex").expect("Failed to open 'iindex' tree")));
static GROUPS: LazyLock<Arc<Tree>> = LazyLock::new(|| Arc::new(get_client().open_tree("groups").expect("Failed to open 'groups' tree")));
static PUNISHMENTS: LazyLock<Arc<Tree>> = LazyLock::new(|| Arc::new(get_client().open_tree("punishments").expect("Failed to open 'punishments' tree")));
static IP_PUNISHMENTS: LazyLock<Arc<Tree>> = LazyLock::new(|| Arc::new(get_client().open_tree("ip_punishments").expect("Failed to open 'ip_punishments' tree")));

pub fn put_user(user: &User) -> Result<(), BackendError> {
    let tree = USERS.clone();

    tree.transaction(move |tree| {
        let user = user;
        let uuid = user.uuid.as_ref();

        tree.insert(format!("{uuid}:name").as_bytes(), &*user.name)?;
        tree.insert(format!("{uuid}:suffix").as_bytes(), &*user.suffix)?;
        tree.insert(format!("{uuid}:lang").as_bytes(), &*user.lang)?;
        tree.insert(format!("{uuid}:chat").as_bytes(), &[user.chat as u8])?;
        tree.insert(format!("{uuid}:pms").as_bytes(), &[user.pms.clone() as u8])?;
        tree.insert(format!("{uuid}:scoreboard").as_bytes(), &[user.scoreboard as u8])?;
        tree.insert(format!("{uuid}:coins").as_bytes(), &user.coins.to_be_bytes())?;
        tree.insert(format!("{uuid}:friend_reqs").as_bytes(), &[user.friend_reqs as u8])?;
        tree.insert(format!("{uuid}:created_at").as_bytes(), &encode_datetime(user.created_at))?;
        for friend in &user.friends {
            tree.insert(format!("{uuid}:friends:{friend}").as_bytes(), friend.as_bytes())?;
        }

        let pun_tree = PUNISHMENTS.clone();
        for pun in &user.punishments {
            tree.insert(format!("{uuid}:punishments:{}", pun.id).as_bytes(), &pun.id.to_be_bytes())?;
            pun_tree.insert(pun.id.to_be_bytes(), stringify(pun.to_json()).as_bytes())?;
        }
        for perm in &user.perms {
            tree.insert(format!("{uuid}:permissions:{}", perm.perm).as_bytes(), &[perm.value as u8])?;
        }
        if let Some(group) = &user.group {
            tree.insert(format!("{uuid}:group").as_bytes(), group.name.as_bytes())?;
        }
        tree.insert(format!("{uuid}:mails").as_bytes(), stringify(get_json_from_mails(&user.inbox)).as_bytes())?;
        for ignored in &user.ignores {
            tree.insert(format!("{uuid}:ignores:{ignored}").as_bytes(), ignored.as_bytes())?;
        }

        Ok::<(), ConflictableTransactionError>(())
    })?;
    Ok(())
}

pub fn create_new_player(uuid: &str, name: &str) -> Result<User, BackendError> {
    let user = User::new_default(uuid, name);

    put_user(&user)?;
    Ok(user)
}

pub fn create_punishment(
    user_uuid: &str,
    title: &str,
    r#type: &str,
    creation_date: DateTime<Utc>,
    expiration_date: Option<DateTime<Utc>>,
    reason: &str,
    alsoip: bool,
    allow_chat: bool,
    allow_ranked: bool,
    allow_unranked: bool,
    allow_join_minigames: bool
) -> Result<Punishment, BackendError> {
    let tree = PUNISHMENTS.clone();

    let punishment = tree.transaction(|db| {
        let id = db.generate_id()?;
        
        let punishment = Punishment {
            id, title: title.into(),
            r#type: r#type.into(),
            creation_date, expiration_date,
            reason: reason.into(),
            alsoip, allow_chat, allow_ranked,
            allow_unranked, allow_join_minigames
        };

        db.insert(&id.to_be_bytes(), json::stringify(punishment.to_json()).as_bytes())?;

        Ok::<Punishment, ConflictableTransactionError>(punishment)
    })?;

    let tree = USERS.clone();
    tree.insert(format!("{user_uuid}:punishments:{}", punishment.id), &punishment.id.to_be_bytes())?;
    if punishment.alsoip {
        let idx = IP_INDEXES.clone();
        let tree = IP_PUNISHMENTS.clone();

        let ip = idx.get(user_uuid)?.ok_or(BackendError::new("IP not indexed", 400))?;
        let subject_addr = from_utf8(&ip)?;

        tree.insert(format!("{}:{}", subject_addr.get(0..subject_addr.rfind('.').ok_or(BackendError::new("Bad ip address", 400))?).unwrap(), punishment.id), &punishment.id.to_be_bytes())?;
    }

    Ok(punishment)
}

pub fn set_name_index(name: &str, uuid: &str) -> Result<(), BackendError> {
    let tree = NAME_INDEXES.clone();

    tree.insert(name, uuid)?;
    Ok(())
}

pub fn set_ip_index(address: &str, uuid: &str) -> Result<(), BackendError> {
    let tree = IP_INDEXES.clone();

    tree.insert(address, uuid)?;
    Ok(())
}

pub fn get_uuid_by_name(name: &str) -> Result<Option<Box<str>>, BackendError> {
    let tree = NAME_INDEXES.clone();

    Ok(tree.get(name)?.map(|v| from_utf8(&v).unwrap_or("Error").into()))
}

pub fn get_user_by_name(name: &str) -> Result<Option<User>, BackendError> {
    let uuid = get_uuid_by_name(name)?;

    if uuid.is_none() {
        return Ok(None);
    }

    get_user(uuid.unwrap().as_ref())
}

pub fn get_user_connected(uuid: &str, name: &str, address: &str) -> Result<User, BackendError> {
    let mut user = get_user(uuid)?.unwrap_or(create_new_player(uuid, name)?);
    set_name_index(name, uuid)?;
    set_ip_index(address, uuid)?;

    let tree = IP_PUNISHMENTS.clone();
    let puns_tree = PUNISHMENTS.clone();

    for p in tree.scan_prefix(address.get(0..address.rfind('.').ok_or(BackendError::new("Bad ip address", 400))?).unwrap())
        .filter_map(|p| {
            let raw: [u8; 8] = p.ok()?.1.as_ref().try_into().ok()?;
            let id = u64::from_be_bytes(raw);

            Punishment::from_json(
                &json::parse(
                    from_utf8(
                        &puns_tree.get(&id.to_be_bytes()).ok()??
                    ).ok()?
                ).ok()?
            ).ok()
        }) {
        user.punishments.push(p);
    }

    Ok(user)
}

pub fn get_default_group_name() -> Result<Option<IVec>, BackendError> {
    let client = get_client();

    let group = client.get(b"default_group")?;
    if group.is_none() {
        return Ok(None);
    }

    Ok(Some(group.unwrap()))
}

pub fn set_default_group(name: &str) -> Result<(), BackendError> {
    get_client().insert(b"default_group", name)?;

    Ok(())
}

pub fn get_group_full(name: &str) -> Result<Option<Group>, BackendError> {
    let tree = GROUPS.clone();

    let prefix = tree.get(format!("{name}:prefix"))?;
    let suffix = tree.get(format!("{name}:suffix"))?;
    if prefix.is_none() || suffix.is_none() {
        return Ok(None);
    }

    let prefix = prefix.unwrap();
    let suffix = suffix.unwrap();
    let mut perms = vec![];

    for key in tree.scan_prefix(format!("{name}:permissions:")) {
        let (perm, value) = key?;
        perms.push(Permission { perm: from_utf8(&perm)?.into(), value: value[0] != 0 });
    }

    Ok(Some(Group {
        name: name.into(),
        prefix: from_utf8(&prefix)?.into(),
        suffix: from_utf8(&suffix)?.into(),
        perms,
    }))
}

fn get_friends(uuid: &str, tree: &Arc<Tree>) -> Result<Vec<Box<str>>, BackendError> {
    let mut friends: Vec<Box<str>> = vec![];

    for friend in tree.scan_prefix(format!("{uuid}:friends:")) {
        let (_, value) = friend?;
        if let Ok(f) = from_utf8(&value) {
            friends.push(f.into());
        }
    }

    Ok(friends)
}

fn get_group_from_opt(mut name_opt: Option<IVec>) -> Result<Option<Group>, BackendError> {
    if name_opt.is_none() {
        name_opt = get_default_group_name()?;
        if name_opt.is_none() {
            return Ok(None);
        }
    }

    let name = unsafe {name_opt.unwrap_unchecked()};

    get_group_full(from_utf8(&name)?)
}

fn get_ignores(uuid: &str, tree: &Arc<Tree>) -> Result<Vec<Box<str>>, BackendError> {
    let mut ignores: Vec<Box<str>> = vec![];

    for ignore in tree.scan_prefix(format!("{uuid}:ignores:")) {
        let (_, value) = ignore?;
        if let Ok(ig) = from_utf8(&value) {
            ignores.push(ig.into());
        }
    }

    Ok(ignores)
}

fn get_user_mails(uuid: &str, tree: &Arc<Tree>) -> Result<Vec<Box<dyn Mail>>, BackendError> {
    let opt = tree.get(format!("{uuid}:mails"))?;
    if opt.is_none() {
        return Ok(vec![]);
    }

    let json = json::parse(from_utf8(&opt.unwrap())?)?;

    Ok(get_mails_from_json(&json))
}

fn get_user_permissions(uuid: &str, tree: &Arc<Tree>) -> Result<Vec<Permission>, BackendError> {
    let mut perms = vec![];

    let prefix = format!("{uuid}:permissions:");
    for perm in tree.scan_prefix(&prefix) {
        let (key, value) = perm?;
        perms.push(Permission { perm: from_utf8(&key[prefix.len()..])?.into(), value: value[0] != 0 });
    }

    Ok(perms)
}

fn get_user_punishments(uuid: &str) -> Result<Vec<Punishment>, BackendError> {
    let mut puns = vec![];

    let prefix = format!("{uuid}:punishments:");
    let tree = USERS.clone();
    let pun_tree = PUNISHMENTS.clone();

    for pun in tree.scan_prefix(&prefix) {
        let id = from_utf8(&pun?.0)?.parse::<u64>()?;
        let value = pun_tree.get(id.to_be_bytes())?.ok_or(BackendError::new("Punishment not found", 500))?;
        let pun = Punishment::from_json(&json::parse(from_utf8(&value)?)?)?;

        puns.push(pun);
    }

    Ok(puns)
}

pub fn get_user(uuid: &str) -> Result<Option<User>, BackendError> {
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
    let coins = if let Some(data) = tree.get(format!("{uuid}:coins"))? {
        let raw: [u8; 8] = data.as_ref().try_into().map_err(|_| BackendError::new("Corrupt coins data", 500))?;
        u64::from_be_bytes(raw)
    } else { 0 };
    let friend_reqs = tree.get(format!("{uuid}:friend_reqs"))?.unwrap_or("1".into())[0] != 0;
    let created_at = decode_datetime(&*tree.get(format!("{uuid}:created_at"))?.unwrap())?;
    let friends: Vec<Box<str>> = get_friends(uuid, &tree)?;
    let ignores: Vec<Box<str>> = get_ignores(uuid, &tree)?;
    let inbox: Vec<Box<dyn Mail>> = get_user_mails(uuid, &tree)?;
    let punishments = get_user_punishments(uuid)?;
    let perms: Vec<Permission> = get_user_permissions(uuid, &tree)?;
    let group = get_group_from_opt(tree.get(format!("{uuid}:group"))?)?;

    let user = User {
        uuid: uuid.into(),
        name: name.into(),
        email: email.map(|em| from_utf8(&em).unwrap().into()),
        chat, pms: pms.into(),
        suffix: suffix.into(),
        lang: lang.into(),
        scoreboard, coins, friend_reqs,
        created_at, friends, ignores,
        inbox, punishments, perms, group
    };
    Ok(Some(user))
}
