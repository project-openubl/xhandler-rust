use rust_decimal_macros::dec;

use crate::catalogs::{Catalog7, FromCode};
use crate::enricher::bounds::detalle::igv_tasa::DetalleIgvTasaGetter;
use crate::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::enricher::bounds::detalle::isc_tasa::DetalleIscTasaGetter;
use crate::enricher::bounds::detalle::precio::DetallePrecioGetter;
use crate::enricher::bounds::detalle::precio_con_impuestos::{
    DetallePrecioConImpuestosGetter, DetallePrecioConImpuestosSetter,
};

pub trait DetallePrecioConImpuestosProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetallePrecioConImpuestosProcessRule for T
where
    T: DetallePrecioConImpuestosGetter
        + DetallePrecioConImpuestosSetter
        + DetalleIgvTipoGetter
        + DetallePrecioGetter
        + DetalleIgvTasaGetter
        + DetalleIscTasaGetter,
{
    fn process(&mut self) -> bool {
        match (
            &self.get_precio_con_impuestos(),
            &self.get_igv_tipo(),
            &self.get_igv_tasa(),
            &self.get_isc_tasa(),
            &self.get_precio(),
        ) {
            (None, Some(igv_tipo), Some(igv_tasa), Some(isc_tasa), Some(precio)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    let precio_con_impuestos = if catalog.onerosa() {
                        precio * (dec!(1) + *igv_tasa) * (dec!(1) + *isc_tasa)
                    } else {
                        dec!(0)
                    };

                    self.set_precio_con_impuestos(precio_con_impuestos);
                    true
                } else {
                    false
                }
            }
            // Si operacion no es onerosa y precio es diferente de cero => modificarlo
            (Some(precio_con_impuestos), Some(igv_tipo), Some(_), Some(_), Some(_)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    if !catalog.onerosa() && precio_con_impuestos > &dec!(0) {
                        self.set_precio_con_impuestos(dec!(0));
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
