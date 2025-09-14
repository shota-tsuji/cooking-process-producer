use crate::domain::Recipe;
#[allow(dead_code)]
pub struct Process {
    pub id: String,
    pub recipes: Vec<Recipe>,
}
