use crate::adapters::grpc::cooking::{Step as StepMessage, StepWithSchedule};
use crate::adapters::grpc::resource_mapper::ResourceMapper;
use crate::application::mapper::grpc_mapper::{EntityToGrpcRequestMapper, GrpcResponseToDtoMapper};
use crate::domain::{ScheduledStepDto, Step};

pub struct StepMapper {}

impl EntityToGrpcRequestMapper<Step, StepMessage> for StepMapper {
    fn map_to_request(entity: Step) -> StepMessage {
        let resource = Some(ResourceMapper::map_to_request(entity.resource));
        StepMessage {
            id: entity.id,
            duration_minutes: entity.duration,
            resource,
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
            resource: ResourceMapper::map_to_dto(message.resource.unwrap()),
        }
    }
}
