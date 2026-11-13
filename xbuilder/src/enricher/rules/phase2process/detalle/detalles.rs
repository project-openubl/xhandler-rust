use anyhow::Result;

use crate::enricher::bounds::detalle::DetallesGetter;
use crate::enricher::rules::phase2process::detalle::icb::DetalleIcbProcessRule;
use crate::enricher::rules::phase2process::detalle::icb_aplica::DetalleIcbAplicaProcessRule;
use crate::enricher::rules::phase2process::detalle::igv::DetalleIgvProcessRule;
use crate::enricher::rules::phase2process::detalle::igv_base_imponible::DetalleIgvBaseImponibleProcessRule;
use crate::enricher::rules::phase2process::detalle::isc::DetalleIscProcessRule;
use crate::enricher::rules::phase2process::detalle::isc_base_imponible::DetalleIscBaseImponibleProcessRule;
use crate::enricher::rules::phase2process::detalle::precio::DetallePrecioProcessRule;
use crate::enricher::rules::phase2process::detalle::precio_con_impuestos::DetallePrecioConImpuestosProcessRule;
use crate::enricher::rules::phase2process::detalle::precio_referencia::DetallePrecioReferenciaProcessRule;
use crate::enricher::rules::phase2process::detalle::total_impuestos::DetalleTotalImpuestosProcessRule;

pub trait DetallesProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetallesProcessRule for T
where
    T: DetallesGetter,
{
    fn process(&mut self) -> Result<bool> {
        let result = self.get_detalles().iter_mut().any(|detalle| {
            let results = [
                DetallePrecioProcessRule::process(detalle).unwrap_or_default(),
                DetallePrecioConImpuestosProcessRule::process(detalle).unwrap_or_default(),
                DetallePrecioReferenciaProcessRule::process(detalle).unwrap_or_default(),
            ];
            if results.contains(&true) {
                true
            } else {
                let results = [
                    DetalleIcbProcessRule::process(detalle).unwrap_or_default(),
                    DetalleIcbAplicaProcessRule::process(detalle).unwrap_or_default(),
                    DetalleIgvProcessRule::process(detalle).unwrap_or_default(),
                    DetalleIscProcessRule::process(detalle).unwrap_or_default(),
                    DetalleTotalImpuestosProcessRule::process(detalle).unwrap_or_default(),
                    DetalleIgvBaseImponibleProcessRule::process(detalle).unwrap_or_default(),
                    DetalleIscBaseImponibleProcessRule::process(detalle).unwrap_or_default(),
                ];
                results.contains(&true)
            }
        });
        Ok(result)
    }
}
