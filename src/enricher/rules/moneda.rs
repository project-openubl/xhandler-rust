use crate::models::traits::moneda::{MonedaGetter, MonedaSetter};

pub trait MonedaRule {
    fn enrich(&mut self) -> bool;
}

impl<T> MonedaRule for T
where
    T: MonedaGetter + MonedaSetter,
{
    fn enrich(&mut self) -> bool {
        match &self.get_moneda() {
            Some(..) => false,
            None => {
                self.set_moneda("PEN");
                true
            }
        }
    }
}
