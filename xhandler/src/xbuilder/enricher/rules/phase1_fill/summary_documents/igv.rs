use anyhow::Result;
use rust_decimal::Decimal;

use crate::xbuilder::enricher::bounds::summary_documents::SummaryDocumentsComprobantesGetter;
use crate::xbuilder::enricher::Defaults;

/// SUNAT requires the IGV and its rate on every line, even when the IGV is 0.
/// The IGV defaults to 0 and the rate to `Defaults.igv_tasa * 100`: the defaults
/// hold a fraction (0.18) while the line holds a percentage (18).
pub trait SummaryDocumentsIgvFillRule {
    fn fill(&mut self, defaults: &Defaults) -> Result<bool>;
}

impl<T> SummaryDocumentsIgvFillRule for T
where
    T: SummaryDocumentsComprobantesGetter,
{
    fn fill(&mut self, defaults: &Defaults) -> Result<bool> {
        let mut changed = false;
        for item in self.comprobantes_mut() {
            let impuestos = &mut item.comprobante.impuestos;
            if impuestos.igv.is_none() {
                impuestos.igv = Some(Decimal::ZERO);
                changed = true;
            }
            if impuestos.igv_tasa.is_none() {
                impuestos.igv_tasa = Some((defaults.igv_tasa * Decimal::ONE_HUNDRED).normalize());
                changed = true;
            }
        }
        Ok(changed)
    }
}
