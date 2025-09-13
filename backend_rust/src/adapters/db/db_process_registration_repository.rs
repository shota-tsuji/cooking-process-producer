use sea_orm::DatabaseConnection;

use crate::application::repository::process_registrations_repository::ProcessRegistrationRepository;
use crate::infrastructure::mysql::entity as db_entity;
use async_trait::async_trait;
use sea_orm::*;

pub struct DbProcessRegistrationRepository {
    pub db_connection: DatabaseConnection,
}
#[async_trait]
impl ProcessRegistrationRepository for DbProcessRegistrationRepository {
    async fn register_process(
        &self,
        process_id: u64,
        recipe_id_list: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let recipe_id_list: Vec<db_entity::process_regsitrations::ActiveModel> = recipe_id_list
            .iter()
            .map(|recipe_id| db_entity::process_regsitrations::ActiveModel {
                id: NotSet,
                process_id: Set(process_id),
                recipe_id: Set(recipe_id.clone()),
            })
            .collect();
        db_entity::process_regsitrations::Entity::insert_many(recipe_id_list)
            .exec(&self.db_connection)
            .await?;
        Ok(())
    }
}
