use crate::enricher::bounds::fecha_emision::{FechaEmisionGetter, FechaEmisionSetter};
use crate::enricher::Defaults;

pub trait FechaEmisionEnrichRule {
    fn fill(&mut self, defaults: &Defaults) -> bool;
}

impl<T> FechaEmisionEnrichRule for T
where
    T: FechaEmisionGetter + FechaEmisionSetter,
{
    fn fill(&mut self, defaults: &Defaults) -> bool {
        match &self.get_fecha_emision() {
            Some(..) => false,
            None => {
                self.set_fecha_emision(defaults.date);
                true
            }
        }
    }
}
