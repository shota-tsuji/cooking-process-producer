pub trait EntityToGrpcRequestMapper<Entity, Request> {
    fn map_to_request(entity: Entity) -> Request;
}

pub trait GrpcResponseToDtoMapper<Response, Dto> {
    fn map_to_dto(message: Response) -> Dto;
}
