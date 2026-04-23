mod db;
pub mod models;
pub mod services;
pub mod utils;

pub use db::get_db_path;
pub use models::*;
pub use services::*;

use log::info;

pub fn init() {
    let _ = &*db::DB;
    db::init_role_permissions();
    db::init_marketing_tables();
    info!("Database initialized successfully");
}
