use crate::application::repository::resource_repository::ResourceRepository;
use crate::application::usecase::interface::AbstractUseCase;
use crate::domain::Resource;
use async_trait::async_trait;

pub struct GetAllResourcesUsecase<'a> {
    pub repository: &'a dyn ResourceRepository,
}

impl<'a> GetAllResourcesUsecase<'a> {
    pub fn new(repository: &'a dyn ResourceRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<'a> AbstractUseCase<Vec<Resource>> for GetAllResourcesUsecase<'a> {
    async fn execute(&self) -> Result<Vec<Resource>, crate::domain::error::ApiError> {
        let resources = self.repository.get_all_resources().await;
        match resources {
            Ok(resources) => Ok(resources),
            Err(e) => {
                let e = crate::domain::error::ApiError {
                    code: 400,
                    message: String::from("Cannot get all resources"),
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

    use crate::application::repository::resource_repository::MockResourceRepository;
    use crate::domain::Resource;

    #[tokio::test]
    async fn test_should_return_all_results() {
        let mut resource_repository = MockResourceRepository::new();
        resource_repository
            .expect_get_all_resources()
            .times(1)
            .returning(|| {
                Ok(vec![
                    Resource {
                        id: 1,
                        name: String::from("Resource 1"),
                        amount: 10,
                    },
                    Resource {
                        id: 2,
                        name: String::from("Resource 2"),
                        amount: 20,
                    },
                ])
            });

        let get_all_resources_usecase = GetAllResourcesUsecase::new(&resource_repository);
        let result = get_all_resources_usecase.execute().await.unwrap();

        let resource1 = Resource {
            id: 1,
            name: String::from("Resource 1"),
            amount: 10,
        };
        let resource2 = Resource {
            id: 2,
            name: String::from("Resource 2"),
            amount: 20,
        };
        assert_eq!(vec![resource1, resource2], result);
    }
}
