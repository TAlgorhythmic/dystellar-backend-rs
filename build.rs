use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());

    let db_url: String;

    if profile == "release" {
        db_url = std::env::var("PROD_DB_URL").expect("Failed to read production DB_URL.");
    } else {
        db_url = std::env::var("TEST_DB_URL").expect("Failed to read testing DB_URL.");
    }

    println!("cargo:rustc-env=DB_URL={}", db_url);
}
