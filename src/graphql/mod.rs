use crate::graphql::elixir::MenuItem;
use async_graphql::FieldResult;
use async_graphql::{Context, Object, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};
use chrono::format::StrftimeItems;
use elixir::Category;
use sqlx::PgPool;

pub mod elixir;

pub(crate) type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// This is the Query object within your schema. It is the root of all queries users can use at your service.
pub(crate) struct Query;

/// Convert datetime to string as format: "2015-09-05 23:56:04"
pub fn format_datetime(datetime: &chrono::NaiveDateTime) -> String {
    let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
    datetime.format_with_items(fmt.clone()).to_string()
}

/// Convert date to string as format: "2015-09-05"
pub fn format_date(date: &chrono::NaiveDate) -> String {
    let fmt = StrftimeItems::new("%Y-%m-%d");
    date.format_with_items(fmt.clone()).to_string()
}

/// The implementation of Query contains all queries your service supports.
#[Object]
impl Query {
    /// hello is your first query. It just returns a static string for now
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello World"
    }

    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn menu_items(&self, ctx: &Context<'_>) -> FieldResult<Vec<MenuItem>> {
        let pool = ctx.data::<PgPool>()?;
        let menu_items = sqlx::query_as::<_, MenuItem>(
            r#"
            SELECT id, name, description, price, added_on, category_id, inserted_at, updated_at
            FROM items
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(menu_items)
    }

    async fn categories(&self, ctx: &Context<'_>) -> FieldResult<Vec<Category>> {
        let pool = ctx.data::<PgPool>()?;
        let categories = sqlx::query_as::<_, Category>(
            r#"
            select id, name, description, inserted_at, updated_at from categories
        "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(categories)
    }
}
