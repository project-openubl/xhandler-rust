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
                    DetallePrecioProcessRule::process(detalle).unwrap_or_default(),
                    DetallePrecioConImpuestosProcessRule::process(detalle).unwrap_or_default(),
                    DetallePrecioReferenciaProcessRule::process(detalle).unwrap_or_default(),
                ];
                if results.contains(&true) {
                    true
                } else {
                    let results = [
                        DetalleICBProcessRule::process(detalle).unwrap_or_default(),
                        DetalleICBAplicaProcessRule::process(detalle).unwrap_or_default(),
                        DetalleIGVProcessRule::process(detalle).unwrap_or_default(),
                        DetalleISCProcessRule::process(detalle).unwrap_or_default(),
                        DetalleTotalImpuestosProcessRule::process(detalle).unwrap_or_default(),
                        DetalleIGVBaseImponibleProcessRule::process(detalle).unwrap_or_default(),
                        DetalleISCBaseImponibleProcessRule::process(detalle).unwrap_or_default(),
                    ];
                    results.contains(&true)
                }
            })
            .any(|changed| changed);
        Ok(result)
    }
}
