use rust_decimal_macros::dec;

use crate::catalogs::{Catalog7, FromCode};
use crate::models::traits::detalle::icb::DetalleICBGetter;
use crate::models::traits::detalle::igv::DetalleIGVGetter;
use crate::models::traits::detalle::igvtipo::DetalleIGVTipoGetter;
use crate::models::traits::detalle::isc::DetalleISCGetter;
use crate::models::traits::detalle::totalimpuestos::{
    DetalleTotalImpuestosGetter, DetalleTotalImpuestosSetter,
};

pub trait DetalleTotalImpuestosProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetalleTotalImpuestosProcessRule for T
where
    T: DetalleTotalImpuestosGetter
        + DetalleTotalImpuestosSetter
        + DetalleIGVTipoGetter
        + DetalleIGVGetter
        + DetalleICBGetter
        + DetalleISCGetter,
{
    fn process(&mut self) -> bool {
        match (
            &self.get_totalimpuestos(),
            &self.get_igvtipo(),
            &self.get_igv(),
            &self.get_icb(),
            &self.get_isc(),
        ) {
            (None, Some(igv_tipo), Some(igv), Some(icb), Some(isc)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    let igv_isc = if catalog.onerosa() {
                        (*igv, *isc)
                    } else {
                        (dec!(0), dec!(0))
                    };

                    let total = icb + igv_isc.0 + igv_isc.1;
                    self.set_totalimpuestos(total);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
