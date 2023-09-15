use crate::enricher::enrich::Defaults;
use crate::models::traits::icb::{ICBTasaGetter, ICBTasaSetter};

pub trait ICBRule {
    fn enrich_icb(&mut self, defaults: &Defaults) -> bool;
}

impl<T> ICBRule for T
where
    T: ICBTasaGetter + ICBTasaSetter,
{
    fn enrich_icb(&mut self, defaults: &Defaults) -> bool {
        match &self.get_icb_tasa() {
            Some(..) => false,
            None => {
                self.set_icb_tasa(defaults.icb_tasa);
                true
            }
        }
    }
}
