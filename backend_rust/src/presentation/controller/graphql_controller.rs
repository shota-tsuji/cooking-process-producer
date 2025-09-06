use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;

use crate::presentation::graphql::query::QuerySchema;

pub async fn graphql_handler(
    schema: Extension<QuerySchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
