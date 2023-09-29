use crate::catalogs::catalog7_value_of_code;
use crate::models::traits::detalle::cantidad::DetalleCantidadGetter;
use crate::models::traits::detalle::igvtipo::DetalleIGVTipoGetter;
use crate::models::traits::detalle::iscbaseimponible::{
    DetalleISCBaseImponibleGetter, DetalleISCBaseImponibleSetter,
};
use crate::models::traits::detalle::precio::DetallePrecioGetter;
use crate::models::traits::detalle::precioreferencia::DetallePrecioReferenciaGetter;

pub trait DetalleISCBaseImponibleProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetalleISCBaseImponibleProcessRule for T
where
    T: DetalleISCBaseImponibleGetter
        + DetalleISCBaseImponibleSetter
        + DetalleIGVTipoGetter
        + DetalleCantidadGetter
        + DetallePrecioGetter
        + DetallePrecioReferenciaGetter,
{
    fn process(&mut self) -> bool {
        match (
            &self.get_iscbaseimponible(),
            &self.get_igvtipo(),
            &self.get_precio(),
            &self.get_precioreferencia(),
        ) {
            (None, Some(igv_tipo), Some(precio), Some(precio_referencia)) => {
                if let Some(catalog) = catalog7_value_of_code(igv_tipo) {
                    let base_imponible = if catalog.onerosa() {
                        self.get_cantidad() * *precio
                    } else {
                        self.get_cantidad() * precio_referencia
                    };

                    self.set_iscbaseimponible(base_imponible);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
