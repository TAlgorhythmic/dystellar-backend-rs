use std::{collections::HashMap, convert::Infallible, error::Error, str::from_utf8, sync::Arc, time::Duration};

use chrono::DateTime;
use futures::{SinkExt, StreamExt};
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response, Version, body::{Buf, Bytes, Incoming}, header::AUTHORIZATION};
use json::{JsonValue, object};
use tokio::{sync::{Mutex, mpsc::{UnboundedSender, unbounded_channel}}, task::JoinHandle};
use tokio_util::bytes::{BufMut, BytesMut};
use tungstenite::{Message, protocol::WebSocketConfig};

use crate::api::{control::ioutils::{encode_msg, read_prefixed_string}, typedef::CacheData};
use crate::api::{control::storage::query::{create_punishment, get_all_groups_full, get_default_group_name, get_user, get_user_connected, put_user}, typedef::{BackendError, User, jsonutils::SerializableJson, routing::{Method, nodes::Router}}, utils::{HttpTransaction, get_body_json, get_body_url_args, response_json}};

static TOKEN: &str = env!("PRIVILEGE_TOKEN");
static ALLOWED_IP: &str = env!("PRIVILEGED_AUTHORIZED_IP");

fn check_token(req: &Request<Incoming>) -> Result<(), BackendError> {
    let http = req.headers().to_owned();

    let header = http.get(AUTHORIZATION);
    if let Some(h) = header && h.to_str().unwrap() == TOKEN {
        return Ok(());
    }

    Err(BackendError::new("Operation not permitted.", 401))
}

/**
* Punish a player, this creates a punishment, assigns it to the player and returns it.
*/
async fn punish(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if ALLOWED_IP != req.uri().host().unwrap() {
        return Err(BackendError::new("Operation not permitted.", 401));
    }
    check_token(&req)?;
    let json = get_body_json(HttpTransaction::Req(req)).await?;

    let user_uuid = json["user_uuid"].as_str().ok_or(BackendError::new("user_uuid missing", 400))?;
    let r#type = json["type"].as_str().ok_or(BackendError::new("type missing", 400))?;
    let title = json["title"].as_str().ok_or(BackendError::new("title missing", 400))?;
    let creation_date = DateTime::from_timestamp_millis(
        json["creation_date"].as_i64().ok_or(BackendError::new("creation_date missing", 400))?
    ).ok_or(BackendError::new("creation date is invalid", 400))?;
    let expiration_date = match json["expiration_date"].as_i64() {
        Some(n) => Some(DateTime::from_timestamp_millis(n).ok_or(BackendError::new("expiration date invalid", 400))?),
        _ => None
    };
    let reason = json["reason"].as_str().ok_or(BackendError::new("reason missing", 400))?;
    let alsoip = json["alsoip"].as_bool().unwrap_or(false);
    let allow_chat = json["allow_chat"].as_bool().unwrap_or(false);
    let allow_ranked = json["allow_ranked"].as_bool().unwrap_or(false);
    let allow_unranked = json["allow_unranked"].as_bool().unwrap_or(false);
    let allow_join_minigames = json["allow_join_minigames"].as_bool().unwrap_or(false);

    let pun = create_punishment(user_uuid, title, r#type, creation_date, expiration_date, reason, alsoip, allow_chat, allow_ranked, allow_unranked, allow_join_minigames)?;
    Ok(response_json(pun.to_json()))
}

/**
* An endpoint used to get the full data of a user, requires a unique token and being from an
* authorized IP.
*/
async fn player_data(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if ALLOWED_IP != req.uri().host().unwrap() {
        return Err(BackendError::new("Operation not permitted.", 401));
    }

    let args = get_body_url_args(&req)?;
    let uuid = args.get("uuid").ok_or(BackendError::new("Malformed url, uuid expected", 400))?;
    check_token(&req)?;
    
    let data = get_user(uuid)?.ok_or(BackendError::new("User not found", 404))?;

    Ok(response_json(data.to_json()))
}

async fn user_connected(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if ALLOWED_IP != req.uri().host().unwrap() {
        return Err(BackendError::new("Operation not permitted.", 401));
    }

    let args = get_body_url_args(&req)?;
    check_token(&req)?;

    let uuid = args.get("uuid").ok_or(BackendError::new("Falformed url, uuid expected", 400))?;
    let name = args.get("name").ok_or(BackendError::new("Falformed url, uuid expected", 400))?;
    let address = args.get("address").ok_or(BackendError::new("Falformed url, address expected", 400))?;

    let data = get_user_connected(uuid.as_ref(), name.as_ref(), address.as_ref())?;

    Ok(response_json(data.to_json()))
}

async fn user_save(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if ALLOWED_IP != req.uri().host().unwrap() {
        return Err(BackendError::new("Operation not permitted.", 401));
    }
    check_token(&req)?;
    let json = get_body_json(HttpTransaction::Req(req)).await?;

    put_user(&User::from_json(&json)?)?;

    Ok(response_json(object! { ok: true }))
}

async fn get_groups(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if ALLOWED_IP != req.uri().host().unwrap() {
        return Err(BackendError::new("Operation not permitted.", 401));
    }
    check_token(&req)?;

    let default_group = get_default_group_name()?;

    if let Some(g) = default_group {
        Ok(response_json(object! {
            default_group: from_utf8(&g)?,
            groups: JsonValue::Array(get_all_groups_full()?.iter().map(|g| g.to_json()).collect())
        }))
    } else {
        Ok(response_json(object! { groups: JsonValue::Array(get_all_groups_full()?.iter().map(|g| g.to_json()).collect()) }))
    }
}

const PROPAGATE: u8 = 0;
const TARGET: u8 = 1;
const CACHE_READ: u8 = 2;
const CACHE_WRITE: u8 = 3;
const CACHE_DELETE: u8 = 4;

pub const REGULAR_MESSAGE: u8 = 0;
pub const CACHE_RESPONSE: u8 = 1;

async fn process_msg_bytes(
    mut b: tungstenite::Bytes,
    clients: Arc<Mutex<HashMap<Box<str>,
    UnboundedSender<Message>>>>,
    cache: Arc<Mutex<HashMap<i32, (Option<JoinHandle<()>>, CacheData)>>>,
    sender: UnboundedSender<Message>
) -> Result<(), BackendError> {
    let packet_type = b.get_u8();
    let source = read_prefixed_string(&mut b)?;

    match packet_type {
        PROPAGATE => {
            let safe = clients.lock().await;
            let source = source.into_boxed_str();
            for c in safe.iter() {
                let (key, client) = c;
                if *key != source {
                    client.send(encode_msg(&source, &mut b)?).map_err(|e| BackendError::new(&e.to_string(), 500))?;
                }
            }
        },
        TARGET => {
            let safe = clients.lock().await;
            let name = read_prefixed_string(&mut b)?.into_boxed_str();
            if let Some(client) = safe.get(&name) {
                client.send(encode_msg(&source, &mut b)?).map_err(|e| BackendError::new(&e.to_string(), 500))?;
            }
        },
        CACHE_READ => {
            let cache_id = b.get_i32();
            let channel = read_prefixed_string(&mut b)?.into_boxed_str();
            let map = cache.lock().await;
            let mut response: BytesMut;

            if let Some(cache) = map.get(&cache_id) && cache.1.channel == channel {
                response = BytesMut::with_capacity(6 + cache.1.payload.len());
                response.put_u8(CACHE_RESPONSE);
                response.put_i32(cache_id);
                response.put_u8(1); // true
                response.extend_from_slice(&cache.1.payload);
            } else {
                response = BytesMut::with_capacity(6);
                response.put_u8(CACHE_RESPONSE);
                response.put_i32(cache_id);
                response.put_u8(0); // false
            }

            sender.send(Message::Binary(response.freeze())).map_err(|e| BackendError::new(&e.to_string(), 500))?;
        },
        CACHE_WRITE => {
            let cache_id = b.get_i32();
            let expiration_millis = b.get_i64();
            let channel = read_prefixed_string(&mut b)?;
            let mut data = BytesMut::with_capacity(32);
            data.extend_from_slice(&b);

            let source = source.into_boxed_str();
            let source_cp = source.clone();

            if expiration_millis > 0 {
                let cache_cl = cache.clone();
                let handle = tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(expiration_millis as u64)).await;

                    let mut map = cache_cl.lock().await;
                    if let Some(entry) = map.get(&cache_id) && entry.1.owner == source_cp {
                        map.remove(&cache_id);
                    }
                });

                let mut map = cache.lock().await;
                map.insert(cache_id, (Some(handle), CacheData { payload: data.freeze(), owner: source, channel: channel.into_boxed_str() }));
            } else {
                let mut map = cache.lock().await;
                map.insert(cache_id, (None, CacheData { payload: data.freeze(), owner: source, channel: channel.into_boxed_str() }));
            }
        },
        CACHE_DELETE => {
            let cache_id = b.get_i32();
            let mut map = cache.lock().await;
            if let Some(entry) = map.get(&cache_id) && entry.1.owner == source.into_boxed_str() {
                if let Some(handle) = &entry.0 {
                    handle.abort();
                }
                map.remove(&cache_id);
            }
        },
        _ => {}
    };
    Ok(())
}

async fn create_ws(
    req: Request<Incoming>,
    clients: Arc<Mutex<HashMap<Box<str>,
    UnboundedSender<Message>>>>,
    cache: Arc<Mutex<HashMap<i32, (Option<JoinHandle<()>>, CacheData)>>>
) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if ALLOWED_IP != req.uri().host().unwrap() {
        return Err(BackendError::new("Operation not permitted.", 401));
    }
    check_token(&req)?;

    let mut query = get_body_url_args(&req)?;
    let name = query.remove("name".into());

    if !hyper_tungstenite::is_upgrade_request(&req) || req.version() != Version::HTTP_11 || name.is_none() {
        return Err(BackendError::new("Bad request", 400));
    }

    let name = name.unwrap();

    let (res, websocket) = hyper_tungstenite::upgrade(req, Some(WebSocketConfig::default()))?;
    let mut safe = clients.lock().await;
    let (tx, mut rx) = unbounded_channel();
    if safe.contains_key(&name.clone()) {
        return Err(BackendError::new("A client with that name already exists", 400));
    }
    safe.insert(name.clone(), tx.clone());

    drop(safe);

    tokio::spawn(async move {
        if let Ok(ws) = websocket.await {
            let (mut writer, mut reader) = ws.split();
            
            // Send messages safely
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if let Err(e) = writer.send(msg).await {
                        println!("Error sending websocket msg: {}", e.to_string());
                    }
                }
            });

            // Listen for messages
            while let Some(msg) = reader.next().await {
                match msg {
                    Ok(Message::Binary(b)) => {
                        if let Err(e) = process_msg_bytes(b, clients.clone(), cache.clone(), tx.clone()).await {
                            eprintln!("Failed to process websocket packet from {}: {}", name, e.get_msg());
                        }
                    },
                    Ok(Message::Close(_)) => {
                        drop(reader);
                        break;
                    },
                    Err(e) => {
                        eprintln!("A client ended a websocket abruptly: {}", e.to_string());
                        drop(reader);
                        break;
                    },
                    _ => {}
                }
            }

            // Cleanup
            let mut caches = cache.lock().await;
            caches.retain(|_, data| {
                let res = data.1.owner == name;
                if res && let Some(handle) = &data.0 {
                    handle.abort();
                }
                res
            });
            drop(caches);

            let mut safe = clients.lock().await;
            safe.remove(&name);
        }
    });

    Ok(res.map(|b| BoxBody::new(b)))
}

pub async fn register(router: &mut Router) -> Result<(), Box<dyn Error + Send + Sync>> {
    router.endpoint(Method::Get, "/api/privileged/player_data", player_data)?;
    router.endpoint(Method::Get, "/api/privileged/user_connected", user_connected)?;
    router.endpoint(Method::Post, "/api/privileged/punish", punish)?;
    router.endpoint(Method::Put, "/api/privileged/user_save", user_save)?;
    router.endpoint(Method::Put, "/api/privileged/get_groups", get_groups)?;

    let clients = Arc::new(Mutex::new(HashMap::new()));
    let bytes = Arc::new(Mutex::new(HashMap::new()));
    router.endpoint(Method::Get, "/api/privileged/create_ws", move |req| create_ws(req, clients.clone(), bytes.clone()))?;

    Ok(())
}
