use crate::unwrap_or_return_result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Option<PgPool> {
    dotenv().ok();
    let database_url = unwrap_or_return_result!(env::var("DATABASE_URL"), "Database URL not set.");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Some(unwrap_or_return_result!(
        Pool::builder().build(manager),
        "Failed to build DB pool"
    ))
}

pub fn create_connection_singleton() -> Option<PgConnection> {
    let database_url = unwrap_or_return_result!(env::var("DATABASE_URL"), "Database URL not set.");
    Some(unwrap_or_return_result!(
        PgConnection::establish(&database_url),
        "Error connecting to database!"
    ))
}