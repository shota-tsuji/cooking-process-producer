use crate::application::repository::RecipeRepository;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::adapters::db::mysql::entity as db_entity;
use crate::adapters::recipe_mapper::RecipeMapper;
use crate::adapters::step_mapper::StepMapper;
use crate::application::mapper::db_mapper::DbMapper;
use crate::domain::Recipe;
use crate::domain::Step;
use crate::domain::error::AsyncDynError;
use async_trait::async_trait;
use sea_orm::*;

pub struct DbRecipeRepository {
    pub db_connection: Arc<DatabaseConnection>,
}

#[async_trait]
impl RecipeRepository for DbRecipeRepository {
    async fn get_recipe_by_id(&self, recipe_id: String) -> Result<Recipe, Box<AsyncDynError>> {
        let mut recipe_with_steps = db_entity::recipes::Entity::find_by_id(recipe_id.clone())
            .find_with_related(db_entity::steps::Entity)
            .all(&*self.db_connection)
            .await
            .unwrap();
        let (recipe_model, step_models) = recipe_with_steps.pop().unwrap();
        let steps: Vec<Step> = step_models.into_iter().map(StepMapper::to_entity).collect();

        Ok(Recipe {
            id: recipe_id,
            name: recipe_model.title,
            description: recipe_model.description.unwrap_or_default(),
            steps,
        })
    }

    async fn get_recipes_by_ids(
        &self,
        recipe_ids: Vec<String>,
    ) -> Result<Vec<Recipe>, Box<AsyncDynError>> {
        use db_entity::recipes;
        use db_entity::steps;
        let recipe_models: Vec<(recipes::Model, Vec<steps::Model>)> = recipes::Entity::find()
            .filter(recipes::Column::Id.is_in(recipe_ids.clone()))
            .find_with_related(steps::Entity)
            .all(&*self.db_connection)
            .await?;

        let mut recipes: Vec<Recipe> = Vec::new();
        for (recipe, steps) in recipe_models {
            let steps: Vec<Step> = steps.into_iter().map(StepMapper::to_entity).collect();
            let mut recipe = RecipeMapper::to_entity(recipe);
            recipe.steps = steps;
            recipes.push(recipe);
        }
        Ok(recipes)
    }

    async fn get_all_recipes(&self) -> Result<Vec<Recipe>, Box<AsyncDynError>> {
        let recipe_models: Vec<db_entity::recipes::Model> = db_entity::recipes::Entity::find()
            .all(&*self.db_connection)
            .await?;
        let recipes = recipe_models
            .into_iter()
            .map(RecipeMapper::to_entity)
            .collect();
        Ok(recipes)
    }
}
