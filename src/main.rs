mod api;

use api::control::sql::setup::init_db;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use hyper_util::rt::TokioIo;
use api::service::srv;
use hyper::server::conn::http2;
use hyper::service::service_fn;

const HOST: &str = env!("HOST");
const PORT: &str = env!("PORT");

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

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Init Database
    init_db().await.expect("Failed to initialize database");

    let address: SocketAddr = (HOST.to_owned() + ":" + PORT).parse().expect("Error parsing ip and port");
    let binding = TcpListener::bind(address).await?;

    println!("Listening to {HOST}:{PORT}");

    loop {
        let (stream, addr) = binding.accept().await?;

        println!("Incoming connection from {addr}.");

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let res = http2::Builder::new(Exec).serve_connection(io, service_fn(srv)).await;

            if res.is_err() {
                eprintln!("Error serving connection: {}", res.err().unwrap().to_string());
            }
        });
    }
}
