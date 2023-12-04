use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::icbtasa::{DetalleICBTasaGetter, DetalleICBTasaSetter};

pub trait DetalleICBTasaEnrichRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleICBTasaEnrichRule for T
where
    T: DetalleICBTasaGetter + DetalleICBTasaSetter,
{
    fn fill(&mut self, defaults: &DetalleDefaults) -> bool {
        match &self.get_icbtasa() {
            Some(..) => false,
            None => {
                self.set_icbtasa(defaults.icb_tasa);
                true
            }
        }
    }
}
