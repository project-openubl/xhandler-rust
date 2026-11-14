use anyhow::Result;
use rust_decimal::Decimal;

use crate::xbuilder::enricher::bounds::detalle::cantidad::DetalleCantidadGetter;
use crate::xbuilder::enricher::bounds::detalle::icb::{DetalleIcbGetter, DetalleIcbSetter};
use crate::xbuilder::enricher::bounds::detalle::icb_aplica::DetalleIcbAplicaGetter;
use crate::xbuilder::enricher::bounds::detalle::icb_tasa::DetalleIcbTasaGetter;

pub trait DetalleIcbProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetalleIcbProcessRule for T
where
    T: DetalleIcbGetter
        + DetalleIcbSetter
        + DetalleIcbAplicaGetter
        + DetalleCantidadGetter
        + DetalleIcbTasaGetter,
{
    fn process(&mut self) -> Result<bool> {
        match &self.icb() {
            Some(..) => Ok(false),
            None => {
                if self.icb_aplica() {
                    if let Some(icb_tasa) = self.icb_tasa() {
                        let icb = self.cantidad() * *icb_tasa;
                        self.set_icb(icb);
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    self.set_icb(Decimal::ZERO);
                    Ok(true)
                }
            }
        }
    }
}
