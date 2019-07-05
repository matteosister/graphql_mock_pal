use graphql_mock_pal::application::{GraphqlRequest, Output};
use serde_json::json;

#[test]
fn simple_match() {
    let request = GraphqlRequest {
        query: "{field { a b }}".to_owned(),
        parameters: json!({}),
    };
    assert_eq!(2, 1);
}
