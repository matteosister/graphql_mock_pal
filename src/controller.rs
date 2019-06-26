use actix_web::web::Json;
use actix_web::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct GraphqlRequest {
    query: String,
    parameters: Value,
}

#[derive(Serialize)]
pub struct Output {
    query: String,
    parameters: Value,
}

impl From<GraphqlRequest> for Output {
    fn from(graphql_request: GraphqlRequest) -> Self {
        Self {
            query: graphql_request.query,
            parameters: graphql_request.parameters,
        }
    }
}

pub fn query_handler(graphql_request: Json<GraphqlRequest>) -> Result<Json<Output>> {
    Ok(Json(graphql_request.into_inner().into()))
}
