use crate::{DBCon, DBPool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::{Config, Error, NoTls};
use std::str::FromStr;
use std::time::Duration;
use std::env;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;


// pub async fn initialize_postgres() {
//
// }


pub async fn pool() -> std::result::Result<DBPool, mobc::Error<Error>> {

    let _pg_user = env::var("PG_USERNAME").unwrap();
    let _pg_pass = env::var("PG_PASSWORD").unwrap();
    let _pg_db_name = env::var("PG_DATABASE").unwrap();

    let _pg_conn_str = format!("postgres://{username}:{password}@127.0.0.1:5432/{database}",
                               username=_pg_user,
                               password=_pg_pass,
                               database=_pg_db_name);

    let config = Config::from_str(&_pg_conn_str)?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(DBPool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

pub async fn get_connection_from_pool(db_pool: &DBPool) -> Result<DBCon,mobc::Error<Error>> {
    db_pool.get().await
}