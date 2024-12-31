mod api;
use api::control::sql::setup::init_db;

fn main() {
    // Init Database
    init_db().expect("Failed to initialize database");
}
