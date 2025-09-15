use crate::adapters::repository::mysql::entity::steps::Model as StepModel;
use crate::application::mapper::db_mapper::DbMapper;
use crate::domain::Step;
use crate::domain::entity::resource::ResourceId;
pub struct MysqlStepMapper {}

impl DbMapper<Step, StepModel> for MysqlStepMapper {
    fn to_db(_entity: Step) -> StepModel {
        unimplemented!()
    }

    fn to_entity(model: StepModel) -> Step {
        Step {
            id: model.id,
            description: model.description,
            order: model.order_number,
            duration: model.duration,
            resource_id: ResourceId(model.resource_id as i32),
        }
    }
}
