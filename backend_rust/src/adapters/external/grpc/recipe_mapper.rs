use crate::adapters::external::grpc::cooking::{Recipe as RecipeMessage, RecipeWithSchedule};
use crate::adapters::external::grpc::step_mapper::StepMapper;
use crate::application::mapper::grpc_mapper::{EntityToGrpcRequestMapper, GrpcResponseToDtoMapper};
use crate::domain::{Recipe, ScheduledRecipeDto};

pub struct RecipeMapper {}

impl EntityToGrpcRequestMapper<Recipe, RecipeMessage> for RecipeMapper {
    fn map_to_request(entity: Recipe) -> RecipeMessage {
        let steps = entity
            .steps
            .into_iter()
            .map(StepMapper::map_to_request)
            .collect();
        RecipeMessage {
            id: entity.id,
            steps,
        }
    }
}

impl GrpcResponseToDtoMapper<RecipeWithSchedule, ScheduledRecipeDto> for RecipeMapper {
    fn map_to_dto(message: RecipeWithSchedule) -> ScheduledRecipeDto {
        let steps = message
            .steps
            .into_iter()
            .map(StepMapper::map_to_dto)
            .collect();
        ScheduledRecipeDto {
            recipe_id: message.id,
            steps,
        }
    }
}
