use crate::enricher::enrich::Defaults;
use crate::models::traits::igv::{IGVTasaGetter, IGVTasaSetter};

pub trait IGVTasaEnrichRule {
    fn enrich(&mut self, defaults: &Defaults) -> bool;
}

impl<T> IGVTasaEnrichRule for T
where
    T: IGVTasaGetter + IGVTasaSetter,
{
    fn enrich(&mut self, defaults: &Defaults) -> bool {
        match &self.get_igv_tasa() {
            Some(..) => false,
            None => {
                self.set_igv_tasa(defaults.igv_tasa);
                true
            }
        }
    }
}
