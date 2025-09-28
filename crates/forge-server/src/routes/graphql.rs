//! GraphQL HTTP handlers

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, response::Html};

use crate::server::AppState;

/// GraphQL handler
pub async fn graphql_handler(
    State(state): State<AppState>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    state.schema.execute(req.into_inner()).await.into()
}

/// GraphQL playground (development only)
pub async fn graphql_playground() -> Html<String> {
    Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}