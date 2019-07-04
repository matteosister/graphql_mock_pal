use actix_web::web::Json;
use graphql_mock_pal::application::{GraphqlRequest, Output};
use serde_json::json;

#[test]
fn simple_match() {
    let request = GraphqlRequest {
        query: "{field { a b }}".to_owned(),
        parameters: json!({}),
    };
    let response: Output = request.into();
    dbg!(response);
    assert_eq!(1, 2);
}
