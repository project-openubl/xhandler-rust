use anyhow::Result;

use crate::catalogs::{Catalog7, FromCode};
use crate::enricher::bounds::detalle::cantidad::DetalleCantidadGetter;
use crate::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::enricher::bounds::detalle::isc_base_imponible::{
    DetalleISCBaseImponibleSetter, DetalleIscBaseImponibleGetter,
};
use crate::enricher::bounds::detalle::precio::DetallePrecioGetter;
use crate::enricher::bounds::detalle::precio_referencia::DetallePrecioReferenciaGetter;

pub trait DetalleISCBaseImponibleProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetalleISCBaseImponibleProcessRule for T
where
    T: DetalleIscBaseImponibleGetter
        + DetalleISCBaseImponibleSetter
        + DetalleIgvTipoGetter
        + DetalleCantidadGetter
        + DetallePrecioGetter
        + DetallePrecioReferenciaGetter,
{
    fn process(&mut self) -> Result<bool> {
        match (
            &self.get_isc_base_imponible(),
            &self.get_igv_tipo(),
            &self.get_precio(),
            &self.get_precio_referencia(),
        ) {
            (None, Some(igv_tipo), Some(precio), Some(precio_referencia)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    let base_imponible = if catalog.onerosa() {
                        self.get_cantidad() * *precio
                    } else {
                        self.get_cantidad() * precio_referencia
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
