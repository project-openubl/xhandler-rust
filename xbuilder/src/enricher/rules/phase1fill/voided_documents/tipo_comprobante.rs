use anyhow::Result;

use crate::catalogs::{Catalog, Catalog1};

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
                let catalog = match item.serie.chars().next().map(|c| c.to_ascii_uppercase()) {
                    Some('F') => Some(Catalog1::Factura),
                    Some('B') => Some(Catalog1::Boleta),
                    Some('T') => Some(Catalog1::GuiaRemisionRemitente),
                    Some('V') => Some(Catalog1::GuiaRemisionTransportista),
                    Some('P') => Some(Catalog1::Percepcion),
                    Some('R') => Some(Catalog1::Retencion),
                    _ => None,
                };
                item.tipo_comprobante = catalog.map(|catalog| catalog.code().to_string());
                if item.tipo_comprobante.is_some() {
                    changed = true;
                }
            }
        }
        Ok(changed)
    }
}
