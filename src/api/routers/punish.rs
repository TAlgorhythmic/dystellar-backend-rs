use crate::api::{routers::ROUTER, typedef::routing::Method};

pub async fn register() {
    let mut router = ROUTER.lock().await;

    router.endpoint(Method::Post,
        "/api/punish/ban",
        Box::new(|req| {Box::pin(player_data(req))})
    ).expect("Failed to register status endpoint");
}
