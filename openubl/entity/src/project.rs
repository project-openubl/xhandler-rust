//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "project")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::credentials::Entity")]
    Credentials,
    #[sea_orm(has_many = "super::keystore::Entity")]
    Keystore,
    #[sea_orm(has_many = "super::send_rule::Entity")]
    SendRule,
    #[sea_orm(has_many = "super::ubl_document::Entity")]
    UblDocument,
    #[sea_orm(has_many = "super::user_role::Entity")]
    UserRole,
}

impl Related<super::credentials::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Credentials.def()
    }
}

impl Related<super::keystore::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Keystore.def()
    }
}

impl Related<super::send_rule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SendRule.def()
    }
}

impl Related<super::ubl_document::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UblDocument.def()
    }
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
