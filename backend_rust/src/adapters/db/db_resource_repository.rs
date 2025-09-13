use crate::adapters::db::mysql::entity as db_entity;
use crate::application::repository::resource_repository::ResourceRepository;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::sync::Arc;

pub struct DbResourceRepository {
    pub db_connection: Arc<DatabaseConnection>,
}

#[async_trait]
impl ResourceRepository for DbResourceRepository {
    async fn get_resource_by_id(
        &self,
        id: i32,
    ) -> Result<crate::domain::resource::Resource, Box<dyn std::error::Error>> {
        let model = db_entity::resources::Entity::find_by_id(id as u64)
            .one(&*self.db_connection)
            .await;

        match model {
            Ok(Some(model)) => {
                let resource = crate::domain::resource::Resource {
                    id: model.id as i32,
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

    async fn get_all_resources(
        &self,
    ) -> Result<Vec<crate::domain::resource::Resource>, Box<dyn std::error::Error>> {
        let models = db_entity::resources::Entity::find()
            .all(&*self.db_connection)
            .await;

        match models {
            Ok(models) => {
                let models = models
                    .into_iter()
                    .map(|model| crate::domain::resource::Resource {
                        id: model.id as i32,
                        name: model.name,
                        amount: model.amount,
                    })
                    .collect::<Vec<crate::domain::resource::Resource>>();
                Ok(models)
            }
            Err(e) => Err(Box::new(std::io::Error::other(e))),
        }
    }
}
