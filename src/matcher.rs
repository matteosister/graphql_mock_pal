use graphql_parser::parse_query;
use graphql_parser::query::Definition::Operation;
use graphql_parser::query::{Definition, Field, OperationDefinition, Selection};

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

pub fn match_query<'a>(query: &str, matchers: &'a [Matcher]) -> Vec<&'a Matcher<'a>> {
    let query_parsed = parse_query(query).expect("malformed query");
    query_parsed
        .definitions
        .into_iter()
        .flat_map(|definition| match_definition(dbg!(&definition), matchers))
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
