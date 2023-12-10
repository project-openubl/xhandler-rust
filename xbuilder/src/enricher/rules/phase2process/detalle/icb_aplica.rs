use rust_decimal::Decimal;

use crate::enricher::bounds::detalle::icb::DetalleIcbGetter;
use crate::enricher::bounds::detalle::icb_aplica::{
    DetalleICBAplicaGetter, DetalleIcbAplicaSetter,
};

pub trait DetalleICBAplicaProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetalleICBAplicaProcessRule for T
where
    T: DetalleICBAplicaGetter + DetalleIcbAplicaSetter + DetalleIcbGetter,
{
    fn process(&mut self) -> bool {
        match (&self.get_icb_aplica(), &self.get_icb()) {
            (false, Some(icb)) => {
                if icb > &Decimal::ZERO {
                    self.set_icb_aplica(true);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
