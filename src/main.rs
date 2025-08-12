mod api;

use crate::api::{control::storage::setup::init_db, routers::{redirections, repository}};
use api::routers::{microsoft, signal, state, users};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use hyper_util::rt::TokioIo;
use api::service::srv;
use hyper::service::service_fn;

use crate::api::routers::privileged;

pub static HOST: &str = env!("HOST");
pub static PORT: &str = env!("PORT");

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

#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Init Database
    init_db().await.expect("Failed to initialize database");

    // Register endpoints
    microsoft::register().await;
    signal::register().await;
    privileged::register().await;
    users::register().await;
    state::register().await?;
    redirections::register()?;
    repository::register().await?;

    let address: SocketAddr = (HOST.to_owned() + ":" + PORT).parse().expect("Error parsing ip and port");
    let binding = TcpListener::bind(address).await?;

    println!("Listening to {HOST}:{PORT}");

    loop {
        let (stream, addr) = binding.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| srv(req, addr));
            let res = hyper_util::server::conn::auto::Builder::new(Exec).serve_connection(io, service).await;

            if res.is_err() {
                eprintln!("Error serving connection: {}", res.err().unwrap().to_string());
            }
        });
    }
}
