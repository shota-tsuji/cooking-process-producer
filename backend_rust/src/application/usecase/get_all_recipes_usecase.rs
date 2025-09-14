use crate::application::repository::recipe_repository::RecipeRepository;
use crate::application::usecase::interface::AbstractUseCase;
use crate::domain::Recipe;
use crate::domain::error::ApiError;
use async_trait::async_trait;

pub struct GetAllRecipesUseCase<'a> {
    pub repository: &'a dyn RecipeRepository,
}

impl<'a> GetAllRecipesUseCase<'a> {
    pub fn new(repository: &'a dyn RecipeRepository) -> Self {
        GetAllRecipesUseCase { repository }
    }
}

#[async_trait]
impl<'a> AbstractUseCase<Vec<Recipe>> for GetAllRecipesUseCase<'a> {
    async fn execute(&self) -> Result<Vec<Recipe>, ApiError> {
        let recipes = self.repository.get_all_recipes().await;
        match recipes {
            Ok(recipes) => Ok(recipes),
            Err(e) => {
                let e = ApiError {
                    code: 400,
                    message: String::from("Cannot get all recipes"),
                    error: Some(e),
                };
                Err(e)
            }
        }
    }
}
