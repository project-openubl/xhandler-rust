use anyhow::Result;

use crate::catalogs::{code_for_name, Catalog, Catalog7};
use crate::enricher::bounds::detalle::igv_tipo::{DetalleIgvTipoGetter, DetalleIgvTipoSetter};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;

pub trait DetalleIgvTipoFillRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> Result<bool>;
}

impl<T> DetalleIgvTipoFillRule for T
where
    T: DetalleIgvTipoGetter + DetalleIgvTipoSetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> Result<bool> {
        match &self.get_igv_tipo() {
            Some(value) => match code_for_name::<Catalog7>(value) {
                Some(code) => {
                    self.set_igv_tipo(&code);
                    Ok(true)
                }
                None => Ok(false),
            },
            None => {
                self.set_igv_tipo(Catalog7::GravadoOperacionOnerosa.code());
                Ok(true)
            }
        }
    }
}
