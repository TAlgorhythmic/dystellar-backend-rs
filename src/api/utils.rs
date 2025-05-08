use std::error::Error;

use http_body_util::{BodyExt, Full};
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::{stringify, JsonValue};

pub fn response(obj: JsonValue) -> Response<Full<Bytes>> {
    Response::new(Full::new(Bytes::from(stringify(obj))))
}

pub async fn get_body(req: Request<Incoming>) -> Result<JsonValue, Box<dyn Error + Send + Sync>> {
    let body = req.into_body().collect().await?;
    let vec = body.to_bytes().to_vec();
    let str = String::from_utf8(vec)?;
    let json = json::parse(str.as_str())?;

    Ok(json)
}
