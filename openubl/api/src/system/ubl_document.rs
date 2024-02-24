use openubl_entity as entity;

use crate::system::InnerSystem;

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

impl UblDocumentContext {
    // fn send_to_sunat(credentials: &entity::credentials::Model) {
    //     FileSender {
    //         urls: Urls {
    //             invoice: credentials.url_invoice.clone(),
    //             perception_retention: credentials.url_perception_retention.clone(),
    //             despatch: credentials.url_despatch.clone(),
    //         },
    //         credentials: Credentials {
    //             username: credentials.username_sol.clone(),
    //             password: credentials.password_sol.clone(),
    //         },
    //     };
    // }
}
