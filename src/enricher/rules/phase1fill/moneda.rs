use crate::enricher::bounds::moneda::{MonedaGetter, MonedaSetter};

pub trait MonedaEnrichRule {
    fn fill(&mut self) -> bool;
}

impl<T> MonedaEnrichRule for T
where
    T: MonedaGetter + MonedaSetter,
{
    fn fill(&mut self) -> bool {
        match &self.get_moneda() {
            Some(..) => false,
            None => {
                self.set_moneda("PEN");
                true
            }
        }
    }
}
