mod api;
use api::control::sql::setup::init_db;


#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Init Database
    init_db().await.expect("Failed to initialize database");

}
