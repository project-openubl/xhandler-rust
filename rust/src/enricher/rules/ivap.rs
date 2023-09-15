use crate::enricher::enrich::Defaults;
use crate::models::traits::ivap::{IVAPTasaGetter, IVAPTasaSetter};

pub trait IVAPRule {
    fn enrich_ivap(&mut self, defaults: &Defaults) -> bool;
}

impl<T> IVAPRule for T
where
    T: IVAPTasaGetter + IVAPTasaSetter,
{
    fn enrich_ivap(&mut self, defaults: &Defaults) -> bool {
        match &self.get_ivap_tasa() {
            Some(..) => false,
            None => {
                self.set_ivap_tasa(defaults.ivap_tasa);
                true
            }
        }
    }
}
