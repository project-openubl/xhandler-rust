use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::isctasa::{DetalleISCTasaGetter, DetalleISCTasaSetter};

pub trait DetalleISCTasaEnrichRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleISCTasaEnrichRule for T
where
    T: DetalleISCTasaGetter + DetalleISCTasaSetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> bool {
        match &self.get_isctasa() {
            Some(..) => false,
            None => {
                self.set_isctasa(0f64);
                true
            }
        }
    }
}
