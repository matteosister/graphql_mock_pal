pub mod common;

use common::{get_multiple_matchers, get_simple_matchers};

use graphql_mock_pal::application::{do_handle_query, get_empty_response, GraphqlRequest};
use graphql_mock_pal::matcher::{Matcher, MatcherOperation};
use serde_json::json;

#[test]
fn no_matches_gives_default_error_response() {
    let request = GraphqlRequest {
        query: "{field { a b }}".to_owned(),
        parameters: json!({}),
    };
    let matchers = get_simple_matchers();

    let output = do_handle_query(request, matchers);
    assert_eq!(get_empty_response().to_string(), output.to_string());
}

#[test]
fn single_match_with_single_response() {
    let request = GraphqlRequest {
        query: "{query { a }}".to_owned(),
        parameters: json!({}),
    };
    let matchers = get_simple_matchers();
    let output = do_handle_query(request, matchers);
    let expected = json!({"data": {"query": {"value": 1}}});
    assert_eq!(expected.to_string(), output.to_string());
}

#[test]
fn multiple_matches_gives_multiple_responses() {
    let request = GraphqlRequest {
        query: "{query_1 { a } query_2 {b}}".to_owned(),
        parameters: json!({}),
    };
    let matchers = get_multiple_matchers(2);
    let output = do_handle_query(request, matchers);
    let expected = json!({"data": {"query_1": {"value": 1}, "query_2": {"value": 2}}});
    assert_eq!(expected.to_string(), output.to_string());
}

#[test]
fn nested_match() {
    let request = GraphqlRequest {
        query: "{query_1 { query_2 {a} }}".to_owned(),
        parameters: json!({}),
    };
    let matchers = vec![Matcher {
        operation: MatcherOperation::Query,
        name: vec!["query_1".to_string(), "query_2".to_string()],
        output: json!({"value": 1}),
    }];
    let output = do_handle_query(request, matchers);
    let expected = json!({"data": {"query_1": {"query_2": {"value": 1}}}});
    assert_eq!(expected.to_string(), output.to_string());
}
