use crate::adapters::external::grpc::cooking::CalculateProcessRequest;
use crate::adapters::external::grpc::cooking::Recipe as RecipeMessage;
use crate::adapters::external::grpc::cooking::Resource as ResourceMessage;
use crate::adapters::external::grpc::cooking::process_service_client::ProcessServiceClient as GrpcClient;
use crate::adapters::external::grpc::recipe_mapper::RecipeMapper;
use crate::adapters::external::grpc::resource_mapper::ResourceMapper;
use crate::application::mapper::grpc_mapper::{EntityToGrpcRequestMapper, GrpcResponseToDtoMapper};
use crate::application::port::ProcessServicePort;
use crate::domain::error::AsyncDynError;
use crate::domain::{Recipe, Resource, ScheduledRecipeDto};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct GrpcProcessServiceClient {
    pub client: Arc<Mutex<GrpcClient<tonic::transport::Channel>>>,
}

#[async_trait]
impl ProcessServicePort for GrpcProcessServiceClient {
    async fn calculate_process(
        &self,
        recipes: Vec<Recipe>,
        resources: Vec<Resource>,
    ) -> Result<Vec<ScheduledRecipeDto>, Box<AsyncDynError>> {
        let recipe_messages: Vec<RecipeMessage> = recipes
            .into_iter()
            .map(RecipeMapper::map_to_request)
            .collect();
        let resource_messages: Vec<ResourceMessage> = resources
            .into_iter()
            .map(ResourceMapper::map_to_request)
            .collect();
        let request = tonic::Request::new(CalculateProcessRequest {
            recipes: recipe_messages,
            resources: resource_messages,
        });

        let mut client = self.client.lock().await;
        let response = client.calculate_process(request).await?;

        let scheduled_recipes: Vec<ScheduledRecipeDto> = response
            .into_inner()
            .recipes
            .into_iter()
            .map(RecipeMapper::map_to_dto)
            .collect();
        Ok(scheduled_recipes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::external::grpc::cooking::process_service_client::ProcessServiceClient;
    use crate::domain::entity::resource::ResourceId;
    use crate::domain::{Recipe, Resource, Step};
    use std::sync::Arc;
    use testcontainers_modules::testcontainers::GenericImage;
    use testcontainers_modules::testcontainers::core::WaitFor;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;
    use tokio::sync::Mutex;
    use tonic::transport::Channel;

    #[tokio::test]
    async fn test_get_scheduled_steps() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Start backend_python container with testcontainers
        let image = GenericImage::new("main", "0.1.1")
            .with_exposed_port(
                testcontainers_modules::testcontainers::core::ContainerPort::Tcp(50051),
            )
            .with_exposed_port(
                testcontainers_modules::testcontainers::core::ContainerPort::Tcp(8081),
            )
            .with_wait_for(WaitFor::healthcheck());
        let node = AsyncRunner::start(image).await?;
        let port = node.get_host_port_ipv4(50051).await?;
        let host = node.get_host().await?;
        let endpoint = format!("http://{}:{}", host, port);

        // Connect gRPC client
        let channel = Channel::from_shared(endpoint)?.connect().await?;
        let client = Arc::new(Mutex::new(ProcessServiceClient::new(channel)));
        let grpc_client = GrpcProcessServiceClient { client };

        let recipes = vec![
            Recipe {
                id: "abc".to_string(),
                name: "Pancake".to_string(),
                description: "".to_string(),
                steps: vec![
                    Step {
                        id: "abc".to_string(),
                        description: "Mix ingredients".to_string(),
                        duration: 0,
                        order: 1,
                        resource_id: Default::default(),
                    },
                    Step {
                        id: "2".to_string(),
                        description: "Cook on pan".to_string(),
                        duration: 0,
                        order: 2,
                        resource_id: Default::default(),
                    },
                ],
                // ...other fields if needed...
            },
            Recipe {
                id: "2".to_string(),
                name: "Omelette".to_string(),
                description: "".to_string(),
                steps: vec![
                    Step {
                        id: "3".to_string(),
                        description: "Beat eggs".to_string(),
                        duration: 0,
                        order: 1,
                        resource_id: Default::default(),
                    },
                    Step {
                        id: "4".to_string(),
                        description: "Cook in pan".to_string(),
                        duration: 0,
                        order: 2,
                        resource_id: Default::default(),
                    },
                ],
            },
        ];
        let resources = vec![Resource {
            id: ResourceId(1),
            name: "Pan".to_string(),
            amount: 1,
        }];

        // Act: call calculate_process
        let scheduled = grpc_client.calculate_process(recipes, resources).await?;

        // Assert: contents and order of recipes and steps
        assert_eq!(scheduled.len(), 2);
        assert_eq!(scheduled[0].recipe_id, "1");
        assert_eq!(scheduled[1].recipe_id, "2");
        assert_eq!(scheduled[0].steps[0].step.id, "a");
        assert_eq!(scheduled[0].steps[0].start_time, 1000);
        assert_eq!(scheduled[1].steps[0].start_time, 2000);
        Ok(())
    }
}
