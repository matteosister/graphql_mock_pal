extern crate graphql_mock_pal;

use graphql_mock_pal::matcher::{match_query, Matcher, MatcherOperation};

#[test]
fn test_no_match_if_no_matcher_is_specified() {
    let query = "{query_name {field1 field2}}";
    let matchers = vec![];
    assert!(match_query(query, &matchers).is_empty());
}

#[test]
fn test_simple_match_with_one_matcher_on_the_query_name() {
    let query = "{query_name {field1 field2}}";
    let matcher = Matcher {
        operation: MatcherOperation::Query,
        name: "query_name",
    };

    assert_eq!(
        vec![&Matcher {
            operation: MatcherOperation::Query,
            name: "query_name",
        }],
        match_query(query, &vec![matcher])
    );
}

#[test]
fn test_non_matching_with_query_name() {
    let query = "{query_name {field1 field2}}";
    let matcher = Matcher {
        operation: MatcherOperation::Query,
        name: "another_query_name",
    };
    let expected:Vec<&Matcher> = vec![];

    assert_eq!(
        expected,
        match_query(query, &vec![matcher])
    );
}

#[test]
fn test_two_different_queried() {
    let query = "{query_name {field1 field2} query_2 { a b c }}";
    let matcher = Matcher {
        operation: MatcherOperation::Query,
        name: "query_name",
    };
    let matcher2 = Matcher {
        operation: MatcherOperation::Query,
        name: "query_2",
    };

    assert_eq!(
        vec![&Matcher {
            operation: MatcherOperation::Query,
            name: "query_name",
        }, &Matcher {
            operation: MatcherOperation::Query,
            name: "query_2",
        }],
        match_query(query, &vec![matcher, matcher2])
    );
}

