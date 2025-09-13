use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // recipes
        manager
            .create_table(
                Table::create()
                    .table(Recipes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Recipes::Id).string_len(36).not_null().primary_key())
                    .col(ColumnDef::new(Recipes::Title).string_len(140).not_null())
                    .col(ColumnDef::new(Recipes::Description).string_len(1000))
                    .to_owned(),
            )
            .await?;

        // resources
        manager
            .create_table(
                Table::create()
                    .table(Resources::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Resources::Id).big_unsigned().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Resources::Name).string_len(140).not_null())
                    .col(ColumnDef::new(Resources::Amount).integer().not_null())
                    .to_owned(),
            )
            .await?;

        // processes
        manager
            .create_table(
                Table::create()
                    .table(Processes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Processes::Id).string_len(36).not_null().primary_key())
                    .col(ColumnDef::new(Processes::Name).string_len(140).not_null())
                    .to_owned(),
            )
            .await?;

        // steps
        manager
            .create_table(
                Table::create()
                    .table(Steps::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Steps::Id).string_len(36).not_null().primary_key())
                    .col(ColumnDef::new(Steps::RecipeId).string_len(36).not_null())
                    .col(ColumnDef::new(Steps::Description).string_len(140).not_null())
                    .col(ColumnDef::new(Steps::ResourceId).big_unsigned().not_null())
                    .col(ColumnDef::new(Steps::OrderNumber).integer().unsigned().not_null())
                    .col(ColumnDef::new(Steps::Duration).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Steps::Table, Steps::RecipeId)
                            .to(Recipes::Table, Recipes::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Steps::Table, Steps::ResourceId)
                            .to(Resources::Table, Resources::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // process_registrations
        manager
            .create_table(
                Table::create()
                    .table(ProcessRegistrations::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ProcessRegistrations::Id).big_unsigned().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(ProcessRegistrations::ProcessId).string_len(36).not_null())
                    .col(ColumnDef::new(ProcessRegistrations::RecipeId).string_len(36).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProcessRegistrations::Table, ProcessRegistrations::ProcessId)
                            .to(Processes::Table, Processes::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProcessRegistrations::Table, ProcessRegistrations::RecipeId)
                            .to(Recipes::Table, Recipes::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop in reverse order due to foreign key constraints
        manager
            .drop_table(Table::drop().table(ProcessRegistrations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Steps::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Processes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Resources::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Recipes::Table).to_owned())
            .await?;
        Ok(())
    }
}

// Table idents
#[derive(DeriveIden)]
enum Recipes {
    Table,
    Id,
    Title,
    Description,
}

#[derive(DeriveIden)]
enum Resources {
    Table,
    Id,
    Name,
    Amount,
}

#[derive(DeriveIden)]
enum Steps {
    Table,
    Id,
    RecipeId,
    Description,
    ResourceId,
    OrderNumber,
    Duration,
}

#[derive(DeriveIden)]
enum Processes {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum ProcessRegistrations {
    Table,
    Id,
    ProcessId,
    RecipeId,
}
