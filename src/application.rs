use actix_web::web::Json;
use actix_web::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::matcher::{match_query, MatcherOperation, Matcher};

#[derive(Deserialize)]
pub struct GraphqlRequest {
    pub query: String,
    pub parameters: Value,
}

#[derive(Serialize, Debug)]
pub struct Output(String);

impl From<GraphqlRequest> for Output {
    fn from(graphql_request: GraphqlRequest) -> Self {
        let matchers = vec![Matcher {
            operation: MatcherOperation::Query,
            name: "field",
        }];
        let res = match_query(graphql_request.query.as_str(), &matchers);
        dbg!(&res);
        Self("pippo".to_owned())
    }
}

pub fn query_handler(graphql_request: Json<GraphqlRequest>) -> Result<Json<Output>> {
    Ok(Json(graphql_request.into_inner().into()))
}

fn do_handle_query(graphql_request: GraphqlRequest) -> Output {
    let matchers = get_matchers();
    let matched = match_query(graphql_request.query.as_str(), &matchers);

    Output("test".to_owned())
}

fn get_matchers<'a>() -> Vec<Matcher<'a>> {
    let matcher = Matcher {
        operation: MatcherOperation::Query,
        name: "field",
    };
    vec![matcher]
}