use crate::enricher::rules::phase2process::detalle::icb::DetalleICBProcessRule;
use crate::enricher::rules::phase2process::detalle::igv::DetalleIGVProcessRule;
use crate::enricher::rules::phase2process::detalle::igvbaseimponible::DetalleIGVBaseImponibleProcessRule;
use crate::enricher::rules::phase2process::detalle::isc::DetalleISCProcessRule;
use crate::enricher::rules::phase2process::detalle::iscbaseimponible::DetalleISCBaseImponibleProcessRule;
use crate::enricher::rules::phase2process::detalle::precio::DetallePrecioProcessRule;
use crate::enricher::rules::phase2process::detalle::precioconimpuestos::DetallePrecioConImpuestosProcessRule;
use crate::enricher::rules::phase2process::detalle::precioreferencia::DetallePrecioReferenciaProcessRule;
use crate::enricher::rules::phase2process::detalle::totalimpuestos::DetalleTotalImpuestosProcessRule;
use crate::models::traits::detalle::DetallesGetter;

pub trait DetallesProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetallesProcessRule for T
where
    T: DetallesGetter,
{
    fn process(&mut self) -> bool {
        self.get_detalles()
            .iter_mut()
            .map(|detalle| {
                let results = vec![
                    DetallePrecioProcessRule::process(detalle),
                    DetallePrecioConImpuestosProcessRule::process(detalle),
                    DetalleICBProcessRule::process(detalle),
                    DetalleIGVProcessRule::process(detalle),
                    DetalleISCProcessRule::process(detalle),
                    DetalleTotalImpuestosProcessRule::process(detalle),
                    DetallePrecioReferenciaProcessRule::process(detalle),
                    DetalleIGVBaseImponibleProcessRule::process(detalle),
                    DetalleISCBaseImponibleProcessRule::process(detalle),
                ];
                results.contains(&true)
            })
            .any(|changed| changed)
    }
}
