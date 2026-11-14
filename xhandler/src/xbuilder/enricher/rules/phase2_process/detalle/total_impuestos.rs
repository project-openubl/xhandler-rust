use anyhow::Result;
use rust_decimal::Decimal;

use crate::xbuilder::catalogs::{Catalog7, FromCode};
use crate::xbuilder::enricher::bounds::detalle::icb::DetalleIcbGetter;
use crate::xbuilder::enricher::bounds::detalle::igv::DetalleIgvGetter;
use crate::xbuilder::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::xbuilder::enricher::bounds::detalle::isc::DetalleIscGetter;
use crate::xbuilder::enricher::bounds::detalle::total_impuestos::{
    DetalleTotalImpuestosGetter, DetalleTotalImpuestosSetter,
};

pub trait DetalleTotalImpuestosProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetalleTotalImpuestosProcessRule for T
where
    T: DetalleTotalImpuestosGetter
        + DetalleTotalImpuestosSetter
        + DetalleIgvTipoGetter
        + DetalleIgvGetter
        + DetalleIcbGetter
        + DetalleIscGetter,
{
    fn process(&mut self) -> Result<bool> {
        match (
            &self.total_impuestos(),
            &self.igv_tipo(),
            &self.igv(),
            &self.icb(),
            &self.isc(),
        ) {
            (None, Some(igv_tipo), Some(igv), Some(icb), Some(isc)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    let igv_isc = if catalog.onerosa() {
                        (*igv, *isc)
                    } else {
                        (Decimal::ZERO, Decimal::ZERO)
                    };

                    let total = icb + igv_isc.0 + igv_isc.1;
                    self.set_total_impuestos(total);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}
