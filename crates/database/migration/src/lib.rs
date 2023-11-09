pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_version_table;
mod m20231108_080026_create_settings;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_version_table::Migration),
            Box::new(m20231108_080026_create_settings::Migration),
        ]
    }
}
