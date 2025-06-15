use dotenv::dotenv;
use std::env::var;

fn main() {
    dotenv().ok();

    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());

    let db_url: String;
    let host: String;
    let port: i32;
    let client_id: String;
    let redirect_uri: String;
    let client_secret: String;
    let privilege_token: String = var("PRIVILEGE_TOKEN").expect("Failed to get privilege token from env");

    if profile == "release" {
        db_url = var("PROD_DB_URL").expect("Failed to read production DB_URL.");
        host = var("PROD_HOST").expect("Failed to read production HOST.");
        port = i32::from_str_radix(var("PROD_PORT").expect("Failed to read production PORT.").as_str(), 10).expect("Failed to parse int.");
        client_id = var("PROD_CLIENT_ID").expect("Failed to get PROD_CLIENT_ID");
        redirect_uri = var("PROD_REDIRECT_URI").expect("Failed to get PROD_REDIRECT_URI");
        client_secret = var("PROD_CLIENT_SECRET").expect("Failed to read PROD_CLIENT_SECRET");
    } else {
        db_url = std::env::var("TEST_DB_URL").expect("Failed to read testing DB_URL.");
        host = var("TEST_HOST").expect("Failed to read production HOST.");
        port = i32::from_str_radix(var("TEST_PORT").expect("Failed to read production PORT.").as_str(), 10).expect("Failed to parse int.");
        client_id = var("TEST_CLIENT_ID").expect("Failed to get TEST_CLIENT_ID");
        redirect_uri = var("TEST_REDIRECT_URI").expect("Failed to get TEST_REDIRECT_URI");
        client_secret = var("TEST_CLIENT_SECRET").expect("Failed to read TEST_CLIENT_SECRET");
    }

    println!("cargo:rustc-env=DB_URL={}", db_url);
    println!("cargo:rustc-env=HOST={}", host);
    println!("cargo:rustc-env=PORT={}", port);
    println!("cargo:rustc-env=CLIENT_ID={}", client_id);
    println!("cargo:rustc-env=REDIRECT_URI={}", redirect_uri);
    println!("cargo:rustc-env=CLIENT_SECRET={}", client_secret);
    println!("cargo:rustc-env=PRIVILEGE_TOKEN={}", privilege_token);
}
