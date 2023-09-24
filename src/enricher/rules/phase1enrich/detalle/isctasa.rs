use crate::enricher::rules::phase1enrich::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::isctasa::{DetalleISCTasaGetter, DetalleISCTasaSetter};

pub trait DetalleISCTasaEnrichRule {
    fn enrich(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleISCTasaEnrichRule for T
where
    T: DetalleISCTasaGetter + DetalleISCTasaSetter,
{
    fn enrich(&mut self, _: &DetalleDefaults) -> bool {
        match &self.get_isctasa() {
            Some(..) => false,
            None => {
                self.set_isctasa(0f32);
                true
            }
        }
    }
}
