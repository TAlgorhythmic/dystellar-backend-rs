mod api;

use api::{service::srv_api, control::{inotify::DirWatcher, storage::setup::init_db}};
use api::routers::{microsoft, signal, state, users, redirections, stream, privileged};
use std::{net::SocketAddr, sync::Arc, thread};
use tokio::{net::TcpListener, runtime::Builder, sync::Mutex};
use hyper_util::rt::TokioIo;
use hyper::service::service_fn;

use crate::api::{routers::mods, typedef::routing::nodes::Router};

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

    let mut router = Router::new();
    let mut watcher = DirWatcher::create(".")?;

    // Register endpoints
    microsoft::register(&mut router).await?;
    signal::register(&mut router).await?;
    privileged::register(&mut router).await?;
    users::register(&mut router).await?;
    state::register(&mut router, &mut watcher).await?;
    stream::register(&mut router).await?;
    mods::register(&mut router).await?;

    let router = Arc::new(Mutex::new(router));

    redirections::register(&mut watcher, router.clone())?;
    watcher.listen();
    // Listen for config file changes

    let address: SocketAddr = (HOST.to_owned() + ":" + PORT).parse().expect("Error parsing ip and port");
    let binding = TcpListener::bind(address).await?;

    println!("Listening to {HOST}:{PORT}");

    loop {
        let (stream, addr) = binding.accept().await?;

        let io = TokioIo::new(stream);
        let router = router.clone();

        tokio::task::spawn(async move {
            let service_api = service_fn(move |req| {
                let router = router.clone();
                srv_api(req, addr, router)
            });
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
