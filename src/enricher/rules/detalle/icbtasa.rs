use crate::models::traits::detalle::icbtasa::{ICBTasaGetter, ICBTasaSetter};

pub trait DetalleICBTasaRule {
    fn enrich(&mut self) -> bool;
}

impl<T> DetalleICBTasaRule for T
where
    T: ICBTasaGetter + ICBTasaSetter,
{
    fn enrich(&mut self) -> bool {
        match &self.get_icbtasa() {
            Some(..) => false,
            None => {
                // self.set_icbtasa("NIU");
                true
            }
        }
    }
}
