use crate::adapters::db::mysql::entity as db_entity;
use crate::application::repository::process_repository::ProcessRepository;
use crate::domain::error::AsyncDynError;
use async_trait::async_trait;
use sea_orm::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::sync::Arc;

pub struct DbProcessRepository {
    pub db_connection: Arc<DatabaseConnection>,
}
#[async_trait]
impl ProcessRepository for DbProcessRepository {
    async fn register_process(
        &self,
        process_id: String,
        recipe_id_list: Vec<String>,
    ) -> Result<(), Box<AsyncDynError>> {
        let process = db_entity::processes::ActiveModel {
            id: Set(process_id.clone()),
            name: Set("process".to_string()),
        };
        let _res = process.insert(&*self.db_connection).await.unwrap();
        //let process_id = _res.id;

        let recipe_id_list: Vec<db_entity::process_registrations::ActiveModel> = recipe_id_list
            .iter()
            .map(|recipe_id| db_entity::process_registrations::ActiveModel {
                id: NotSet,
                process_id: Set(process_id.clone()),
                recipe_id: Set(recipe_id.clone()),
            })
            .collect();
        db_entity::process_registrations::Entity::insert_many(recipe_id_list)
            .exec(&*self.db_connection)
            .await?;
        Ok(())
    }
}
