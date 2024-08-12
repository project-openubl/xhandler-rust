use anyhow::Result;
use log::warn;

use crate::catalogs::{Catalog7, FromCode};
use crate::enricher::bounds::detalle::cantidad::DetalleCantidadGetter;
use crate::enricher::bounds::detalle::igv_base_imponible::{
    DetalleIGVBaseImponibleSetter, DetalleIgvBaseImponibleGetter,
};
use crate::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::enricher::bounds::detalle::isc::DetalleIscGetter;
use crate::enricher::bounds::detalle::precio::DetallePrecioGetter;
use crate::enricher::bounds::detalle::precio_referencia::DetallePrecioReferenciaGetter;

pub trait DetalleIGVBaseImponibleProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetalleIGVBaseImponibleProcessRule for T
where
    T: DetalleIgvBaseImponibleGetter
        + DetalleIGVBaseImponibleSetter
        + DetalleIgvTipoGetter
        + DetalleCantidadGetter
        + DetallePrecioGetter
        + DetallePrecioReferenciaGetter
        + DetalleIscGetter,
{
    fn process(&mut self) -> Result<bool> {
        match (
            &self.get_igv_base_imponible(),
            &self.get_igv_tipo(),
            &self.get_precio(),
            &self.get_precio_referencia(),
            &self.get_isc(),
        ) {
            (None, Some(igv_tipo), Some(precio), Some(precio_referencia), Some(isc)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    let base_imponible = if catalog.onerosa() {
                        self.get_cantidad() * precio
                    } else {
                        self.get_cantidad() * precio_referencia
                    };

                    self.set_igv_base_imponible(base_imponible + *isc);
                    Ok(true)
                } else {
                    warn!("DetalleIGVBaseImponibleProcessRule: {igv_tipo} codigo no valido para Catalog7");
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}
