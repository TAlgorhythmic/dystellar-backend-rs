use std::sync::Arc;

use crate::api::{routers::ROUTER, typedef::{fs_json::redirects::Redirects}, utils::temporary_redirection};

pub async fn register(redirections: Arc<std::sync::Mutex<Redirects>>) {
    let mut router = ROUTER.lock().await;
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
