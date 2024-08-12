use anyhow::Result;

use crate::enricher::bounds::detalle::DetallesGetter;
use crate::enricher::rules::phase2process::detalle::icb::DetalleICBProcessRule;
use crate::enricher::rules::phase2process::detalle::icb_aplica::DetalleICBAplicaProcessRule;
use crate::enricher::rules::phase2process::detalle::igv::DetalleIGVProcessRule;
use crate::enricher::rules::phase2process::detalle::igv_base_imponible::DetalleIGVBaseImponibleProcessRule;
use crate::enricher::rules::phase2process::detalle::isc::DetalleISCProcessRule;
use crate::enricher::rules::phase2process::detalle::isc_base_imponible::DetalleISCBaseImponibleProcessRule;
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
        let result = self
            .get_detalles()
            .iter_mut()
            .map(|detalle| {
                let results = [
                    DetallePrecioProcessRule::process(detalle).map_or(false, |e| e),
                    DetallePrecioConImpuestosProcessRule::process(detalle).map_or(false, |e| e),
                    DetallePrecioReferenciaProcessRule::process(detalle).map_or(false, |e| e),
                ];
                if results.contains(&true) {
                    true
                } else {
                    let results = [
                        DetalleICBProcessRule::process(detalle).map_or(false, |e| e),
                        DetalleICBAplicaProcessRule::process(detalle).map_or(false, |e| e),
                        DetalleIGVProcessRule::process(detalle).map_or(false, |e| e),
                        DetalleISCProcessRule::process(detalle).map_or(false, |e| e),
                        DetalleTotalImpuestosProcessRule::process(detalle).map_or(false, |e| e),
                        DetalleIGVBaseImponibleProcessRule::process(detalle).map_or(false, |e| e),
                        DetalleISCBaseImponibleProcessRule::process(detalle).map_or(false, |e| e),
                    ];
                    results.contains(&true)
                }
            })
            .any(|changed| changed);
        Ok(result)
    }
}
