use crate::adapters::grpc::cooking::CalculateProcessRequest;
use crate::adapters::grpc::cooking::Recipe as RecipeMessage;
use crate::adapters::grpc::cooking::process_service_client::ProcessServiceClient as GrpcClient;
use crate::adapters::grpc::recipe_mapper::RecipeMapper;
use crate::application::mapper::grpc_mapper::{EntityToGrpcRequestMapper, GrpcResponseToDtoMapper};
use crate::application::port::ProcessServicePort;
use crate::domain::error::AsyncDynError;
use crate::domain::{Recipe, ScheduledRecipeDto};
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
    ) -> Result<Vec<ScheduledRecipeDto>, Box<AsyncDynError>> {
        let recipe_messages: Vec<RecipeMessage> = recipes
            .into_iter()
            .map(RecipeMapper::map_to_request)
            .collect();
        let request = tonic::Request::new(CalculateProcessRequest {
            recipes: recipe_messages,
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
