use anyhow::Result;

use crate::catalogs::{code_for_name, Catalog53};
use crate::enricher::bounds::invoice::cargos::InvoiceCargosGetter;

/// Accepts the cargo `tipo` by name, e.g. "CARGOS_GLOBALES_QUE_AFECTAN_BASE_IMPONIBLE_IGV_IVAP"
pub trait InvoiceCargosFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> InvoiceCargosFillRule for T
where
    T: InvoiceCargosGetter,
{
    fn fill(&mut self) -> Result<bool> {
        let mut changed = false;
        for cargo in self.cargos_mut() {
            if let Some(code) = code_for_name::<Catalog53>(&cargo.tipo) {
                cargo.tipo = code;
                changed = true;
            }
        }
        Ok(changed)
    }
}
