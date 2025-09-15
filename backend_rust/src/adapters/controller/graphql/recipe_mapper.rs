use crate::adapters::controller::graphql::object::Recipe as RecipeObject;
use crate::adapters::controller::graphql::object::RecipeDetail as RecipeDetailObject;
use crate::adapters::controller::graphql::step_mapper::StepMapper;
use crate::application::mapper::api_mapper::ApiMapper;
use crate::domain::Recipe;

pub struct RecipeMapper {}

impl ApiMapper<Recipe, RecipeObject> for RecipeMapper {
    fn to_api(entity: Recipe) -> RecipeObject {
        RecipeObject {
            id: entity.id,
            description: entity.description,
            title: entity.name,
        }
    }
}

pub struct RecipeDetailMapper {}
impl ApiMapper<Recipe, RecipeDetailObject> for RecipeDetailMapper {
    fn to_api(entity: Recipe) -> RecipeDetailObject {
        RecipeDetailObject {
            id: entity.id,
            description: entity.description,
            title: entity.name,
            steps: entity.steps.into_iter().map(StepMapper::to_api).collect(),
        }
    }
}
