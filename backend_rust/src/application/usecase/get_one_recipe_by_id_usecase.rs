use crate::application::repository::recipe_repository::RecipeRepository;
use crate::application::usecase::interface::AbstractUseCase;
use crate::domain::error::ApiError;
use crate::domain::recipe::Recipe;

pub struct GetOneRecipeByIdUseCase<'a> {
    pub recipe_id: &'a i32,
    pub repository: &'a dyn RecipeRepository,
}

impl<'a> GetOneRecipeByIdUseCase<'a> {
    pub fn new(recipe_id: &'a i32, repository: &'a dyn RecipeRepository) -> Self {
        GetOneRecipeByIdUseCase { recipe_id, repository }
    }
}

impl<'a> AbstractUseCase<Recipe> for GetOneRecipeByIdUseCase<'a> {
    async fn execute(&self) -> Result<Recipe, ApiError> {
        let recipe = self.repository.get_recipe_by_id(*self.recipe_id).await;
        match recipe {
            Ok(recipe) => Ok(recipe),
            Err(e) => {
                let e = ApiError {
                    code: 400,
                    message: String::from("Cannot get single recipe"),
                    error: Some(e),
                };
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use std::io::{Error, ErrorKind};
    use crate::{
        application::repository::recipe_repository::MockRecipeRepository,
        domain::recipe::Recipe,
    };

    #[tokio::test]
    async fn test_should_return_one_result() {
        // given the "one recipe by id" usecase repo returning one result
        let mut recipe_repository = MockRecipeRepository::new();
        recipe_repository
            .expect_get_recipe_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| {
                Ok(Recipe {
                    id: 1,
                    name: String::from("Recipe 1"),
                    description: String::from("Ingredient 1"),
                    steps: vec![],
                })
            });

        // when calling usecase
        let get_one_recipe_by_id_usecase = GetOneRecipeByIdUseCase::new(&1, &recipe_repository);
        let result = get_one_recipe_by_id_usecase.execute().await.unwrap();
        // then should return one result
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "Recipe 1");
    }
}