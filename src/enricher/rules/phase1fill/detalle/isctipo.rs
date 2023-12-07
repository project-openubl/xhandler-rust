use crate::catalogs::{Catalog, Catalog8};
use crate::enricher::bounds::detalle::isc_tipo::{DetalleIscTipoGetter, DetalleIscTipoSetter};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;

pub trait DetalleISCTipoEnrichRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleISCTipoEnrichRule for T
where
    T: DetalleIscTipoGetter + DetalleIscTipoSetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> bool {
        match &self.get_isc_tipo() {
            Some(..) => false,
            None => {
                self.set_isc_tipo(Catalog8::SistemaAlValor.code());
                true
            }
        }
    }
}
