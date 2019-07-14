use crate::matcher::{match_query, Matcher, MatcherOperation};
use actix_web::web::Json;
use actix_web::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt;

#[derive(Deserialize)]
pub struct GraphqlRequest {
    pub query: String,
    pub parameters: Value,
}

#[derive(Serialize, Debug)]
pub struct Output(String);

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn query_handler(graphql_request: Json<GraphqlRequest>) -> Result<Json<String>> {
    let matchers = get_matchers();
    let output = do_handle_query(graphql_request.into_inner(), matchers);
    Ok(Json(output.0))
}

pub fn do_handle_query(graphql_request: GraphqlRequest, matchers: Vec<Matcher>) -> Output {
    let matched = match_query(graphql_request.query.as_str(), &matchers);
    if matched.is_empty() {
        Output(get_empty_response().to_string())
    } else {
        let result: Value = matched
            .into_iter()
            .fold(json!({"data": {}}), |mut value, matcher| {
                let mut names = matcher.name.clone();
                names.reverse();
                let root = names.pop().unwrap();
                let internal_value =
                    if names.is_empty() {
                        matcher.output.clone()
                    } else {
                        let res = names
                            .into_iter()
                            .fold((json!({}), Some(matcher)), |(int_value, matcher), name| {
                                let mut new_value = json!({});
                                let val = match matcher {
                                    Some(m) => m.output.clone(),
                                    None => int_value
                                };
                                new_value[name] = val;
                                (new_value, None)
                            });
                        res.0
                    };

                value["data"].as_object_mut().unwrap().insert(root, internal_value);
                value
            });

        Output(result.to_string())
    }
}

fn get_matchers<'a>() -> Vec<Matcher> {
    let matcher = Matcher {
        operation: MatcherOperation::Query,
        name: vec!["field".to_string()],
        output: json!({"a": 1}),
    };
    vec![matcher]
}

pub fn get_empty_response() -> Value {
    json!({"errors": [{"message": "the field field could not be found"}]})
}
