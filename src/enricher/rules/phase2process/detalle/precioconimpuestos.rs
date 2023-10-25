use crate::catalogs::catalog7_value_of_code;
use crate::models::traits::detalle::igvtasa::DetalleIGVTasaGetter;
use crate::models::traits::detalle::igvtipo::DetalleIGVTipoGetter;
use crate::models::traits::detalle::isctasa::DetalleISCTasaGetter;
use crate::models::traits::detalle::precio::DetallePrecioGetter;
use crate::models::traits::detalle::precioconimpuestos::{
    DetallePrecioConImpuestosGetter, DetallePrecioConImpuestosSetter,
};

pub trait DetallePrecioConImpuestosProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetallePrecioConImpuestosProcessRule for T
where
    T: DetallePrecioConImpuestosGetter
        + DetallePrecioConImpuestosSetter
        + DetalleIGVTipoGetter
        + DetallePrecioGetter
        + DetalleIGVTasaGetter
        + DetalleISCTasaGetter,
{
    fn process(&mut self) -> bool {
        match (
            &self.get_precioconimpuestos(),
            &self.get_igvtipo(),
            &self.get_igvtasa(),
            &self.get_isctasa(),
            &self.get_precio(),
        ) {
            (None, Some(igv_tipo), Some(igv_tasa), Some(isc_tasa), Some(precio)) => {
                if let Some(catalog) = catalog7_value_of_code(igv_tipo) {
                    let precio_con_impuestos = if catalog.onerosa() {
                        precio * (1f64 + *igv_tasa) * (1f64 + *isc_tasa)
                    } else {
                        0f64
                    };

                    self.set_precioconimpuestos(precio_con_impuestos);
                    true
                } else {
                    false
                }
            }
            // Si operacion onerosa y precio es diferente de cero => modificarlo
            (Some(precio_con_impuestos), Some(igv_tipo), Some(_), Some(_), Some(_)) => {
                if let Some(catalog) = catalog7_value_of_code(igv_tipo) {
                    if !catalog.onerosa() && precio_con_impuestos > &0f64 {
                        self.set_precioconimpuestos(0f64);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
