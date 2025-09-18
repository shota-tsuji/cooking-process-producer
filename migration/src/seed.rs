use crate::{async_trait, base};
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub mod entity;
mod m20250917_105121_resource_repository_medium_test;

pub struct ResourceRepositoryMediumTestMigrator;

#[async_trait::async_trait]
impl MigratorTrait for ResourceRepositoryMediumTestMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations = base::base_migrations();
        migrations.push(Box::new(
            m20250917_105121_resource_repository_medium_test::Migration,
        ));
        migrations
    }
}

use sea_orm_cli::MigrateSubcommands;
use sea_orm_migration::cli;
use sea_orm_migration::prelude::*;

pub async fn seed_resource_repository_medium_test(db: &sea_orm::DatabaseConnection) {
    //cli::run_cli(migration::Migrator).await;

    // Run migration with Fresh command
    /*
    let db = sea_orm::Database::connect(std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("Failed to connect to database");
     */
    cli::run_migrate(
        ResourceRepositoryMediumTestMigrator,
        db,
        Some(MigrateSubcommands::Fresh),
        false,
    )
    .await
    .expect("Migration failed");
}
