use anyhow::Result;
use rust_decimal::Decimal;

use crate::enricher::bounds::summary_documents::SummaryDocumentsComprobantesGetter;
use crate::enricher::Defaults;

/// SUNAT requires the IGV rate (as a percentage, e.g. 18) on every line, even
/// when the IGV is 0.
pub trait SummaryDocumentsIgvTasaFillRule {
    fn fill(&mut self, defaults: &Defaults) -> Result<bool>;
}

impl<T> SummaryDocumentsIgvTasaFillRule for T
where
    T: SummaryDocumentsComprobantesGetter,
{
    fn fill(&mut self, defaults: &Defaults) -> Result<bool> {
        let mut changed = false;
        for item in self.get_comprobantes() {
            let impuestos = &mut item.comprobante.impuestos;
            if impuestos.igv_tasa.is_none() {
                impuestos.igv_tasa = Some((defaults.igv_tasa * Decimal::ONE_HUNDRED).normalize());
                changed = true;
            }
        }
        Ok(changed)
    }
}
