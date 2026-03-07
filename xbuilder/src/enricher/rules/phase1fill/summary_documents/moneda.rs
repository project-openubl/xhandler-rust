use anyhow::Result;

use crate::enricher::bounds::summary_documents::{
    SummaryDocumentsComprobantesGetter, SummaryDocumentsMonedaGetter, SummaryDocumentsMonedaSetter,
};

pub trait SummaryDocumentsMonedaFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> SummaryDocumentsMonedaFillRule for T
where
    T: SummaryDocumentsMonedaGetter
        + SummaryDocumentsMonedaSetter
        + SummaryDocumentsComprobantesGetter,
{
    fn fill(&mut self) -> Result<bool> {
        let mut changed = false;

        if self.get_moneda().is_none() {
            self.set_moneda("PEN");
            changed = true;
        }

        let doc_moneda = self.get_moneda().clone();
        for item in self.get_comprobantes() {
            if item.comprobante.moneda.is_none() {
                item.comprobante.moneda = doc_moneda.clone();
                changed = true;
            }
        }

        Ok(changed)
    }
}
