use anyhow::Result;
use rust_decimal::Decimal;

use crate::xbuilder::enricher::bounds::detalle::icb::DetalleIcbGetter;
use crate::xbuilder::enricher::bounds::detalle::icb_aplica::{
    DetalleIcbAplicaGetter, DetalleIcbAplicaSetter,
};

pub trait DetalleIcbAplicaProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetalleIcbAplicaProcessRule for T
where
    T: DetalleIcbAplicaGetter + DetalleIcbAplicaSetter + DetalleIcbGetter,
{
    fn process(&mut self) -> Result<bool> {
        match (&self.icb_aplica(), &self.icb()) {
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
