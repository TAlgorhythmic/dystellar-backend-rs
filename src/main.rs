mod api;

use crate::api::{control::storage::setup::init_db, routers::users};
use api::{routers::{microsoft, signal}, typedef::Router};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use hyper_util::rt::TokioIo;
use api::service::srv;
use hyper::service::service_fn;

use crate::api::routers::privileged;

pub static HOST: &str = env!("HOST");
pub static PORT: &str = env!("PORT");

// Executor start
#[derive(Clone)]
struct Exec;

impl<F> hyper::rt::Executor<F> for Exec
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static
{
    fn execute(&self, fut: F) {
        tokio::task::spawn(fut);
    }
}
// Executor end

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Init Database
    init_db().await.expect("Failed to initialize database");

    let router = Arc::new(Mutex::new(Router::new("api")));

    // Register endpoints
    microsoft::register(&router).await;
    signal::register(&router).await;
    privileged::register(&router).await;
    users::register(&router);

    let address: SocketAddr = (HOST.to_owned() + ":" + PORT).parse().expect("Error parsing ip and port");
    let binding = TcpListener::bind(address).await?;

    println!("Listening to {HOST}:{PORT}");

    loop {
        let (stream, _) = binding.accept().await?;

        let io = TokioIo::new(stream);
        
        let cl = router.clone();
        tokio::task::spawn(async move {
            let service = service_fn(move |req| srv(req, cl.clone()));
            let res = hyper_util::server::conn::auto::Builder::new(Exec).serve_connection(io, service).await;

            if res.is_err() {
                eprintln!("Error serving connection: {}", res.err().unwrap().to_string());
            }
        });
    }
}
