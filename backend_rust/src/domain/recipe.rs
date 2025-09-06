use crate::domain::step::Step;

pub struct Recipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<Step>,
}