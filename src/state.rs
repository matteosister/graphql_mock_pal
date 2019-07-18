use crate::matcher::Matcher;

pub struct AppState {
    matchers: Vec<Matcher>,
}

impl AppState {
    pub fn new(matchers: Vec<Matcher>) -> Self {
        Self { matchers }
    }

    pub fn get_matchers(&self) -> &Vec<Matcher> {
        &self.matchers
    }
}
