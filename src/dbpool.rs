use sqlx::postgres::{
    PgPoolOptions,
    PgPool
};
use std::sync::{Arc, Mutex};
use sqlx::{
    pool::{PoolConnection},
    postgres::{Postgres},
};

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
    pub fn new(connection_str:&str, max_conn:u32) -> Self {
        ConnectOptions{
            connect_str : connection_str.to_string(),
            max_connections: max_conn
        }
    }
}

pub struct DBPool{}

impl DBPool{

    pub fn connect_options(options:Option<ConnectOptions>) -> Option<ConnectOptions>{
        static mut OPTIONS:Option<ConnectOptions> = None;

        unsafe{
            if options.is_none() && OPTIONS.is_some(){
                return OPTIONS.clone();
            }
            if OPTIONS.is_none() && options.is_some(){
                OPTIONS = options;
            }
            return None;
        }
    }

    pub async fn acquire() -> Result<PoolConnection<Postgres>,ModError> {
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
                if let Ok(ref mut pcm) = amp.lock() {
                    let conn = pcm.acquire().await;
                    match conn {
                        Ok(_conn) => { return Ok(_conn); },
                        Err(_error) => { return Err(ModError::DB(_error)); },
                    }
                } else {
                    return Err(ModError::Connection("Unable to get a lock on pool connection".to_string()));
                }
            } else {
                return Err(ModError::Connection("No Pool Connection instantiated".to_string()));
            }
        }
    }
}
