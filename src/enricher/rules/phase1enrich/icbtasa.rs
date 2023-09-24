use crate::enricher::enrich::Defaults;
use crate::models::traits::icb::{ICBTasaGetter, ICBTasaSetter};

pub trait ICBTasaEnrichRule {
    fn enrich(&mut self, defaults: &Defaults) -> bool;
}

impl<T> ICBTasaEnrichRule for T
where
    T: ICBTasaGetter + ICBTasaSetter,
{
    fn enrich(&mut self, defaults: &Defaults) -> bool {
        match &self.get_icb_tasa() {
            Some(..) => false,
            None => {
                self.set_icb_tasa(defaults.icb_tasa);
                true
            }
        }
    }
}
