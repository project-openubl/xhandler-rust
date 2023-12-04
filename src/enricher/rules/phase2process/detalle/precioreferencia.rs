use crate::catalogs::catalog7_value_of_code;
use crate::models::traits::detalle::igvtipo::DetalleIGVTipoGetter;
use crate::models::traits::detalle::precio::DetallePrecioGetter;
use crate::models::traits::detalle::precioconimpuestos::DetallePrecioConImpuestosGetter;
use crate::models::traits::detalle::precioreferencia::{
    DetallePrecioReferenciaGetter, DetallePrecioReferenciaSetter,
};

pub trait DetallePrecioReferenciaProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetallePrecioReferenciaProcessRule for T
where
    T: DetallePrecioReferenciaGetter
        + DetallePrecioReferenciaSetter
        + DetallePrecioGetter
        + DetallePrecioConImpuestosGetter
        + DetalleIGVTipoGetter,
{
    fn process(&mut self) -> bool {
        match (
            &self.get_precioreferencia(),
            &self.get_igvtipo(),
            &self.get_precio(),
            &self.get_precioconimpuestos(),
        ) {
            (None, Some(igv_tipo), Some(precio), Some(precio_con_impuestos)) => {
                if let Some(catalog) = catalog7_value_of_code(igv_tipo) {
                    let precio_referencia = if catalog.onerosa() {
                        precio_con_impuestos
                    } else {
                        precio
                    };
                    self.set_precioreferencia(*precio_referencia);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
