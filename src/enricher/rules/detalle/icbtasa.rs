use crate::enricher::rules::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::icbtasa::{DetalleICBTasaGetter, DetalleICBTasaSetter};

pub trait DetalleICBTasaRule {
    fn enrich(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleICBTasaRule for T
where
    T: DetalleICBTasaGetter + DetalleICBTasaSetter,
{
    fn enrich(&mut self, defaults: &DetalleDefaults) -> bool {
        match &self.get_icbtasa() {
            Some(..) => false,
            None => {
                self.set_icbtasa(defaults.icb_tasa);
                true
            }
        }
    }
}
