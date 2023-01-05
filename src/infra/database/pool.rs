use sqlx::pool::Pool;
use sqlx::mysql::{MySqlPoolOptions, MySql};
use std::{collections::HashMap};
use once_cell::sync::OnceCell;
use config::Value;
use crate::util::error;

use crate::util::error::{Error, Result};
pub type DbPool = Pool<MySql>;

static DB_POOL: OnceCell<DbPool> = OnceCell::new();

pub async fn create_pool(config: &HashMap<String, Value>) -> Result<()> {
    let max_connections:u32 = config.get("max_connection").expect("max connection should configured").to_string().parse()?;
    if max_connections == 0 {
        return Err(Error::ConfigError(format!("max connection for database is incorrect {}", max_connections)))
    }
    let db_connection = config.get("connection_url").expect("database connection url should configured").to_string();
    if db_connection.is_empty() {
        return Err(Error::ConfigError(format!("database connection url is incorrect {}", db_connection)))
    }
    let pool = MySqlPoolOptions::new()
        .max_connections(max_connections)
        .connect(db_connection.as_str())
        .await
        .map_err(Error::from)?;
    DB_POOL.set(pool).expect("db pool configured");
    ping().await?;
    Ok(())
}

pub fn get_db_pool() -> Result<DbPool> {
    return match DB_POOL.get() {
        None => {
            Err(error::Error::DatabaseError("failed to get database pool".to_string()))
        }
        Some(pool) => {
            Ok(pool.clone())
        }
    }
}

pub async fn ping() -> Result<()> {
    info!("Checking on database connection...");
    let pool = get_db_pool();
    match pool {
        Ok(pool) => {
            sqlx::query("SELECT 1")
                .fetch_one(&pool)
                .await
                .expect("Failed to PING database");
            info!("Database PING executed successfully!");
        }
        Err(e) => {
            return Err(Error::DatabaseError(e.to_string()))
        }
    }
    Ok(())
}
