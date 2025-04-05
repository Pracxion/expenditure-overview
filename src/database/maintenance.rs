use sqlx::{SqlitePool,
           migrate::MigrateDatabase};

use crate::config::env_config::Config;

pub async fn initialize_database() {
    create_database_if_not_exists().await;
    apply_migrations().await;
}

pub async fn create_database_if_not_exists() {
    let database_url = &Config::default().database_url;
    if !sqlx::Sqlite::database_exists(database_url).await.expect("Failed to check if database exists") {
        sqlx::Sqlite::create_database(database_url).await.expect("Failed to create database")
    }
}

pub async fn apply_migrations() {
    let pool = connection_pool().await;
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to apply migrations")
}

pub async fn connection_pool() -> SqlitePool {
    SqlitePool::connect(&Config::default().database_url).await.expect("Failed to connect to database")
}
