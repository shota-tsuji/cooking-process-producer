use crate::seed::entity::resources;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Insert a row using resources::ActiveModel
        let resource = resources::ActiveModel {
            id: Set(1),
            name: Set("Sugar".to_string()),
            amount: Set(2),
        };
        resource.insert(db).await?;

        Ok(())
    }
}
