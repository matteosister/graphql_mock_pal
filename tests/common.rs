use graphql_mock_pal::matcher::{Matcher, MatcherOperation};
use serde_json::json;

pub fn get_simple_matchers() -> Vec<Matcher> {
    vec![Matcher {
        operation: MatcherOperation::Query,
        name: "query".to_string(),
        output: json!({"value": 1}),
    }]
}

pub fn get_multiple_matchers(num: usize) -> Vec<Matcher> {
    (1..=num)
        .map(|iteration_num| {
            Matcher::new(
                MatcherOperation::Query,
                format!("query_{}", iteration_num),
                json!({ "value": iteration_num }),
            )
        })
        .collect()
}
