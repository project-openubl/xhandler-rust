use crate::enricher::bounds::ivap::{IvapTasaGetter, IvapTasaSetter};
use crate::enricher::Defaults;

pub trait IvapTasaFillRule {
    fn fill(&mut self, defaults: &Defaults) -> bool;
}

impl<T> IvapTasaFillRule for T
where
    T: IvapTasaGetter + IvapTasaSetter,
{
    fn fill(&mut self, defaults: &Defaults) -> bool {
        match &self.get_ivap_tasa() {
            Some(..) => false,
            None => {
                self.set_ivap_tasa(defaults.ivap_tasa);
                true
            }
        }
    }
}
