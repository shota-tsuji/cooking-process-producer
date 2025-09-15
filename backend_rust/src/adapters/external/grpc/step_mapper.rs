use crate::adapters::external::grpc::cooking::{Step as StepMessage, StepWithSchedule};
use crate::application::mapper::grpc_mapper::{EntityToGrpcRequestMapper, GrpcResponseToDtoMapper};
use crate::domain::{ScheduledStepDto, Step};

pub struct StepMapper {}

impl EntityToGrpcRequestMapper<Step, StepMessage> for StepMapper {
    fn map_to_request(entity: Step) -> StepMessage {
        StepMessage {
            id: entity.id,
            duration_minutes: entity.duration,
            resource_id: entity.resource_id.0.to_string(),
        }
    }
}

impl GrpcResponseToDtoMapper<StepWithSchedule, ScheduledStepDto> for StepMapper {
    fn map_to_dto(message: StepWithSchedule) -> ScheduledStepDto {
        let step = Step {
            id: message.id,
            duration: message.duration_minutes,
            ..Default::default()
        };
        ScheduledStepDto {
            step,
            start_time: message.start_time,
        }
    }
}
