use crate::graphql::elixir::MenuItem;
use async_graphql::FieldResult;
use async_graphql::{Context, Object, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};
use sqlx::PgPool;

pub mod elixir;

pub(crate) type ServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// This is the Query object within your schema. It is the root of all queries users can use at your service.
pub(crate) struct Query;

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
        let menu_items = sqlx::query_as!(
            MenuItem,
            r#"
            SELECT id, name, description, price, added_on, category_id, inserted_at, updated_at
            FROM items
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(menu_items)
    }
}
