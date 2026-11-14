use anyhow::Result;

use crate::xbuilder::catalogs::{Catalog7, FromCode};
use crate::xbuilder::enricher::bounds::detalle::cantidad::DetalleCantidadGetter;
use crate::xbuilder::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::xbuilder::enricher::bounds::detalle::isc_base_imponible::{
    DetalleIscBaseImponibleGetter, DetalleIscBaseImponibleSetter,
};
use crate::xbuilder::enricher::bounds::detalle::precio::DetallePrecioGetter;
use crate::xbuilder::enricher::bounds::detalle::precio_referencia::DetallePrecioReferenciaGetter;

pub trait DetalleIscBaseImponibleProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetalleIscBaseImponibleProcessRule for T
where
    T: DetalleIscBaseImponibleGetter
        + DetalleIscBaseImponibleSetter
        + DetalleIgvTipoGetter
        + DetalleCantidadGetter
        + DetallePrecioGetter
        + DetallePrecioReferenciaGetter,
{
    fn process(&mut self) -> Result<bool> {
        match (
            &self.isc_base_imponible(),
            &self.igv_tipo(),
            &self.precio(),
            &self.precio_referencia(),
        ) {
            (None, Some(igv_tipo), Some(precio), Some(precio_referencia)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    let base_imponible = if catalog.onerosa() {
                        self.cantidad() * *precio
                    } else {
                        self.cantidad() * precio_referencia
                    };

                    self.set_isc_base_imponible(base_imponible);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}
