use anyhow::Result;

use crate::catalogs::{code_for_name, Catalog19};
use crate::enricher::bounds::summary_documents::SummaryDocumentsComprobantesGetter;

/// Accepts each line's `tipo_operacion` by name, e.g. "ADICIONAR", and stores its code.
pub trait SummaryDocumentsTipoOperacionFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> SummaryDocumentsTipoOperacionFillRule for T
where
    T: SummaryDocumentsComprobantesGetter,
{
    fn fill(&mut self) -> Result<bool> {
        let mut changed = false;
        for item in self.get_comprobantes() {
            if let Some(code) = code_for_name::<Catalog19>(&item.tipo_operacion) {
                item.tipo_operacion = code;
                changed = true;
            }
        }
        Ok(changed)
    }
}
