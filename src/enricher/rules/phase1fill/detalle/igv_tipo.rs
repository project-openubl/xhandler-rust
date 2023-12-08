use crate::catalogs::{Catalog, Catalog7};
use crate::enricher::bounds::detalle::igv_tipo::{DetalleIgvTipoGetter, DetalleIgvTipoSetter};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;

pub trait DetalleIGVTipoFillRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleIGVTipoFillRule for T
where
    T: DetalleIgvTipoGetter + DetalleIgvTipoSetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> bool {
        match &self.get_igv_tipo() {
            Some(..) => false,
            None => {
                self.set_igv_tipo(Catalog7::GravadoOperacionOnerosa.code());
                true
            }
        }
    }
}
