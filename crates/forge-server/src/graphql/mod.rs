//! GraphQL schema and resolvers

use anyhow::Result;
use async_graphql::{EmptyMutation, EmptySubscription, Schema as GraphQLSchema};
use forge_storage::DbPool;

mod query;

pub type Schema = GraphQLSchema<query::Query, EmptyMutation, EmptySubscription>;

/// Create the GraphQL schema
pub async fn create_schema(db_pool: DbPool) -> Result<Schema> {
    let schema = Schema::build(
        query::Query::default(),
        EmptyMutation,
        EmptySubscription,
    )
    .data(db_pool)
    .finish();
    
    Ok(schema)
}