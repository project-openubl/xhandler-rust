use crate::enricher::rules::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::isctasa::{DetalleISCTasaGetter, DetalleISCTasaSetter};

pub trait DetalleISCTasaRule {
    fn enrich(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleISCTasaRule for T
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
