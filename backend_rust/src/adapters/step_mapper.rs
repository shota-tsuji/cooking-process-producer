use crate::adapters::db::mysql::entity::steps::Model as StepModel;
use crate::application::mapper::api_mapper::ApiMapper;
use crate::application::mapper::db_mapper::DbMapper;
use crate::domain::resource::Resource;
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

impl DbMapper<Step, StepModel> for StepMapper {
    fn to_db(_entity: Step) -> StepModel {
        unimplemented!()
    }

    fn to_entity(model: StepModel) -> Step {
        Step {
            id: model.id,
            description: model.description,
            order: model.order_number,
            duration: model.duration,
            resource: Resource {
                id: 1,
                name: String::new(),
                amount: 0,
            },
        }
    }
}
