mod db;
mod models;
mod services;
mod utils;

pub use db::get_db_path;
pub use models::*;
pub use services::*;

use log::info;

pub fn init() {
    let _ = &*db::DB;
    db::init_role_permissions();
    info!("Database initialized successfully");
}
