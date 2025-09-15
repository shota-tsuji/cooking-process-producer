use crate::application::port::repository::recipe_repository::RecipeRepository;
use crate::application::usecase::interface::AbstractUseCase;
use crate::domain::Recipe;
use crate::domain::error::ApiError;
use async_trait::async_trait;

pub struct GetOneRecipeByIdUseCase<'a> {
    pub recipe_id: &'a String,
    pub repository: &'a dyn RecipeRepository,
}

impl<'a> GetOneRecipeByIdUseCase<'a> {
    pub fn new(recipe_id: &'a String, repository: &'a dyn RecipeRepository) -> Self {
        GetOneRecipeByIdUseCase {
            recipe_id,
            repository,
        }
    }
}

#[async_trait]
impl<'a> AbstractUseCase<Recipe> for GetOneRecipeByIdUseCase<'a> {
    async fn execute(&self) -> Result<Recipe, ApiError> {
        let recipe = self
            .repository
            .get_recipe_by_id(self.recipe_id.to_string())
            .await;
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
    use crate::application::port::repository::recipe_repository::MockRecipeRepository;
    use crate::domain::entity::recipe::Recipe;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn test_should_return_one_result() {
        // given the "one recipe by id" usecase repo returning one result
        let mut recipe_repository = MockRecipeRepository::new();
        recipe_repository
            .expect_get_recipe_by_id()
            .with(eq(String::from("01K3ZV924ATX4E5P9RZ57HKYF5")))
            .times(1)
            .returning(|_| {
                Ok(Recipe {
                    id: String::from("01K3ZV924ATX4E5P9RZ57HKYF5"),
                    name: String::from("Recipe 1"),
                    description: String::from("Ingredient 1"),
                    steps: vec![],
                })
            });

        // when calling usecase
        let id = String::from("01K3ZV924ATX4E5P9RZ57HKYF5");
        let get_one_recipe_by_id_usecase = GetOneRecipeByIdUseCase::new(&id, &recipe_repository);
        let result = get_one_recipe_by_id_usecase.execute().await.unwrap();

        // then should return one result
        assert_eq!(String::from("01K3ZV924ATX4E5P9RZ57HKYF5"), result.id);
        assert_eq!("Recipe 1", result.name);
    }
}
