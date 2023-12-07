use rust_decimal::Decimal;

use crate::catalogs::{Catalog7, FromCode};
use crate::enricher::bounds::detalle::igv_tasa::DetalleIgvTasaGetter;
use crate::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::enricher::bounds::detalle::isc_tasa::DetalleIscTasaGetter;
use crate::enricher::bounds::detalle::precio::{DetallePrecioGetter, DetallePrecioSetter};
use crate::enricher::bounds::detalle::precio_con_impuestos::DetallePrecioConImpuestosGetter;

pub trait DetallePrecioProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetallePrecioProcessRule for T
where
    T: DetallePrecioGetter
        + DetallePrecioSetter
        + DetalleIgvTipoGetter
        + DetallePrecioConImpuestosGetter
        + DetalleIgvTasaGetter
        + DetalleIscTasaGetter,
{
    fn process(&mut self) -> bool {
        match (
            &self.get_precio(),
            &self.get_igv_tipo(),
            &self.get_igv_tasa(),
            &self.get_isc_tasa(),
            &self.get_precio_con_impuestos(),
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
