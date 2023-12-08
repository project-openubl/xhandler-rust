use crate::enricher::bounds::moneda::{MonedaGetter, MonedaSetter};

pub trait MonedaFillRule {
    fn fill(&mut self) -> bool;
}

impl<T> MonedaFillRule for T
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
