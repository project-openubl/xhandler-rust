pub use sea_orm_migration::prelude::*;

mod m20231223_071007_create_project;
mod m20231223_075825_create_user_role;
mod m20240101_104121_create_ubl_document;
mod m20240113_213636_create_keystore;
mod m20240113_213657_create_keystore_config;
mod m20240114_154538_create_credentials;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231223_071007_create_project::Migration),
            Box::new(m20231223_075825_create_user_role::Migration),
            Box::new(m20240101_104121_create_ubl_document::Migration),
            Box::new(m20240113_213636_create_keystore::Migration),
            Box::new(m20240113_213657_create_keystore_config::Migration),
            Box::new(m20240114_154538_create_credentials::Migration),
        ]
    }
}
