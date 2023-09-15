use crate::enricher::enrich::Defaults;
use crate::models::traits::igv::{IGVTasaGetter, IGVTasaSetter};

pub trait IGVRule {
    fn enrich_igv(&mut self, defaults: &Defaults) -> bool;
}

impl<T> IGVRule for T
where
    T: IGVTasaGetter + IGVTasaSetter,
{
    fn enrich_igv(&mut self, defaults: &Defaults) -> bool {
        match &self.get_igv_tasa() {
            Some(..) => false,
            None => {
                self.set_igv_tasa(defaults.igv_tasa);
                true
            }
        }
    }
}
