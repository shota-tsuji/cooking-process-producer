use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProcessSteps::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProcessSteps::Id)
                            .char_len(36) // changed from big_unsigned to string
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProcessSteps::ProcessId)
                            .string_len(36)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProcessSteps::StepId).char_len(36).not_null())
                    .col(
                        ColumnDef::new(ProcessSteps::StartTime)
                            .unsigned()
                            .not_null(), // changed from date_time to uint
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-process_steps-process_id")
                            .from(ProcessSteps::Table, ProcessSteps::ProcessId)
                            .to(Processes::Table, Processes::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-process_steps-step_id")
                            .from(ProcessSteps::Table, ProcessSteps::StepId)
                            .to(Steps::Table, Steps::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProcessSteps::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum ProcessSteps {
    Table,
    Id,
    ProcessId,
    StepId,
    StartTime,
}

#[derive(Iden)]
enum Processes {
    Table,
    Id,
}

#[derive(Iden)]
enum Steps {
    Table,
    Id,
}
