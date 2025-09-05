use crate::application::repository::resource_repository::ResourceRepository;
use crate::application::usecase::interface::AbstractUseCase;
use crate::domain::error::ApiError;
use crate::domain::resource::Resource;

pub struct GetOneResourceByIdUseCase<'a> {
    // Add fields as necessary, e.g., repository references
    pub resource_id: &'a i32,
    pub repository: &'a dyn ResourceRepository,
}

impl<'a> GetOneResourceByIdUseCase<'a> {
    pub fn new(resource_id: &'a i32, repository: &'a dyn ResourceRepository) -> Self {
        Self {
            resource_id,
            repository,
        }
    }
}

impl<'a> AbstractUseCase<Resource> for GetOneResourceByIdUseCase<'a> {
    async fn execute(&self) -> Result<Resource, ApiError> {
        // Implement the logic to get one resource by ID using the repository
        let resource = self.repository.get_resource_by_id(*self.resource_id).await;
        match resource {
            Ok(resource) => Ok(resource),
            Err(e) => {
                let e = ApiError {
                    code: 400,
                    message: String::from("Cannot get single resource"),
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
        application::repository::resource_repository::MockResourceRepository,
        domain::resource::Resource,
    };

    #[tokio::test]
    async fn test_should_return_one_result() {
        // given the "one resource by id" usecase repo returning one result
        let mut resource_repository = MockResourceRepository::new();
        resource_repository
            .expect_get_resource_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| {
                Ok(Resource {
                    id: 1,
                    name: String::from("Resource1"),
                    amount: 10,
                })
            });

        // when calling usecase
        let get_one_resource_by_id_usecase =
            GetOneResourceByIdUseCase::new(&1, &resource_repository);
        let data = get_one_resource_by_id_usecase.execute().await.unwrap();

        // then one result
        assert_eq!(1, data.id);
        assert_eq!("Resource1", data.name);
        assert_eq!(10, data.amount);
    }
}
