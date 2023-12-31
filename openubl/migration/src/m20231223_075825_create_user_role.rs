use sea_orm_migration::prelude::*;

use crate::m20231223_071007_create_project::Project;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRole::UserId).string().not_null())
                    .col(ColumnDef::new(UserRole::ProjectId).integer().not_null())
                    .col(ColumnDef::new(UserRole::Role).string().not_null())
                    .primary_key(
                        Index::create()
                            .col(UserRole::UserId)
                            .col(UserRole::ProjectId)
                            .col(UserRole::Role),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(UserRole::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserRole {
    Table,
    UserId,
    ProjectId,
    Role,
}
