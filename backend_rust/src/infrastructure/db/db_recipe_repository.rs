use crate::application::repository::recipe_repository::RecipeRepository;
use sea_orm::DatabaseConnection;

use crate::domain::recipe::Recipe;
use crate::domain::step::Step;
use crate::infrastructure::mysql::entity as db_entity;
use crate::adapters::step_mapper::StepMapper;
use crate::application::mapper::db_mapper::DbMapper;
use async_trait::async_trait;
use sea_orm::*;
pub struct DbRecipeRepository {
    pub db_connection: DatabaseConnection,
}

#[async_trait]
impl RecipeRepository for DbRecipeRepository {
    async fn get_recipe_by_id(
        &self,
        recipe_id: String,
    ) -> Result<Recipe, Box<dyn std::error::Error>> {
        let mut recipe_with_steps = db_entity::recipes::Entity::find_by_id(recipe_id.clone())
            .find_with_related(db_entity::steps::Entity)
            .all(&self.db_connection)
            .await
            .unwrap();
        let (recipe_model, step_models) = recipe_with_steps.pop().unwrap();
        let steps: Vec<Step> = step_models
            .into_iter()
            .map(StepMapper::to_entity)
            .collect();

        Ok(Recipe {
            id: recipe_id,
            name: recipe_model.title,
            description: recipe_model.description.unwrap_or_default(),
            steps,
        })
    }

    async fn get_all_recipes(
        &self,
    ) -> Result<Vec<crate::domain::recipe::Recipe>, Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
