use anyhow::Result;

use crate::enricher::bounds::voided_documents::VoidedDocumentsComprobantesGetter;

pub trait VoidedDocumentsTipoComprobanteFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> VoidedDocumentsTipoComprobanteFillRule for T
where
    T: VoidedDocumentsComprobantesGetter,
{
    fn fill(&mut self) -> Result<bool> {
        let mut changed = false;
        for item in self.get_comprobantes() {
            if item.tipo_comprobante.is_none() {
                item.tipo_comprobante = match item.serie.chars().next() {
                    Some('F') | Some('f') => Some("01"),
                    Some('B') | Some('b') => Some("03"),
                    _ => None,
                };
                if item.tipo_comprobante.is_some() {
                    changed = true;
                }
            }
        }
        Ok(changed)
    }
}
