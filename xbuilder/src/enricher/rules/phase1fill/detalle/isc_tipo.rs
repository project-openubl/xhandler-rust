use anyhow::Result;

use crate::catalogs::{Catalog, Catalog8};
use crate::enricher::bounds::detalle::isc_tipo::{DetalleIscTipoGetter, DetalleIscTipoSetter};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;

pub trait DetalleISCTipoFillRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> Result<bool>;
}

impl<T> DetalleISCTipoFillRule for T
where
    T: DetalleIscTipoGetter + DetalleIscTipoSetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> Result<bool> {
        match &self.get_isc_tipo() {
            Some(..) => Ok(false),
            None => {
                self.set_isc_tipo(Catalog8::SistemaAlValor.code());
                Ok(true)
            }
        }
    }
}
