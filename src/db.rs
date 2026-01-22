use sqlx::{sqlite::SqlitePool, migrate::MigrateDatabase};
pub type Db = SqlitePool;

pub async fn init_db() -> Db {
    let database_url = "sqlite:data.db";

    if !sqlx::Sqlite::database_exists(database_url).await.unwrap_or(false) {
        sqlx::Sqlite::create_database(database_url).await.expect("Failed to create database file");
        println!("Database file created");
    }

    let pool = SqlitePool::connect(database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    println!("Migrations applied successfully");
    pool
}