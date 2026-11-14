use anyhow::Result;

use crate::xbuilder::enricher::bounds::moneda::{MonedaGetter, MonedaSetter};
use crate::xbuilder::enricher::bounds::summary_documents::SummaryDocumentsComprobantesGetter;

pub trait SummaryDocumentsMonedaFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> SummaryDocumentsMonedaFillRule for T
where
    T: MonedaGetter + MonedaSetter + SummaryDocumentsComprobantesGetter,
{
    fn fill(&mut self) -> Result<bool> {
        let mut changed = false;

        if self.moneda().is_none() {
            self.set_moneda("PEN");
            changed = true;
        }

        let doc_moneda = self.moneda().clone();
        for item in self.comprobantes_mut() {
            if item.comprobante.moneda.is_none() {
                item.comprobante.moneda = doc_moneda.clone();
                changed = true;
            }
        }

        Ok(changed)
    }
}
