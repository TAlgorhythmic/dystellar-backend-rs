use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());

    let db_url: String;
    let db_user: String;
    let db_password: String;

    if profile == "release" {
        db_url = std::env::var("PROD_DB_URL").expect("Failed to read production DB_URL.");
        db_user = std::env::var("PROD_DB_USER").expect("Failed to read production DB_USER.");
        db_password = std::env::var("PROD_DB_USER").expect("Failed to read production DB_PASSWORD.");
    } else {
        db_url = std::env::var("TEST_DB_URL").expect("Failed to read testing DB_URL.");
        db_user = std::env::var("TEST_DB_USER").expect("Failed to read testing DB_USER.");
        db_password = std::env::var("TEST_DB_USER").expect("Failed to read testing DB_PASSWORD.");
    }

    println!("cargo:rustc-env=DB_URL={}", db_url);
    println!("cargo:rustc-env=DB_USER={}", db_user);
    println!("cargo:rustc-env=DB_PASSWORD={}", db_password);
}
