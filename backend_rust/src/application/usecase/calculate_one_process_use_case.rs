use crate::application::port::ProcessServicePort;
use crate::application::repository::process_repository::ProcessRepository;
use crate::application::repository::recipe_repository::RecipeRepository;
use crate::application::usecase::interface::AbstractUseCase;
use crate::domain::error::ApiError;
use async_trait::async_trait;
use ulid::Ulid;

pub struct CalculateOneProcessUseCase<'a> {
    pub recipe_id_list: &'a Vec<String>,
    pub repository: &'a dyn ProcessRepository,
    pub process_service: &'a dyn ProcessServicePort,
    pub recipe_repository: &'a dyn RecipeRepository,
}

impl<'a> CalculateOneProcessUseCase<'a> {
    pub fn new(
        repository: &'a dyn ProcessRepository,
        recipe_id_list: &'a Vec<String>,
        process_service: &'a dyn ProcessServicePort,
        recipe_repository: &'a dyn RecipeRepository,
    ) -> Self {
        CalculateOneProcessUseCase {
            repository,
            recipe_id_list,
            process_service,
            recipe_repository,
        }
    }
}

#[async_trait]
impl<'a> AbstractUseCase<String> for CalculateOneProcessUseCase<'a> {
    async fn execute(&self) -> Result<String, ApiError> {
        let process_id = Ulid::new().to_string();
        self.repository
            .register_process(process_id.clone(), self.recipe_id_list.to_vec())
            .await
            .map_err(|e| ApiError {
                code: 400,
                message: String::from("Cannot register process"),
                error: Some(e),
            })?;
        let recipes = self
            .recipe_repository
            .get_recipes_by_ids(self.recipe_id_list.to_vec())
            .await
            .map_err(|e| ApiError {
                code: 400,
                message: String::from("Cannot get recipes by ids"),
                error: Some(e),
            })?;
        let scheduled_recipes = self.process_service.calculate_process(recipes).await;
        if let Err(e) = scheduled_recipes {
            let e = ApiError {
                code: 400,
                message: String::from("Cannot calculate process"),
                error: Some(e),
            };
            return Err(e);
        }
        println!("Calculate process successfully");
        Ok(process_id)
    }
}
