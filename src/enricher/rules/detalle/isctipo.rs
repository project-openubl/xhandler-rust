use crate::catalogs::{Catalog, Catalog8};
use crate::enricher::rules::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::isctipo::{DetalleISCTipoGetter, DetalleISCTipoSetter};

pub trait DetalleISCTipoRule {
    fn enrich(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleISCTipoRule for T
where
    T: DetalleISCTipoGetter + DetalleISCTipoSetter,
{
    fn enrich(&mut self, _: &DetalleDefaults) -> bool {
        match &self.get_isctipo() {
            Some(..) => false,
            None => {
                self.set_isctipo(Catalog8::SistemaAlValor.code());
                true
            }
        }
    }
}
