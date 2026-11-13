use anyhow::Result;

use crate::catalogs::{Catalog7, FromCode};
use crate::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::enricher::bounds::detalle::precio::DetallePrecioGetter;
use crate::enricher::bounds::detalle::precio_con_impuestos::DetallePrecioConImpuestosGetter;
use crate::enricher::bounds::detalle::precio_referencia::{
    DetallePrecioReferenciaGetter, DetallePrecioReferenciaSetter,
};

pub trait DetallePrecioReferenciaProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetallePrecioReferenciaProcessRule for T
where
    T: DetallePrecioReferenciaGetter
        + DetallePrecioReferenciaSetter
        + DetallePrecioGetter
        + DetallePrecioConImpuestosGetter
        + DetalleIgvTipoGetter,
{
    fn process(&mut self) -> Result<bool> {
        match (
            &self.get_precio_referencia(),
            &self.get_igv_tipo(),
            &self.get_precio(),
            &self.get_precio_con_impuestos(),
        ) {
            (None, Some(igv_tipo), Some(precio), Some(precio_con_impuestos)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    let precio_referencia = if catalog.onerosa() {
                        precio_con_impuestos
                    } else {
                        precio
                    };
                    self.set_precio_referencia(*precio_referencia);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}
