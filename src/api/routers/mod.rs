pub mod microsoft;

use std::{collections::HashMap, error::Error, sync::Arc};
use hyper::{Response, Request, body::{Bytes, Incoming}};
use http_body_util::Full;
use tokio::sync::Mutex;

use super::typedef::Router;

pub async fn handle(req: Request<Incoming>, router: Arc<Mutex<Router>>) -> Result<Response<Full<Bytes>>, Box<dyn Error + Send + Sync>> {
    let mut map: HashMap<Box<str>, Box<str>> = HashMap::new();
    let split: Vec<&str> = req.uri().path().split('?').collect();
    
    if split.len() > 1 {
        let args = split[1].split('&').into_iter();
        for s in args {
            let keyv: Vec<&str> = s.split('=').collect();
            if keyv.len() < 2 {
                return Err("Malformed url".into());
            }

            map.insert(keyv[0].into(), keyv[1].into());
        }
    }
    
    if let Some(endpoint) = router.lock().await.get_endpoint(split[0], req.method().as_str().into()) {
        let fut = endpoint.get_handler()(req, map);
        return fut.await;
    }

    Err("Path not found".into())
}
