use anyhow::Result;
use rust_decimal::Decimal;

use crate::enricher::bounds::detalle::cantidad::DetalleCantidadGetter;
use crate::enricher::bounds::detalle::icb::{DetalleIcbGetter, DetalleIcbSetter};
use crate::enricher::bounds::detalle::icb_aplica::DetalleICBAplicaGetter;
use crate::enricher::bounds::detalle::icb_tasa::DetalleIcbTasaGetter;

pub trait DetalleICBProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetalleICBProcessRule for T
where
    T: DetalleIcbGetter
        + DetalleIcbSetter
        + DetalleICBAplicaGetter
        + DetalleCantidadGetter
        + DetalleIcbTasaGetter,
{
    fn process(&mut self) -> Result<bool> {
        match &self.get_icb() {
            Some(..) => Ok(false),
            None => {
                if self.get_icb_aplica() {
                    if let Some(icb_tasa) = self.get_icb_tasa() {
                        let icb = self.get_cantidad() * *icb_tasa;
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
