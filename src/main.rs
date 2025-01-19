mod api;
use api::control::sql::setup::init_db;
use std::net::{SocketAddr, IpAddr};
use tokio::net::TcpListener;
use hyper_util::rt::TokioIo;
use api::service::srv;

const HOST: &str = env!("HOST");
const PORT: &str = env!("PORT");

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Init Database
    init_db().await.expect("Failed to initialize database");
    
    let address = SocketAddr::new(HOST.parse::<IpAddr>().expect("Failed to parse host."), u16::from_str_radix(PORT, 10).expect("Failed to parse port."));
    let binding = TcpListener::bind(address).await?;

    loop {
        let (stream, addr) = binding.accept().await?;
        
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            
        });
    }
}
