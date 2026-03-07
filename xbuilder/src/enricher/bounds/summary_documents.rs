use crate::models::summary_documents::{SummaryDocuments, SummaryDocumentsItem};

pub trait SummaryDocumentsComprobantesGetter {
    fn get_comprobantes(&mut self) -> &mut Vec<SummaryDocumentsItem>;
}

pub trait SummaryDocumentsMonedaGetter {
    fn get_moneda(&self) -> &Option<String>;
}

pub trait SummaryDocumentsMonedaSetter {
    fn set_moneda(&mut self, val: &str);
}

pub trait SummaryDocumentsDocumentoIdGetter {
    fn get_documento_id(&self) -> &Option<String>;
}

pub trait SummaryDocumentsDocumentoIdSetter {
    fn set_documento_id(&mut self, val: String);
}

pub trait SummaryDocumentsNumeroGetter {
    fn get_numero(&self) -> u32;
}

impl SummaryDocumentsComprobantesGetter for SummaryDocuments {
    fn get_comprobantes(&mut self) -> &mut Vec<SummaryDocumentsItem> {
        &mut self.comprobantes
    }
}

impl SummaryDocumentsMonedaGetter for SummaryDocuments {
    fn get_moneda(&self) -> &Option<String> {
        &self.moneda
    }
}

impl SummaryDocumentsMonedaSetter for SummaryDocuments {
    fn set_moneda(&mut self, val: &str) {
        self.moneda = Some(val.to_string());
    }
}

impl SummaryDocumentsDocumentoIdGetter for SummaryDocuments {
    fn get_documento_id(&self) -> &Option<String> {
        &self.documento_id
    }
}

impl SummaryDocumentsDocumentoIdSetter for SummaryDocuments {
    fn set_documento_id(&mut self, val: String) {
        self.documento_id = Some(val);
    }
}

impl SummaryDocumentsNumeroGetter for SummaryDocuments {
    fn get_numero(&self) -> u32 {
        self.numero
    }
}
