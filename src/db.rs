use sqlx::sqlite::SqlitePool;
pub type Db = SqlitePool;

pub async fn init_db() -> Db {
    let database_url = "sqlite:data.db";

    SqlitePool::connect(database_url)
        .await
        .expect("Failed to connect to database")
}