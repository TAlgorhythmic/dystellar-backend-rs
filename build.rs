use dotenv::dotenv;
use std::env::var;

fn main() {
    dotenv().ok();

    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());

    let db_url: String;
    let host: String;
    let port: i32;

    if profile == "release" {
        db_url = var("PROD_DB_URL").expect("Failed to read production DB_URL.");
        host = var("PROD_HOST").expect("Failed to read production HOST.");
        port = i32::from_str_radix(var("PROD_PORT").expect("Failed to read production PORT.").as_str(), 10).expect("Failed to parse int.");
    } else {
        db_url = std::env::var("TEST_DB_URL").expect("Failed to read testing DB_URL.");
        host = var("TEST_HOST").expect("Failed to read production HOST.");
        port = i32::from_str_radix(var("TEST_PORT").expect("Failed to read production PORT.").as_str(), 10).expect("Failed to parse int.");

    }

    println!("cargo:rustc-env=DB_URL={}", db_url);
    println!("cargo:rustc-env=HOST={}", host);
    println!("cargo:rustc-env=PORT={}", port);
}
