use graphql_parser::parse_query;
use graphql_parser::query::Definition::Operation;
use graphql_parser::query::{Definition, Document, Field, OperationDefinition, Selection};
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum MatcherOperation {
    Query,
    Mutation,
}

#[derive(Debug, PartialEq)]
pub struct Matcher {
    pub operation: MatcherOperation,
    pub name: Vec<String>,
    pub output: Value,
}

impl Matcher {
    fn matches_field(&self, field: &Field) -> bool {
        self.name
            .last()
            .map(|last_name| last_name.eq(&field.name))
            .unwrap_or(false)
    }

    pub fn new(operation: MatcherOperation, name: Vec<String>, output: Value) -> Self {
        Self {
            operation,
            name,
            output,
        }
    }
}

pub fn match_query<'a>(query: &str, matchers: &'a [Matcher]) -> Vec<&'a Matcher> {
    let document = parse_query(query).expect("malformed query");
    do_match_query(&Default::default(), document, matchers)
}

fn do_match_query<'a>(
    branches: &Vec<&str>,
    document: Document,
    matchers: &'a [Matcher],
) -> Vec<&'a Matcher> {
    document
        .definitions
        .into_iter()
        .flat_map(|definition| match_definition(branches, &definition, matchers))
        .collect()
}

fn match_definition<'a>(
    branches: &Vec<&str>,
    definition: &Definition,
    matchers: &'a [Matcher],
) -> Vec<&'a Matcher> {
    match definition {
        Operation(operation_definition) => {
            match_operation_definition(branches, operation_definition, matchers)
        }
        _ => Default::default(),
    }
}

fn match_operation_definition<'a>(
    branches: &Vec<&str>,
    operation_definition: &OperationDefinition,
    matchers: &'a [Matcher],
) -> Vec<&'a Matcher> {
    match operation_definition {
        OperationDefinition::SelectionSet(selection_set) => selection_set
            .items
            .iter()
            .flat_map(|selection| match_selection(branches, selection, matchers))
            .collect(),
        OperationDefinition::Query(_) => Default::default(),
        OperationDefinition::Mutation(_) => Default::default(),
        OperationDefinition::Subscription(_) => Default::default(),
    }
}

fn match_selection<'a>(
    branches: &Vec<&str>,
    selection: &Selection,
    matchers: &'a [Matcher],
) -> Vec<&'a Matcher> {
    match selection {
        Selection::Field(field) => {
            let mut matched: Vec<&'a Matcher> = matchers
                .iter()
                .filter(|matcher| matcher.matches_field(field))
                .collect();

            let mut selection_set = match_items(branches, &field.selection_set.items, matchers);

            matched.append(&mut selection_set);
            matched
        }
        Selection::FragmentSpread(_) => Default::default(),
        Selection::InlineFragment(_) => Default::default(),
    }
}

fn match_items<'a>(
    branches: &Vec<&str>,
    selections: &Vec<Selection>,
    matchers: &'a [Matcher],
) -> Vec<&'a Matcher> {
    selections
        .into_iter()
        .flat_map(|selection| match_selection(branches, selection, matchers))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn default_matcher<'a>() -> Matcher {
        Matcher {
            operation: MatcherOperation::Query,
            name: vec!["query_name".to_string()],
            output: json!({"a": 1}),
        }
    }

    fn subfield_matcher<'a>() -> Matcher {
        Matcher {
            operation: MatcherOperation::Query,
            name: vec!["query_name".to_string(), "subfield".to_string()],
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
            name: vec!["query_name".to_string()],
            output: json!({"a": 1}),
        };
        let matcher2 = Matcher {
            operation: MatcherOperation::Query,
            name: vec!["query_2".to_string()],
            output: json!({"b": 2}),
        };

        assert_eq!(
            vec![
                &Matcher {
                    operation: MatcherOperation::Query,
                    name: vec!["query_name".to_string()],
                    output: json!({"a": 1})
                },
                &Matcher {
                    operation: MatcherOperation::Query,
                    name: vec!["query_2".to_string()],
                    output: json!({"b": 2})
                }
            ],
            match_query(query, &vec![matcher, matcher2])
        );
    }

    #[test]
    fn test_nested_match() {
        let query = "{query_name {subfield {field1 field2}}}";
        let subfield_matcher1 = subfield_matcher();
        let subfield_matcher2 = subfield_matcher();

        assert_eq!(
            vec![&subfield_matcher1],
            match_query(query, &vec![subfield_matcher2])
        );
    }
}
