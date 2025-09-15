use crate::application::mapper::api_mapper::ApiMapper;
use crate::domain::Step;
use crate::adapters::controller::graphql::object::Step as StepObject;
pub struct StepMapper {}

impl ApiMapper<Step, StepObject> for StepMapper {
    fn to_api(entity: Step) -> StepObject {
        StepObject {
            id: entity.id,
            description: entity.description,
            resource_id: entity.resource_id.0 as u64,
            order_number: entity.order,
            duration: entity.duration,
        }
    }
}
