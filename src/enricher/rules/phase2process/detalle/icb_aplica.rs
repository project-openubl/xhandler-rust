use rust_decimal::Decimal;

use crate::models::traits::detalle::icb::DetalleICBGetter;
use crate::models::traits::detalle::icbaplica::{DetalleICBAplicaGetter, DetalleICBAplicaSetter};

pub trait DetalleICBAplicaProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetalleICBAplicaProcessRule for T
where
    T: DetalleICBAplicaGetter + DetalleICBAplicaSetter + DetalleICBGetter,
{
    fn process(&mut self) -> bool {
        match (&self.get_icbaplica(), &self.get_icb()) {
            (false, Some(icb)) => {
                if icb > &Decimal::ZERO {
                    self.set_icbaplica(true);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
