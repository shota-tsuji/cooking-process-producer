use crate::domain::recipe::Recipe;
use async_trait::async_trait;
use std::error::Error;

#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait RecipeRepository: Send + Sync {
    async fn get_recipe_by_id(&self, resource_id: String) -> Result<Recipe, Box<dyn Error>>;
    async fn get_all_recipes(&self) -> Result<Vec<Recipe>, Box<dyn Error>>;
}
