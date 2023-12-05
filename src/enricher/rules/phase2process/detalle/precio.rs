use rust_decimal::Decimal;

use crate::catalogs::{Catalog7, FromCode};
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
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    let precio = if catalog.onerosa() {
                        precio_con_impuestos
                            / (Decimal::ONE + *igv_tasa)
                            / (Decimal::ONE + *isc_tasa)
                    } else {
                        *precio_con_impuestos
                    };

                    self.set_precio(precio);
                    true
                } else {
                    false
                }
            }
            // Si operacion onerosa y precio es diferente de cero => modificarlo
            (Some(precio), Some(igv_tipo), Some(_), Some(_), Some(_)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    if !catalog.onerosa() && precio > &Decimal::ZERO {
                        self.set_precio(Decimal::ZERO);
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
