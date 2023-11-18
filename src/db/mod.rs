use sqlx::sqlite::SqlitePool;
use sqlx::{Pool, Sqlite};
use std::env;

pub async fn get_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(&db_path).await
}
