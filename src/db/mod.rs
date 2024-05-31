use sqlx::Row;
use std::error::Error;
use tracing::info;

const DB_FOR_DEV: &str = "postgres://postgres:postgres@localhost:5432/myapp";

pub async fn test() -> Result<(), Box<dyn Error>> {
    // let mut connection = sqlx::postgres::PgConnection::connect(DB_FOR_DEV).await?;
    let pool = sqlx::postgres::PgPool::connect(DB_FOR_DEV).await?;
    let res = sqlx::query("SELECT 1 + 1 as sum").fetch_one(&pool).await?;

    let sum: i32 = res.get("sum");
    info!("1 + 1 = {sum}");

    Ok(())
}
