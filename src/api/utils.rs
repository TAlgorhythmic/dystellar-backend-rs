use http_body_util::Full;
use hyper::{body::Bytes, Response};
use json::{stringify, JsonValue};

pub fn response(obj: JsonValue) -> Response<Full<Bytes>> {
    Response::new(Full::new(Bytes::from(stringify(obj))))
}
