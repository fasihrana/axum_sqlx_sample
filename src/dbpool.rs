use sqlx::postgres::{
    PgPoolOptions,
    PgPool
};
use std::sync::{Arc, Mutex};
use sqlx::{
    pool::{PoolConnection},
    postgres::{Postgres},
};
use once_cell::sync::Lazy;

#[derive(Debug)]
pub enum ModError{
    DB(sqlx::Error),
    Connection(String),
}

#[derive(Clone)]
pub struct ConnectOptions{
    connect_str: String,
    max_connections: u32,
}

impl ConnectOptions {
    pub fn new(connect_str: String, max_connections: u32) -> Self {
        ConnectOptions{
            connect_str,
            max_connections
        }
    }
}

#[derive(Clone)]
pub struct DBPool{
    pool: PgPool
}

impl DBPool{

    pub async fn from_options(options: &ConnectOptions) -> Result<Self,ModError> {
        match PgPoolOptions::new()
            .max_connections(options.max_connections)
            .connect(options.connect_str.as_str())
            .await
        {
            Ok(pool) => {
                println!("Successfully connected to target PostgreSQL server!");
                Ok(DBPool{pool})
            }
            Err(err) => {
                Err(ModError::DB(err))
            }
        }
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }

    /*pub async fn acquire() -> Result<PoolConnection<Postgres>,ModError> {

        static mut POOL : Option<Arc<Mutex<PgPool>>> = None;
        unsafe{
            if POOL.is_none() {
                let options = Self::connect_options(None);
                match options {
                    None => { return Err(ModError::Connection("No ConnectOptions provided for connection".to_string())); },
                    Some(_options) => {
                        let poolres = PgPoolOptions::new()
                            .max_connections(_options.max_connections)
                            .connect(_options.connect_str.as_str())
                            .await;
                        match poolres {
                            Err(_error) => { return Err(ModError::DB(_error)); },
                            Ok(_pool) => {
                                POOL = Some(Arc::new(Mutex::new(_pool)));
                            }
                        }
                    }
                }
            }

            let mut oamp = POOL.clone();
            if let Some(ref mut amp) = oamp {
                let pcm = if let Ok(ref mut pcm) = amp.lock(){
                    pcm.clone()
                } else {
                    return Err(ModError::Connection(
                        "Unable to get a lock on pool connection".to_string(),
                    ));
                };

                let conn = pcm.acquire().await;
                match conn {
                    Ok(_conn) => {
                        return Ok(_conn);
                    },
                    Err(_error) => {
                        return Err(ModError::DB(_error));
                    },
                }
            } else {
                return Err(ModError::Connection(
                    "No Pool Connection instantiated".to_string(),
                ));
            }
        }
    }*/
}
