use crate::application::port::ProcessServicePort;
use crate::application::repository::process_repository::ProcessRepository;
use crate::application::usecase::interface::AbstractUseCase;
use crate::domain::error::ApiError;
use async_trait::async_trait;
use ulid::Ulid;

pub struct CalculateOneProcessUseCase<'a> {
    pub recipe_id_list: &'a Vec<String>,
    pub repository: &'a dyn ProcessRepository,
    pub process_service: &'a dyn ProcessServicePort,
}

impl<'a> CalculateOneProcessUseCase<'a> {
    pub fn new(
        repository: &'a dyn ProcessRepository,
        recipe_id_list: &'a Vec<String>,
        process_service: &'a dyn ProcessServicePort,
    ) -> Self {
        CalculateOneProcessUseCase {
            repository,
            recipe_id_list,
            process_service,
        }
    }
}

#[async_trait]
impl<'a> AbstractUseCase<String> for CalculateOneProcessUseCase<'a> {
    async fn execute(&self) -> Result<String, ApiError> {
        let process_id = Ulid::new().to_string();
        let recipe = self
            .repository
            .register_process(process_id.clone(), self.recipe_id_list.to_vec())
            .await;
        if let Err(e) = recipe {
            let e = ApiError {
                code: 400,
                message: String::from("Cannot register process"),
                error: Some(e),
            };
            return Err(e);
        }
        let scheduled_recipes = self.process_service.calculate_process(vec![]).await;
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
