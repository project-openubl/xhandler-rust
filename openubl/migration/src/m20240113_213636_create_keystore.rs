use crate::m20231223_071007_create_project::Project;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Keystore::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Keystore::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Keystore::Name).string().not_null())
                    .col(ColumnDef::new(Keystore::ProjectId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(Keystore::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .col(Keystore::Name)
                            .col(Keystore::ProjectId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Keystore::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Keystore {
    Table,
    Id,
    Name,
    ProjectId,
}
