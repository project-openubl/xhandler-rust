use crate::enricher::bounds::icb::{IcbTasaGetter, IcbTasaSetter};
use crate::enricher::Defaults;

pub trait IcbTasaFillRule {
    fn fill(&mut self, defaults: &Defaults) -> bool;
}

impl<T> IcbTasaFillRule for T
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
