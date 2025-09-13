use crate::application::repository::process_registrations_repository::ProcessRegistrationRepository;
use crate::application::usecase::interface::AbstractUseCase;
use crate::domain::error::ApiError;
use async_trait::async_trait;

pub struct CalculateOneProcessUseCase<'a> {
    pub recipe_id_list: &'a Vec<String>,
    pub repository: &'a dyn ProcessRegistrationRepository,
}

impl<'a> CalculateOneProcessUseCase<'a> {
    pub fn new(
        repository: &'a dyn ProcessRegistrationRepository,
        recipe_id_list: &'a Vec<String>,
    ) -> Self {
        CalculateOneProcessUseCase {
            repository,
            recipe_id_list,
        }
    }
}

#[async_trait]
impl<'a> AbstractUseCase<String> for CalculateOneProcessUseCase<'a> {
    async fn execute(&self) -> Result<String, ApiError> {
        let recipe = self
            .repository
            .register_process(123, self.recipe_id_list.to_vec())
            .await;
        match recipe {
            Ok(_) => Ok(123.to_string()),
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
