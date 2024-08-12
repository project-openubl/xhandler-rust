use anyhow::Result;

use crate::catalogs::{Catalog, Catalog16, Catalog7, FromCode};
use crate::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::enricher::bounds::detalle::precio_referencia_tipo::{
    DetallePrecioReferenciaTipoGetter, DetallePrecioReferenciaTipoSetter,
};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;

pub trait DetallePrecioReferenciaTipoFillRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> Result<bool>;
}

impl<T> DetallePrecioReferenciaTipoFillRule for T
where
    T: DetallePrecioReferenciaTipoGetter + DetallePrecioReferenciaTipoSetter + DetalleIgvTipoGetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> Result<bool> {
        match (self.get_precio_referencia_tipo(), *self.get_igv_tipo()) {
            (None, Some(igv_tipo)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    let catalog16 = if catalog.onerosa() {
                        &Catalog16::PrecioUnitarioIncluyeIgv
                    } else {
                        &Catalog16::ValorReferencialUnitarioEnOperacionesNoOnerosas
                    };
                    self.set_precio_referencia_tipo(catalog16.code());
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}
