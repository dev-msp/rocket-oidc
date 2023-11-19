use std::env;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SeaOrm error: {0}")]
    SeaOrm(#[from] sea_orm::error::DbErr),
}

pub async fn get_seaorm_pool() -> Result<sea_orm::DatabaseConnection, Error> {
    let db_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = sea_orm::Database::connect(&db_path).await?;
    Ok(db)
}
