use crate::adapters::repository::mysql::entity as db_entity;
use crate::adapters::repository::mysql::resource_mapper::ResourceMapper;
use crate::application::mapper::db_mapper::DbMapper;
use crate::application::port::repository::ResourceRepository;
use crate::domain::Resource;
use crate::domain::entity::resource::ResourceId;
use crate::domain::error::AsyncDynError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::sync::Arc;

pub struct MysqlResourceRepository {
    pub db_connection: Arc<DatabaseConnection>,
}

#[async_trait]
impl ResourceRepository for MysqlResourceRepository {
    async fn get_resource_by_id(&self, id: i32) -> Result<Resource, Box<AsyncDynError>> {
        let model = db_entity::resources::Entity::find_by_id(id as u64)
            .one(&*self.db_connection)
            .await;

        match model {
            Ok(Some(model)) => {
                let resource = Resource {
                    id: ResourceId(model.id as i32),
                    name: model.name,
                    amount: model.amount,
                };
                Ok(resource)
            }
            Err(e) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                e,
            ))),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Resource not found",
            ))),
        }
    }

    async fn get_resources_by_ids(
        &self,
        resource_ids: Vec<ResourceId>,
    ) -> Result<Vec<Resource>, Box<AsyncDynError>> {
        use db_entity::resources;
        let ids: Vec<u64> = resource_ids.iter().map(|rid| rid.0 as u64).collect();
        let models = resources::Entity::find()
            .filter(resources::Column::Id.is_in(ids))
            .all(&*self.db_connection)
            .await
            .map_err(|e| Box::new(std::io::Error::other(e)))?;

        let resources: Vec<Resource> = models.into_iter().map(ResourceMapper::to_entity).collect();
        Ok(resources)
    }

    async fn get_all_resources(&self) -> Result<Vec<Resource>, Box<AsyncDynError>> {
        let models = db_entity::resources::Entity::find()
            .all(&*self.db_connection)
            .await;

        match models {
            Ok(models) => {
                let models = models
                    .into_iter()
                    .map(|model| Resource {
                        id: ResourceId(model.id as i32),
                        name: model.name,
                        amount: model.amount,
                    })
                    .collect::<Vec<Resource>>();
                Ok(models)
            }
            Err(e) => Err(Box::new(std::io::Error::other(e))),
        }
    }
}
