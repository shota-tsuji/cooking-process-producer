pub mod m20220101_000001_create_table;
pub mod m20250909_110532_add_process_steps_table;

use sea_orm_migration::prelude::*;

/// Returns the list of migrations for use in seed and elsewhere.
pub fn base_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20220101_000001_create_table::Migration),
        Box::new(m20250909_110532_add_process_steps_table::Migration),
    ]
}
