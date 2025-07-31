use std::sync::Arc;

use tokio::sync::Mutex;

use crate::api::{typedef::{fs_json::redirects::Redirects, BackendError, Router}, utils::temporary_redirection};

pub async fn register(rout: &Arc<Mutex<Router>>, redirections: Arc<std::sync::Mutex<Redirects>>) {
    let mut router = rout.lock().await;
    let redirects = redirections.lock().unwrap();
    
    for (key, value) in &redirects.mappings {
        let cl = value.clone();

        let _ = router.endpoint(
            crate::api::typedef::Method::Get,
            format!("/{key}").as_str(),
            Box::new(move |_| {
                Box::pin({
                    let url = cl.clone();
                    async move {
                        Ok(temporary_redirection(&url))
                    }
                })
            })
        );
    }
}
