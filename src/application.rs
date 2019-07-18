use crate::matcher::{match_query, Matcher};
use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct GraphqlRequest {
    pub query: String,
    pub parameters: Option<Value>,
}

#[derive(Serialize, Debug)]
pub struct Output(String);

impl Responder for Output {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = &self.0;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

pub fn query_handler(
    graphql_request: Json<GraphqlRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    do_handle_query(graphql_request.into_inner(), data.get_matchers())
}

pub fn do_handle_query(graphql_request: GraphqlRequest, matchers: &Vec<Matcher>) -> Output {
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
                let internal_value = if names.is_empty() {
                    matcher.output.clone()
                } else {
                    let res = names.into_iter().fold(
                        (json!({}), Some(matcher)),
                        |(int_value, matcher), name| {
                            let mut new_value = json!({});
                            let val = match matcher {
                                Some(m) => m.output.clone(),
                                None => int_value,
                            };
                            new_value[name] = val;
                            (new_value, None)
                        },
                    );
                    res.0
                };

                value["data"]
                    .as_object_mut()
                    .unwrap()
                    .insert(root, internal_value);
                value
            });

        Output(result.to_string())
    }
}

pub fn get_empty_response() -> Value {
    json!({"errors": [{"message": "the field field could not be found"}]})
}
