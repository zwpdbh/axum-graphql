use async_graphql::{Context, Object, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};

pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

/// This is the Query object within your schema. It is the root of all queries users can use at your service.
pub(crate) struct QueryRoot;

/// The implementation of QueryRoot contains all queries your service supports.
#[Object]
impl QueryRoot {
    /// hello is your first query. It just returns a static string for now
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello World"
    }
}
