use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Project::Name)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Project::Description).string())
                    .col(ColumnDef::new(Project::SunatUsername).string().not_null())
                    .col(ColumnDef::new(Project::SunatPassword).string().not_null())
                    .col(ColumnDef::new(Project::SunatFacturaUrl).string().not_null())
                    .col(ColumnDef::new(Project::SunatGuiaUrl).string().not_null())
                    .col(
                        ColumnDef::new(Project::SunatPercepcionRetencionUrl)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Project {
    Table,
    Name,
    Description,
    SunatUsername,
    SunatPassword,
    SunatFacturaUrl,
    SunatGuiaUrl,
    SunatPercepcionRetencionUrl,
}
