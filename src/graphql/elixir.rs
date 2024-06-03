use crate::graphql::format_datetime;
use async_graphql::Object;
use bigdecimal::ToPrimitive;
use sqlx::postgres::PgRow;
use sqlx::types::BigDecimal;
use sqlx::Row;

use super::format_date;

// {
//   categories{
//     id,
//     description,
//     insertedAt,
//     updatedAt
// 	}
// }
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub inserted_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[Object]
impl Category {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn description(&self) -> Option<String> {
        self.description.clone()
    }

    async fn inserted_at(&self) -> String {
        format_datetime(&self.inserted_at)
    }

    async fn updated_at(&self) -> String {
        format_datetime(&self.updated_at)
    }
}

impl<'r> sqlx::FromRow<'r, PgRow> for Category {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description").unwrap_or(None),
            inserted_at: row.try_get("inserted_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
// {
//   menuItems{
//     name,
//     description,
//     updatedAt
//   }
// }
pub struct MenuItem {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub added_on: chrono::NaiveDate,
    pub category_id: i32,
    pub inserted_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl<'r> sqlx::FromRow<'r, PgRow> for MenuItem {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description").unwrap_or(None),
            price: row.try_get("price")?,
            added_on: row.try_get("added_on")?,
            category_id: row.try_get("category_id")?,
            inserted_at: row.try_get("inserted_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[Object]
impl MenuItem {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn description(&self) -> Option<String> {
        self.description.clone()
    }

    async fn price(&self) -> f64 {
        self.price.to_f64().unwrap_or(0.0)
    }

    async fn added_one(&self) -> String {
        format_date(&self.added_on)
    }

    async fn category_id(&self) -> i32 {
        self.category_id
    }

    async fn inserted_at(&self) -> String {
        format_datetime(&self.inserted_at)
    }

    async fn updated_at(&self) -> String {
        format_datetime(&self.updated_at)
    }
}
