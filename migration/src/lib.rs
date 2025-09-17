pub use sea_orm_migration::prelude::*;

pub mod base;
pub mod seed;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        base::base_migrations()
    }
}
