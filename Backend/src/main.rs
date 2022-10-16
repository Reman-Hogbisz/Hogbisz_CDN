#[macro_use]
extern crate rocket;

extern crate openssl;
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

embed_migrations!("migrations");

pub mod auth;
pub mod db_connection;
pub mod file;
pub mod model;
pub mod schema;
pub mod util;

use dotenv::dotenv;
use std::sync::Arc;

pub struct ManagedPool(pub Arc<db_connection::PgPool>);

#[rocket::main]
async fn main() {
    println!("Starting server");
    match dotenv() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to load .env file: {}", e);
        }
    };

    println!("Initializing SSL certificate environment variables");
    openssl_probe::init_ssl_cert_env_vars();

    {
        println!("Initializing temporary connection to database");
        let connection =
            db_connection::create_connection_singleton().expect("Failed to connect to database");
        println!("Running migration");
        embedded_migrations::run(&connection).expect("Failed to run embedded migrations");
    }

    println!("Initializing database connection pool");
    let pool = Arc::new(db_connection::init_pool().expect("Failed to initialize DB pool"));

    let save_path = crate::util::UPLOAD_PATH.as_str();

    if !std::path::Path::new(save_path).exists() {
        println!("Creating upload directory");
        match std::fs::create_dir_all(save_path) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to create upload directory: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("Starting rocket backend");
    match rocket::build()
        .manage(ManagedPool(pool))
        .mount(
            "/api",
            routes![file::upload_file, file::get_file, auth::login],
        )
        .launch()
        .await
    {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to launch server with error: {}", e);
            std::process::exit(1);
        }
    }
}
