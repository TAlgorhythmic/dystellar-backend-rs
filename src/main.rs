mod api;

use api::{service::srv_api, control::{inotify::DirWatcher, storage::setup::init_db}};
use api::routers::{microsoft, signal, state, users, redirections, stream, privileged};
use std::{net::SocketAddr, thread};
use tokio::{net::TcpListener, runtime::Builder};
use hyper_util::rt::TokioIo;
use hyper::service::service_fn;

use crate::api::routers::mods;

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

async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Init Database
    init_db().await.expect("Failed to initialize database");

    let mut watcher = DirWatcher::create(".")?;

    // Register endpoints
    microsoft::register().await;
    signal::register().await;
    privileged::register().await;
    users::register().await;
    state::register(&mut watcher).await?;
    redirections::register(&mut watcher)?;
    stream::register().await?;
    mods::register().await?;

    // Listen for config file changes
    watcher.listen();

    let address: SocketAddr = (HOST.to_owned() + ":" + PORT).parse().expect("Error parsing ip and port");
    let binding = TcpListener::bind(address).await?;

    println!("Listening to {HOST}:{PORT}");

    loop {
        let (stream, addr) = binding.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let service_api = service_fn(move |req| srv_api(req, addr));
            let res = hyper_util::server::conn::auto::Builder::new(Exec).serve_connection(io, service_api).await;

            if res.is_err() {
                eprintln!("Error serving connection: {}", res.err().unwrap().to_string());
            }
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cores = thread::available_parallelism()?.get();

    let runtime = Builder::new_multi_thread()
        .worker_threads(cores)
        .enable_all()
        .build()?;

    runtime.block_on(async {
        run().await
    })?;

    Ok(())
}
