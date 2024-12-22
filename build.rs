use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());

    let db_url;
    let db;
}
