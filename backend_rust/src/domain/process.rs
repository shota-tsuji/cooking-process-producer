use crate::domain::recipe::Recipe;
#[allow(dead_code)]
pub struct Process {
    pub id: String,
    pub recipes: Vec<Recipe>,
}
