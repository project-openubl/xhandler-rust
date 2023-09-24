use crate::catalogs::{Catalog, Catalog7};
use crate::enricher::rules::phase1enrich::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::igvtipo::{DetalleIGVTipoGetter, DetalleIGVTipoSetter};

pub trait DetalleIGVTipoEnrichRule {
    fn enrich(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleIGVTipoEnrichRule for T
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
