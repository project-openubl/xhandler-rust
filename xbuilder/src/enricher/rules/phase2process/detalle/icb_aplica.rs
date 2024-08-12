use anyhow::Result;
use rust_decimal::Decimal;

use crate::enricher::bounds::detalle::icb::DetalleIcbGetter;
use crate::enricher::bounds::detalle::icb_aplica::{
    DetalleICBAplicaGetter, DetalleIcbAplicaSetter,
};

pub trait DetalleICBAplicaProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetalleICBAplicaProcessRule for T
where
    T: DetalleICBAplicaGetter + DetalleIcbAplicaSetter + DetalleIcbGetter,
{
    fn process(&mut self) -> Result<bool> {
        match (&self.get_icb_aplica(), &self.get_icb()) {
            (false, Some(icb)) => {
                if icb > &Decimal::ZERO {
                    self.set_icb_aplica(true);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}
