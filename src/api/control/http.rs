use std::error::Error;

use http_body_util::Full;
use hyper::{body::{Bytes, Incoming}, client::conn::http2::handshake, header::{HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, HOST}, Request, Response, Uri};
use hyper_util::rt::TokioIo;
use json::{stringify, JsonValue};
use tokio::net::TcpStream;

use crate::Exec;

fn empty() -> Full<Bytes> {
    Full::new(Bytes::new())
}

async fn request(uri: Uri, req: Request<Full<Bytes>>) -> Result<Response<Incoming>, Box<dyn Error + Send + Sync>> {
    let hostopt = uri.host();
    if hostopt.is_none() {
        return Err("Invalid url (backend side error)".into());
    }

    let host = hostopt.unwrap();
    let port = uri.port_u16().unwrap_or(443);

    let addr = format!("{host}:{port}");
    let stream = TcpStream::connect(addr).await?;

    let io = TokioIo::new(stream);

    let (mut sender, _) = handshake(Exec, io).await?;

    let res = sender.send_request(req).await?;
    Ok(res)
}

pub async fn post_urlencoded(url: &str, body_params: String) -> Result<Response<Incoming>, Box<dyn Error + Send + Sync>> {
    let uri: Uri = url.parse()?;
    let authority = uri.authority().unwrap().clone();
    
    let req = Request::builder()
        .method("POST")
        .uri(&uri)
        .header(HOST, authority.as_str())
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Full::new(Bytes::from(body_params)))?;

    request(uri, req).await
}

pub async fn post_json(url: &str, body: JsonValue) -> Result<Response<Incoming>, Box<dyn Error + Send + Sync>> {
    let uri: Uri = url.parse()?;
    let authority = uri.authority().unwrap();

    let req = Request::builder()
        .method("POST")
        .uri(&uri)
        .header(HOST, authority.as_str())
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .body(Full::new(Bytes::from(stringify(body))))?;

    request(uri, req).await
}

/**
* Issue a get request with the possibility for custom headers
*/
pub async fn get_json(url: &str, add_headers: Option<&[(HeaderName, HeaderValue)]>) -> Result<Response<Incoming>, Box<dyn Error + Send + Sync>> {
    let uri: Uri = url.parse()?;
    let authority = uri.authority().unwrap();

    let mut req_build = Request::builder()
        .method("GET")
        .uri(&uri)
        .header(HOST, authority.as_str())
        .header(CONTENT_TYPE, "application/json");

    // Add additional headers
    if let Some(headers) = add_headers {
        for header in headers {
            let (key, value) = header;

            req_build = req_build.header(key, value);
        }
    }

    let req = req_build.body(empty())?;

    request(uri, req).await
}
