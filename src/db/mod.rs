use sqlx::Row;
use std::error::Error;
use tracing::info;

pub mod bookstore;

pub const DB_FOR_DEV: &str = "postgres://postgres:postgres@localhost:5432/myapp";

pub async fn test(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let res = sqlx::query("SELECT 1 + 1 as sum").fetch_one(pool).await?;

    let sum: i32 = res.get("sum");
    info!("1 + 1 = {sum}");

    Ok(())
}

/// Example show how to delete all current tables and run migrations
/// cargo run -- sqlx migrate --folder bookstore
pub async fn migrate_bookstore(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let _ = delete_all_tables(&pool).await.unwrap();
    sqlx::migrate!("migrations/bookstore").run(pool).await?;

    Ok(())
}

async fn delete_all_tables(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    // sqlx::PgPool already specify the db
    let sql = r#"
        DO $$ DECLARE
            r RECORD;
        BEGIN
            -- Iterate over all tables in the public schema
            FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
                -- Drop each table
                EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
            END LOOP;
        END $$;
    "#;

    sqlx::query(&sql).execute(pool).await?;

    Ok(())
}
