extern crate graphql_mock_pal;

use graphql_mock_pal::matcher::{match_query, Matcher, MatcherOperation};

#[test]
fn test_no_match_if_no_matcher_is_specified() {
    let query = "{query_name {field1 field2}}";
    let matchers = vec![];
    assert!(match_query(query, &matchers).is_none());
}

#[test]
fn test_simple_match_with_one_matcher_on_the_query_name() {
    let query = "{query_name {field1 field2}}";
    let matcher = Matcher {
        operation: MatcherOperation::Query,
        name: "query_name",
    };
    assert_eq!(
        Matcher {
            operation: MatcherOperation::Query,
            name: "query_name"
        },
        match_query(query, &vec![matcher]).unwrap()
    );
}
