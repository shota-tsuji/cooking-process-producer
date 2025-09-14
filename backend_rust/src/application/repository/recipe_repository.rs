use crate::domain::entity::recipe::Recipe;
use async_trait::async_trait;

use crate::domain::error::AsyncDynError;
#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait RecipeRepository: Send + Sync {
    async fn get_recipe_by_id(&self, resource_id: String) -> Result<Recipe, Box<AsyncDynError>>;
    async fn get_all_recipes(&self) -> Result<Vec<Recipe>, Box<AsyncDynError>>;
}
