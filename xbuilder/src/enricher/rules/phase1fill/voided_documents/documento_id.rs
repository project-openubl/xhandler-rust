use anyhow::Result;

use crate::enricher::bounds::fecha_emision::FechaEmisionGetter;
use crate::enricher::bounds::voided_documents::{
    VoidedDocumentsDocumentoIdGetter, VoidedDocumentsDocumentoIdSetter, VoidedDocumentsNumeroGetter,
};

pub trait VoidedDocumentsDocumentoIdFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> VoidedDocumentsDocumentoIdFillRule for T
where
    T: VoidedDocumentsDocumentoIdGetter
        + VoidedDocumentsDocumentoIdSetter
        + VoidedDocumentsNumeroGetter
        + FechaEmisionGetter,
{
    fn fill(&mut self) -> Result<bool> {
        match (&self.get_documento_id(), &self.get_fecha_emision()) {
            (None, Some(fecha)) => {
                let id = format!("RA-{}-{}", fecha.format("%Y%m%d"), self.get_numero());
                self.set_documento_id(id);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
