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
                    .table(UblDocument::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UblDocument::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UblDocument::ProjectId).integer().not_null())
                    .col(ColumnDef::new(UblDocument::FileId).string().not_null())
                    .col(ColumnDef::new(UblDocument::SupplierId).string().not_null())
                    .col(ColumnDef::new(UblDocument::DocumentId).string().not_null())
                    .col(
                        ColumnDef::new(UblDocument::DocumentType)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UblDocument::VoidedDocumentCode).string())
                    .col(ColumnDef::new(UblDocument::Sha256).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(UblDocument::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UblDocument::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UblDocument {
    Table,
    Id,
    ProjectId,
    FileId,
    DocumentType,
    DocumentId,
    SupplierId,
    VoidedDocumentCode,
    Sha256,
}
