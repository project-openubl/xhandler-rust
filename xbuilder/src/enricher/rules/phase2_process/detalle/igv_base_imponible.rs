use anyhow::Result;
use log::warn;

use crate::catalogs::{Catalog7, FromCode};
use crate::enricher::bounds::detalle::cantidad::DetalleCantidadGetter;
use crate::enricher::bounds::detalle::igv_base_imponible::{
    DetalleIgvBaseImponibleGetter, DetalleIgvBaseImponibleSetter,
};
use crate::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::enricher::bounds::detalle::isc::DetalleIscGetter;
use crate::enricher::bounds::detalle::precio::DetallePrecioGetter;
use crate::enricher::bounds::detalle::precio_referencia::DetallePrecioReferenciaGetter;

pub trait DetalleIgvBaseImponibleProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetalleIgvBaseImponibleProcessRule for T
where
    T: DetalleIgvBaseImponibleGetter
        + DetalleIgvBaseImponibleSetter
        + DetalleIgvTipoGetter
        + DetalleCantidadGetter
        + DetallePrecioGetter
        + DetallePrecioReferenciaGetter
        + DetalleIscGetter,
{
    fn process(&mut self) -> Result<bool> {
        match (
            &self.igv_base_imponible(),
            &self.igv_tipo(),
            &self.precio(),
            &self.precio_referencia(),
            &self.isc(),
        ) {
            (None, Some(igv_tipo), Some(precio), Some(precio_referencia), Some(isc)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    let base_imponible = if catalog.onerosa() {
                        self.cantidad() * precio
                    } else {
                        self.cantidad() * precio_referencia
                    };

                    self.set_igv_base_imponible(base_imponible + *isc);
                    Ok(true)
                } else {
                    warn!("DetalleIgvBaseImponibleProcessRule: {igv_tipo} codigo no valido para Catalog7");
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}
