use sea_orm_migration::sea_orm::{entity::*, query::*};
use sea_orm_migration::prelude::*;
use crate::seed::entity::resources;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Insert a row using resources::ActiveModel
        let active_model = resources::ActiveModel {
            // Set your fields here, e.g.:
            // id: Set(1),
            // name: Set("Sample Resource".to_owned()),
            // ...other fields...
            ..Default::default()
        };
        resources::Entity::insert(active_model).exec(db).await?;

        Ok(())
    }
}
