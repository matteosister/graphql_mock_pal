use graphql_parser::parse_query;
use graphql_parser::query::{Definition, OperationDefinition, Selection};
use std::string::ParseError;
use graphql_parser::query::Definition::Operation;


#[derive(Debug, PartialEq)]
pub enum MatcherOperation {
    Query,
    Mutation,
}

#[derive(Debug, PartialEq)]
pub struct Matcher<'a> {
    pub operation: MatcherOperation,
    pub name: &'a str,
}

pub fn match_query<'a>(query: &str, matchers: &'a Vec<Matcher>) -> Option<Matcher<'a>> {
    let query_parsed = parse_query(query).expect("malformed query");
    let definition = query_parsed
        .definitions
        .into_iter()
        .find(|definition| match_definition(definition, matchers));

    None
}

fn match_definition<'a>(definition: &Definition, matchers: &Vec<Matcher>) -> bool {
    match definition {
        Operation(operation_definition) => {
            match_operation_definition(operation_definition, matchers)
        }
        _ => false
    }
}

fn match_operation_definition(operation_definition: &OperationDefinition, matchers: &Vec<Matcher>) -> bool {
    match operation_definition {
        OperationDefinition::SelectionSet(selection_set) => {
            selection_set.items.clone().into_iter().find(|selection| match_selection(selection, matchers)).is_some()
        },
        OperationDefinition::Query(query) => false,
        OperationDefinition::Mutation(_) => false,
        OperationDefinition::Subscription(_) => false,
    }
}

fn match_selection(selection: &Selection, matchers: &Vec<Matcher>) -> bool {
    match selection {
        Selection::Field(field) => {
            dbg!(field);
            false
        },
        Selection::FragmentSpread(_) => false,
        Selection::InlineFragment(_) => false,
    }
}