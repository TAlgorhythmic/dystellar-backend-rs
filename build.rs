use dotenv::dotenv;
use std::env::var;

fn main() {
    dotenv().ok();

    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());

    let host: String;
    let port: i32;
    let client_id: String;
    let redirect_uri: String;
    let client_secret: String;
    let privilege_token: String = var("PRIVILEGE_TOKEN").expect("Failed to get privilege token from env");
    let authorized_ip: String;

    if profile == "release" {
        host = var("PROD_HOST").expect("Failed to read production HOST.");
        port = i32::from_str_radix(var("PROD_PORT").expect("Failed to read production PORT.").as_str(), 10).expect("Failed to parse int.");
        client_id = var("PROD_CLIENT_ID").expect("Failed to get PROD_CLIENT_ID");
        redirect_uri = var("PROD_REDIRECT_URI").expect("Failed to get PROD_REDIRECT_URI");
        client_secret = var("PROD_CLIENT_SECRET").expect("Failed to read PROD_CLIENT_SECRET");
        authorized_ip = var("PROD_PRIVILEGED_AUTHORIZED_IP").expect("Failed to get privileged authrized ips env");
    } else {
        host = var("TEST_HOST").expect("Failed to read production HOST.");
        port = i32::from_str_radix(var("TEST_PORT").expect("Failed to read production PORT.").as_str(), 10).expect("Failed to parse int.");
        client_id = var("TEST_CLIENT_ID").expect("Failed to get TEST_CLIENT_ID");
        redirect_uri = var("TEST_REDIRECT_URI").expect("Failed to get TEST_REDIRECT_URI");
        client_secret = var("TEST_CLIENT_SECRET").expect("Failed to read TEST_CLIENT_SECRET");
        authorized_ip = var("TEST_PRIVILEGED_AUTHORIZED_IP").expect("Failed to get privileged authrized ips env");
    }

    println!("cargo:rustc-env=HOST={}", host);
    println!("cargo:rustc-env=PORT={}", port);
    println!("cargo:rustc-env=CLIENT_ID={}", client_id);
    println!("cargo:rustc-env=REDIRECT_URI={}", redirect_uri);
    println!("cargo:rustc-env=CLIENT_SECRET={}", client_secret);
    println!("cargo:rustc-env=PRIVILEGE_TOKEN={}", privilege_token);
    println!("cargo:rustc-env=PRIVILEGED_AUTHORIZED_IP={}", authorized_ip);
}
