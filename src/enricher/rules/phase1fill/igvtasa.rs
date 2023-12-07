use crate::enricher::bounds::igv::{IgvTasaGetter, IgvTasaSetter};
use crate::enricher::enrich::Defaults;

pub trait IGVTasaEnrichRule {
    fn fill(&mut self, defaults: &Defaults) -> bool;
}

impl<T> IGVTasaEnrichRule for T
where
    T: IgvTasaGetter + IgvTasaSetter,
{
    fn fill(&mut self, defaults: &Defaults) -> bool {
        match &self.get_igv_tasa() {
            Some(..) => false,
            None => {
                self.set_igv_tasa(defaults.igv_tasa);
                true
            }
        }
    }
}
