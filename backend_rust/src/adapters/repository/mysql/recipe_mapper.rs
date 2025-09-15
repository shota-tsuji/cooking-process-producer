use crate::adapters::repository::mysql::entity::recipes::Model as RecipeModel;
use crate::application::mapper::db_mapper::DbMapper;
use crate::domain::Recipe;

pub struct MysqlRecipeMapper {}

impl DbMapper<Recipe, RecipeModel> for MysqlRecipeMapper {
    fn to_db(entity: Recipe) -> RecipeModel {
        RecipeModel {
            id: entity.id,
            title: entity.name,
            description: Some(entity.description),
        }
    }

    fn to_entity(model: RecipeModel) -> Recipe {
        Recipe {
            id: model.id,
            name: model.title,
            description: model.description.unwrap_or_default(),
            steps: vec![],
        }
    }
}
