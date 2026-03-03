use crate::models::voided_documents::{VoidedDocuments, VoidedDocumentsItem};

pub trait VoidedDocumentsComprobantesGetter {
    fn get_comprobantes(&mut self) -> &mut Vec<VoidedDocumentsItem>;
}

pub trait VoidedDocumentsDocumentoIdGetter {
    fn get_documento_id(&self) -> &Option<String>;
}

pub trait VoidedDocumentsDocumentoIdSetter {
    fn set_documento_id(&mut self, val: String);
}

pub trait VoidedDocumentsNumeroGetter {
    fn get_numero(&self) -> u32;
}

impl VoidedDocumentsComprobantesGetter for VoidedDocuments {
    fn get_comprobantes(&mut self) -> &mut Vec<VoidedDocumentsItem> {
        &mut self.comprobantes
    }
}

impl VoidedDocumentsDocumentoIdGetter for VoidedDocuments {
    fn get_documento_id(&self) -> &Option<String> {
        &self.documento_id
    }
}

impl VoidedDocumentsDocumentoIdSetter for VoidedDocuments {
    fn set_documento_id(&mut self, val: String) {
        self.documento_id = Some(val);
    }
}

impl VoidedDocumentsNumeroGetter for VoidedDocuments {
    fn get_numero(&self) -> u32 {
        self.numero
    }
}
