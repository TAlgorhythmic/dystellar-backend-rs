use std::collections::HashMap;

use http_body_util::{BodyExt, Full};
use hyper::{body::{Bytes, Incoming}, header::{CONTENT_TYPE, LOCATION}, Request, Response};
use json::{stringify, JsonValue};

use crate::api::control::http::empty;

use super::typedef::BackendError;

pub enum HttpTransaction {
    Req(Request<Incoming>),
    Res(Response<Incoming>)
}

pub fn response_json(obj: JsonValue) -> Response<Full<Bytes>> {
    response_status_json(obj, 200)
}

pub fn response_status_json(obj: JsonValue, status: u16) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from(stringify(obj))))
        .unwrap()
}

pub async fn get_body_str(http: HttpTransaction) -> Result<String, BackendError> {
    let body_res = match http {
        HttpTransaction::Req(req) => req.into_body().collect().await,
        HttpTransaction::Res(res) => res.into_body().collect().await
    };

    if body_res.is_err() {
        return Err(BackendError::new("Failed to decode body", 500));
    }

    let body = body_res.unwrap();

    let vec = body.to_bytes().to_vec();
    let str_opt = String::from_utf8(vec);

    if let Err(err) = &str_opt {
        return Err(BackendError::new(err.to_string().as_str(), 500));
    }

    let str = str_opt.unwrap();
    Ok(str)
}

pub async fn get_body_url_args(req: &Request<Incoming>) -> Result<HashMap<Box<str>, Box<str>>, BackendError> {
    let mut map: HashMap<Box<str>, Box<str>> = HashMap::new();

    let bodyopt = req.uri().query();
    if bodyopt.is_none() {
        return Err(BackendError::new("No query found", 400));
    }

    let body = bodyopt.unwrap();

    let pairs = body.split('&');
    for pair in pairs {
        let split: Vec<&str> = pair.split('=').collect();
        if split.len() != 2 {
            return Err(BackendError::new("Failed to parse url query (malformed url)", 400));
        }
        map.insert(split[0].into(), split[1].replace('+', " ").into());
    }
    Ok(map)
}

pub async fn get_body_json(http: HttpTransaction) -> Result<JsonValue, BackendError> {
    let body_str = get_body_str(http).await?;
    let json = json::parse(body_str.as_str());
    if let Err(err) = &json  {
        return Err(BackendError::new(format!("Malformed body, couldn't decode json: {}", err.to_string()).as_str(), 400));
    }

    Ok(json.unwrap())
}

pub fn temporary_redirection(url: &str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(302)
        .header(LOCATION, url)
        .body(empty())
        .unwrap()
}
