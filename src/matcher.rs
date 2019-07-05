use graphql_parser::parse_query;
use graphql_parser::query::Definition::Operation;
use graphql_parser::query::{Definition, Field, OperationDefinition, Selection};
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum MatcherOperation {
    Query,
    Mutation,
}

#[derive(Debug, PartialEq)]
pub struct Matcher<'a> {
    pub operation: MatcherOperation,
    pub name: &'a str,
    pub output: Value,
}

impl<'a> Matcher<'a> {
    fn matches_field(&self, field: &Field) -> bool {
        self.name == field.name
    }
}

pub fn match_query<'a>(query: &str, matchers: &'a [Matcher]) -> Vec<&'a Matcher<'a>> {
    let query_parsed = parse_query(query).expect("malformed query");
    query_parsed
        .definitions
        .into_iter()
        .flat_map(|definition| match_definition(&definition, matchers))
        .collect()
}

fn match_definition<'a>(
    definition: &Definition,
    matchers: &'a [Matcher<'a>],
) -> Vec<&'a Matcher<'a>> {
    match definition {
        Operation(operation_definition) => {
            match_operation_definition(operation_definition, matchers)
        }
        _ => Default::default(),
    }
}

fn match_operation_definition<'a>(
    operation_definition: &OperationDefinition,
    matchers: &'a [Matcher<'a>],
) -> Vec<&'a Matcher<'a>> {
    match operation_definition {
        OperationDefinition::SelectionSet(selection_set) => selection_set
            .items
            .iter()
            .flat_map(|selection| match_selection(selection, matchers))
            .collect(),
        OperationDefinition::Query(_) => Default::default(),
        OperationDefinition::Mutation(_) => Default::default(),
        OperationDefinition::Subscription(_) => Default::default(),
    }
}

fn match_selection<'a>(selection: &Selection, matchers: &'a [Matcher<'a>]) -> Vec<&'a Matcher<'a>> {
    match selection {
        Selection::Field(field) => matchers
            .iter()
            .filter(|matcher| matcher.matches_field(field))
            .collect(),
        Selection::FragmentSpread(_) => Default::default(),
        Selection::InlineFragment(_) => Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn default_matcher<'a>() -> Matcher<'a> {
        Matcher {
            operation: MatcherOperation::Query,
            name: "query_name",
            output: json!({"a": 1}),
        }
    }

    #[test]
    fn test_no_match_if_no_matcher_is_specified() {
        let query = "{query_name {field1 field2}}";
        let matchers = vec![];
        assert!(match_query(query, &matchers).is_empty());
    }

    #[test]
    fn test_simple_match_with_one_matcher_on_the_query_name() {
        let query = "{query_name {field1 field2}}";
        let default_matcher1 = default_matcher();
        let default_matcher2 = default_matcher();
        assert_eq!(
            vec![&default_matcher1],
            match_query(query, &vec![default_matcher2])
        );
    }

    #[test]
    fn test_non_matching_with_query_name() {
        let query = "{another_query {field1 field2}}";
        let default_matcher = default_matcher();
        let expected: Vec<&Matcher> = vec![];

        assert_eq!(expected, match_query(query, &vec![default_matcher]));
    }

    #[test]
    fn test_two_different_queried() {
        let query = "{query_name {field1 field2} query_2 { a b c }}";
        let matcher = Matcher {
            operation: MatcherOperation::Query,
            name: "query_name",
            output: json!({"a": 1}),
        };
        let matcher2 = Matcher {
            operation: MatcherOperation::Query,
            name: "query_2",
            output: json!({"b": 2}),
        };

        assert_eq!(
            vec![
                &Matcher {
                    operation: MatcherOperation::Query,
                    name: "query_name",
                    output: json!({"a": 1})
                },
                &Matcher {
                    operation: MatcherOperation::Query,
                    name: "query_2",
                    output: json!({"b": 2})
                }
            ],
            match_query(query, &vec![matcher, matcher2])
        );
    }

}
