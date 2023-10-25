use crate::catalogs::catalog7_value_of_code;
use crate::models::traits::detalle::igvtasa::DetalleIGVTasaGetter;
use crate::models::traits::detalle::igvtipo::DetalleIGVTipoGetter;
use crate::models::traits::detalle::isctasa::DetalleISCTasaGetter;
use crate::models::traits::detalle::precio::{DetallePrecioGetter, DetallePrecioSetter};
use crate::models::traits::detalle::precioconimpuestos::DetallePrecioConImpuestosGetter;

pub trait DetallePrecioProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetallePrecioProcessRule for T
where
    T: DetallePrecioGetter
        + DetallePrecioSetter
        + DetalleIGVTipoGetter
        + DetallePrecioConImpuestosGetter
        + DetalleIGVTasaGetter
        + DetalleISCTasaGetter,
{
    fn process(&mut self) -> bool {
        match (
            &self.get_precio(),
            &self.get_igvtipo(),
            &self.get_igvtasa(),
            &self.get_isctasa(),
            &self.get_precioconimpuestos(),
        ) {
            (None, Some(igv_tipo), Some(igv_tasa), Some(isc_tasa), Some(precio_con_impuestos)) => {
                if let Some(catalog) = catalog7_value_of_code(igv_tipo) {
                    let precio = if catalog.onerosa() {
                        precio_con_impuestos / (1f64 + *igv_tasa) / (1f64 + *isc_tasa)
                    } else {
                        0f64
                    };

                    self.set_precio(precio);
                    true
                } else {
                    false
                }
            }
            // Si operacion onerosa y precio es diferente de cero => modificarlo
            (Some(precio), Some(igv_tipo), Some(_), Some(_), Some(_)) => {
                if let Some(catalog) = catalog7_value_of_code(igv_tipo) {
                    if !catalog.onerosa() && precio > &0f64 {
                        self.set_precio(0f64);
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
