use crate::adapters::repository::mysql::entity::resources::Model as ResourceModel;
use crate::application::mapper::db_mapper::DbMapper;
use crate::domain::Resource;
use crate::domain::entity::resource::ResourceId;
pub struct ResourceMapper {}

impl DbMapper<Resource, ResourceModel> for ResourceMapper {
    fn to_db(_entity: Resource) -> ResourceModel {
        unimplemented!()
    }

    fn to_entity(model: ResourceModel) -> Resource {
        Resource {
            id: ResourceId(model.id as i32),
            name: model.name,
            amount: model.amount,
        }
    }
}
