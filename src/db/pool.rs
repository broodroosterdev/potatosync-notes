use diesel::r2d2::{Pool, ConnectionManager};
use diesel::{PgConnection, Connection};
use std::env;

/// An alias to the type for a pool of Diesel Postgres Connection
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Initialize the database pool.
pub fn connect() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("Failed to create pool")
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("Could not find DATABASE URL in .env")
}

pub fn pg_connection() -> PgConnection {
    PgConnection::establish(database_url().as_str()).unwrap()
}