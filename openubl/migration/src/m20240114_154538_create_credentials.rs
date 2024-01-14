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
                    .table(Credentials::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Credentials::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Credentials::Name).string().not_null())
                    .col(ColumnDef::new(Credentials::UsernameSol).string().not_null())
                    .col(ColumnDef::new(Credentials::PasswordSol).string().not_null())
                    .col(ColumnDef::new(Credentials::ClientId).string().not_null())
                    .col(
                        ColumnDef::new(Credentials::ClientSecret)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Credentials::ProjectId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(Credentials::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .col(Credentials::Name)
                            .col(Credentials::ProjectId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Credentials::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Credentials {
    Table,
    Id,
    Name,
    UsernameSol,
    PasswordSol,
    ClientId,
    ClientSecret,
    ProjectId,
}
