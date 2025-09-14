use crate::adapters::grpc::cooking::Resource as ResourceMessage;
use crate::application::mapper::grpc_mapper::{EntityToGrpcRequestMapper, GrpcResponseToDtoMapper};
use crate::domain::Resource;
use crate::domain::dto::resource_dto::ResourceDto;

pub struct ResourceMapper {}

impl EntityToGrpcRequestMapper<Resource, ResourceMessage> for ResourceMapper {
    fn map_to_request(entity: Resource) -> ResourceMessage {
        ResourceMessage {
            id: entity.id.to_string(),
        }
    }
}

impl GrpcResponseToDtoMapper<ResourceMessage, ResourceDto> for ResourceMapper {
    fn map_to_dto(message: ResourceMessage) -> ResourceDto {
        ResourceDto { id: message.id }
    }
}
