use anyhow::Result;

use crate::xbuilder::catalogs::{code_for_name, Catalog, Catalog8};
use crate::xbuilder::enricher::bounds::detalle::isc_tipo::{
    DetalleIscTipoGetter, DetalleIscTipoSetter,
};
use crate::xbuilder::enricher::rules::phase1_fill::detalle::detalles::DetalleDefaults;

pub trait DetalleIscTipoFillRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> Result<bool>;
}

impl<T> DetalleIscTipoFillRule for T
where
    T: DetalleIscTipoGetter + DetalleIscTipoSetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> Result<bool> {
        match &self.isc_tipo() {
            Some(value) => match code_for_name::<Catalog8>(value) {
                Some(code) => {
                    self.set_isc_tipo(&code);
                    Ok(true)
                }
                None => Ok(false),
            },
            None => {
                self.set_isc_tipo(Catalog8::SistemaAlValor.code());
                Ok(true)
            }
        }
    }
}
