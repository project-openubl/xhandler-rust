use crate::enricher::bounds::icb::{IcbTasaGetter, IcbTasaSetter};
use crate::enricher::Defaults;

pub trait ICBTasaEnrichRule {
    fn fill(&mut self, defaults: &Defaults) -> bool;
}

impl<T> ICBTasaEnrichRule for T
where
    T: IcbTasaGetter + IcbTasaSetter,
{
    fn fill(&mut self, defaults: &Defaults) -> bool {
        match &self.get_icb_tasa() {
            Some(..) => false,
            None => {
                self.set_icb_tasa(defaults.icb_tasa);
                true
            }
        }
    }
}
