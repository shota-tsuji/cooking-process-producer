use crate::application::mapper::api_mapper::ApiMapper;
use crate::domain::step::Step;
use crate::presentation::graphql::object::Step as StepObject;
pub struct StepMapper {}

impl ApiMapper<Step, StepObject> for StepMapper {
    fn to_api(entity: Step) -> StepObject {
        StepObject {
            id: entity.id,
            description: entity.description,
            resource_id: entity.resource.id as u64,
            order_number: entity.order,
            duration: entity.duration,
        }
    }
}
