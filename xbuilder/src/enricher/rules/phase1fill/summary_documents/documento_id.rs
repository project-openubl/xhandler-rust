use anyhow::Result;

use crate::enricher::bounds::fecha_emision::FechaEmisionGetter;
use crate::enricher::bounds::summary_documents::{
    SummaryDocumentsDocumentoIdGetter, SummaryDocumentsDocumentoIdSetter,
    SummaryDocumentsNumeroGetter,
};

pub trait SummaryDocumentsDocumentoIdFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> SummaryDocumentsDocumentoIdFillRule for T
where
    T: SummaryDocumentsDocumentoIdGetter
        + SummaryDocumentsDocumentoIdSetter
        + SummaryDocumentsNumeroGetter
        + FechaEmisionGetter,
{
    fn fill(&mut self) -> Result<bool> {
        match (&self.get_documento_id(), &self.get_fecha_emision()) {
            (None, Some(fecha)) => {
                let id = format!("RC-{}-{}", fecha.format("%Y%m%d"), self.get_numero());
                self.set_documento_id(id);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
