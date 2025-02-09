// src/db.rs
use mysql::*;
use std::env;

pub fn create_pool() -> Result<Pool> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    Pool::new(database_url.as_str())
}

pub fn get_conn(pool: &Pool) -> Result<PooledConn> {
    pool.get_conn()
}