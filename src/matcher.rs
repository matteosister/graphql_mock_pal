use graphql_parser::parse_query;
use graphql_parser::query::Definition::Operation;
use graphql_parser::query::{Definition, Field, OperationDefinition, Selection};
use std::string::ParseError;

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

impl<'a> Matcher<'a> {
    fn matches_field(&self, field: &Field) -> bool {
        self.name == field.name
    }
}

pub fn match_query<'a>(query: &str, matchers: &'a Vec<Matcher>) -> Option<&'a Matcher<'a>> {
    let query_parsed = parse_query(query).expect("malformed query");
    query_parsed
        .definitions
        .into_iter()
        .filter_map(|definition| match_definition(&definition, matchers))
        .find(|m| true)
}

fn match_definition<'a>(
    definition: &Definition,
    matchers: &'a Vec<Matcher <'a>>,
) -> Option<&'a Matcher<'a>> {
    match definition {
        Operation(operation_definition) => {
            match_operation_definition(operation_definition, matchers)
        }
        _ => None,
    }
}

fn match_operation_definition<'a>(
    operation_definition: &OperationDefinition,
    matchers: &'a Vec<Matcher<'a>>,
) -> Option<&'a Matcher<'a>> {
    match operation_definition {
        OperationDefinition::SelectionSet(selection_set) => selection_set
            .items
            .iter()
            .filter_map(|selection| match_selection(selection, matchers))
            .find(|m| true)
        ,
        OperationDefinition::Query(query) => None,
        OperationDefinition::Mutation(_) => None,
        OperationDefinition::Subscription(_) => None,
    }
}

fn match_selection<'a>(
    selection: &Selection,
    matchers: &'a Vec<Matcher<'a>>,
) -> Option<&'a Matcher<'a>> {
    match selection {
        Selection::Field(field) => matchers.iter().find(|matcher| matcher.matches_field(field)),
        Selection::FragmentSpread(_) => None,
        Selection::InlineFragment(_) => None,
    }
}
