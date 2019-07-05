use crate::matcher::{match_query, Matcher, MatcherOperation};
use actix_web::web::Json;
use actix_web::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct GraphqlRequest {
    pub query: String,
    pub parameters: Value,
}

#[derive(Serialize, Debug)]
pub struct Output(String);

pub fn query_handler(graphql_request: Json<GraphqlRequest>) -> Result<Json<Output>> {
    Ok(Json(do_handle_query(graphql_request.into_inner())))
}

fn do_handle_query(graphql_request: GraphqlRequest) -> Output {
    let matchers = get_matchers();
    let _matched = match_query(graphql_request.query.as_str(), &matchers);

    Output("test".to_owned())
}

fn get_matchers<'a>() -> Vec<Matcher<'a>> {
    let matcher = Matcher {
        operation: MatcherOperation::Query,
        name: "field",
        output: json!({"a": 1}),
    };
    vec![matcher]
}
