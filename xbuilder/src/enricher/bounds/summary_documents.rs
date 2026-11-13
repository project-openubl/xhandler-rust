use crate::models::summary_documents::{SummaryDocuments, SummaryDocumentsItem};

pub trait SummaryDocumentsComprobantesGetter {
    fn comprobantes_mut(&mut self) -> &mut Vec<SummaryDocumentsItem>;
}

pub trait SummaryDocumentsDocumentoIdGetter {
    fn documento_id(&self) -> &Option<String>;
}

pub trait SummaryDocumentsDocumentoIdSetter {
    fn set_documento_id(&mut self, val: String);
}

pub trait SummaryDocumentsNumeroGetter {
    fn numero(&self) -> u32;
}

impl SummaryDocumentsComprobantesGetter for SummaryDocuments {
    fn comprobantes_mut(&mut self) -> &mut Vec<SummaryDocumentsItem> {
        &mut self.comprobantes
    }
}

impl SummaryDocumentsDocumentoIdGetter for SummaryDocuments {
    fn documento_id(&self) -> &Option<String> {
        &self.documento_id
    }
}

impl SummaryDocumentsDocumentoIdSetter for SummaryDocuments {
    fn set_documento_id(&mut self, val: String) {
        self.documento_id = Some(val);
    }
}

impl SummaryDocumentsNumeroGetter for SummaryDocuments {
    fn numero(&self) -> u32 {
        self.numero
    }
}
