use std::env;
use deadpool_diesel::postgres::{BuildError, Manager, Pool};
use deadpool_diesel::Runtime;
use dotenvy::dotenv;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PoolError {
    #[error(".env wont load")]
    Env(dotenvy::Error),

    #[error("bad DB url")]
    DBurl,

    #[error("pool could not build")]
    PoolBuildError(BuildError),

    #[error("connection establishment failed")]
    ConnError
}

pub fn init_pool () -> Result<Pool, PoolError> {
    dotenv().ok();
    let uri = env::var("POSTGRES_URI").map_err(|_| PoolError::DBurl)?;
    let manager = Manager::new(uri, Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();
    Ok(pool)
}