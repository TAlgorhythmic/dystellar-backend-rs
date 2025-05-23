use std::{collections::HashMap, error::Error};

use http_body_util::{BodyExt, Full};
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::{stringify, JsonValue};

pub enum HttpTransaction {
    Req(Request<Incoming>),
    Res(Response<Incoming>)
}

pub fn response(obj: JsonValue) -> Response<Full<Bytes>> {
    Response::new(Full::new(Bytes::from(stringify(obj))))
}

pub async fn get_body_str(http: HttpTransaction) -> Result<String, Box<dyn Error + Send + Sync>> {
    let body = match http {
        HttpTransaction::Req(req) => req.into_body().collect().await?,
        HttpTransaction::Res(res) => res.into_body().collect().await?
    };

    let vec = body.to_bytes().to_vec();
    let str = String::from_utf8(vec)?;

    Ok(str)
}

pub async fn get_body_body_args(http: HttpTransaction) -> Result<HashMap<Box<str>, Box<str>>, Box<dyn Error + Send + Sync>> {
    let mut map: HashMap<Box<str>, Box<str>> = HashMap::new();

    let body = get_body_str(http).await?;
    println!("{}", body);
    let pairs = body.split('&');
    for pair in pairs {
        let split: Vec<&str> = pair.split('=').collect();
        if split.len() != 2 {
            return Err("Failed to parse url parameters (malformed url)".into());
        }
        map.insert(split[0].into(), split[1].replace('+', " ").into());
    }
    Ok(map)
}

pub async fn get_body_url_args(req: &Request<Incoming>) -> Result<HashMap<Box<str>, Box<str>>, Box<dyn Error + Send + Sync>> {
    let mut map: HashMap<Box<str>, Box<str>> = HashMap::new();

    let bodyopt = req.uri().query();
    if bodyopt.is_none() {
        return Err("No query found".into());
    }

    let body = bodyopt.unwrap();

    let pairs = body.split('&');
    for pair in pairs {
        let split: Vec<&str> = pair.split('=').collect();
        if split.len() != 2 {
            return Err("Failed to parse url parameters (malformed url)".into());
        }
        map.insert(split[0].into(), split[1].replace('+', " ").into());
    }
    Ok(map)
}

pub async fn get_body_json(http: HttpTransaction) -> Result<JsonValue, Box<dyn Error + Send + Sync>> {
    let json = json::parse(get_body_str(http).await?.as_str())?;

    Ok(json)
}
