use crate::enricher::bounds::detalle::icb_tasa::{DetalleIcbTasaGetter, DetalleIcbTasaSetter};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;

pub trait DetalleICBTasaEnrichRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleICBTasaEnrichRule for T
where
    T: DetalleIcbTasaGetter + DetalleIcbTasaSetter,
{
    fn fill(&mut self, defaults: &DetalleDefaults) -> bool {
        match &self.get_icb_tasa() {
            Some(..) => false,
            None => {
                self.set_icb_tasa(defaults.icb_tasa);
                true
            }
        }
    }
}
