use sea_orm_migration::prelude::*;
use sea_orm_migration::cli;
use sea_orm_cli::MigrateSubcommands;

#[async_std::main]
async fn main() {
    //cli::run_cli(migration::Migrator).await;

    // Run migration with Fresh command
    let db = sea_orm::Database::connect(std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("Failed to connect to database");
    cli::run_migrate(
        migration::Migrator,
        &db,
        Some(MigrateSubcommands::Fresh),
        false,
    )
    .await
    .expect("Migration failed");
}
