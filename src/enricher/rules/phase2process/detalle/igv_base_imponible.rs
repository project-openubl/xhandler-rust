use log::warn;

use crate::catalogs::catalog7_value_of_code;
use crate::models::traits::detalle::cantidad::DetalleCantidadGetter;
use crate::models::traits::detalle::igvbaseimponible::{
    DetalleIGVBaseImponibleGetter, DetalleIGVBaseImponibleSetter,
};
use crate::models::traits::detalle::igvtipo::DetalleIGVTipoGetter;
use crate::models::traits::detalle::isc::DetalleISCGetter;
use crate::models::traits::detalle::precio::DetallePrecioGetter;
use crate::models::traits::detalle::precioreferencia::DetallePrecioReferenciaGetter;

pub trait DetalleIGVBaseImponibleProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetalleIGVBaseImponibleProcessRule for T
where
    T: DetalleIGVBaseImponibleGetter
        + DetalleIGVBaseImponibleSetter
        + DetalleIGVTipoGetter
        + DetalleCantidadGetter
        + DetallePrecioGetter
        + DetallePrecioReferenciaGetter
        + DetalleISCGetter,
{
    fn process(&mut self) -> bool {
        match (
            &self.get_igvbaseimponible(),
            &self.get_igvtipo(),
            &self.get_precio(),
            &self.get_precioreferencia(),
            &self.get_isc(),
        ) {
            (None, Some(igv_tipo), Some(precio), Some(precio_referencia), Some(isc)) => {
                if let Some(catalog) = catalog7_value_of_code(igv_tipo) {
                    let base_imponible = if catalog.onerosa() {
                        self.get_cantidad() * precio
                    } else {
                        self.get_cantidad() * precio_referencia
                    };

                    self.set_igvbaseimponible(base_imponible + *isc);
                    true
                } else {
                    warn!("DetalleIGVBaseImponibleProcessRule: {igv_tipo} codigo no valido para Catalog7");
                    false
                }
            }
            _ => false,
        }
    }
}
