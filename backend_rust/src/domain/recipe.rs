use crate::domain::step::Step;

pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub steps: Vec<Step>,
}