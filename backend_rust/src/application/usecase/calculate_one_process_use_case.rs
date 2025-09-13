use crate::application::repository::process_repository::ProcessRepository;
use crate::application::usecase::interface::AbstractUseCase;
use crate::domain::error::ApiError;
use async_trait::async_trait;
use ulid::Ulid;

pub struct CalculateOneProcessUseCase<'a> {
    pub recipe_id_list: &'a Vec<String>,
    pub repository: &'a dyn ProcessRepository,
}

impl<'a> CalculateOneProcessUseCase<'a> {
    pub fn new(repository: &'a dyn ProcessRepository, recipe_id_list: &'a Vec<String>) -> Self {
        CalculateOneProcessUseCase {
            repository,
            recipe_id_list,
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
        match recipe {
            Ok(_) => Ok(process_id),
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
