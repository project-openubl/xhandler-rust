use crate::system::InnerSystem;
use openubl_entity as entity;

pub struct UblDocumentContext {
    pub system: InnerSystem,
    pub ubl_document: entity::ubl_document::Model,
}

impl From<(&InnerSystem, entity::ubl_document::Model)> for UblDocumentContext {
    fn from((system, ubl_document): (&InnerSystem, entity::ubl_document::Model)) -> Self {
        Self {
            system: system.clone(),
            ubl_document,
        }
    }
}
