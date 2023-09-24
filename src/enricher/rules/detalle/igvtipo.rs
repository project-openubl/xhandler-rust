use crate::catalogs::{Catalog, Catalog7};
use crate::enricher::rules::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::igvtipo::{DetalleIGVTipoGetter, DetalleIGVTipoSetter};

pub trait DetalleIGVTipoRule {
    fn enrich(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleIGVTipoRule for T
where
    T: DetalleIGVTipoGetter + DetalleIGVTipoSetter,
{
    fn enrich(&mut self, _: &DetalleDefaults) -> bool {
        match &self.get_igvtipo() {
            Some(..) => false,
            None => {
                self.set_igvtipo(Catalog7::GravadoOperacionOnerosa.code());
                true
            }
        }
    }
}
