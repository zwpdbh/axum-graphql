use async_graphql::{Context, FieldResult, Object, Schema, SimpleObject};
use chrono;
// use sqlx::types::chrono;
use sqlx::PgPool;

#[derive(SimpleObject)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub inserted_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(SimpleObject)]
pub struct MenuItem {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub added_on: chrono::NaiveDate,
    pub category_id: i32,
    pub inserted_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
