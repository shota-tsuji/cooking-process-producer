pub trait ApiMapper<Entity, Payload> {
    // Map an Entity to a Presenter
    fn to_api(entity: Entity) -> Payload;

    // Map a Payload to an Entity
    //fn to_entity(payload: Payload) -> Entity;
}