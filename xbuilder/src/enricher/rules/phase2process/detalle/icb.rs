use rust_decimal::Decimal;

use crate::enricher::bounds::detalle::cantidad::DetalleCantidadGetter;
use crate::enricher::bounds::detalle::icb::{DetalleIcbGetter, DetalleIcbSetter};
use crate::enricher::bounds::detalle::icb_aplica::DetalleICBAplicaGetter;
use crate::enricher::bounds::detalle::icb_tasa::DetalleIcbTasaGetter;

pub trait DetalleICBProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetalleICBProcessRule for T
where
    T: DetalleIcbGetter
        + DetalleIcbSetter
        + DetalleICBAplicaGetter
        + DetalleCantidadGetter
        + DetalleIcbTasaGetter,
{
    fn process(&mut self) -> bool {
        match &self.get_icb() {
            Some(..) => false,
            None => {
                if self.get_icb_aplica() {
                    if let Some(icb_tasa) = self.get_icb_tasa() {
                        let icb = self.get_cantidad() * *icb_tasa;
                        self.set_icb(icb);
                        true
                    } else {
                        false
                    }
                } else {
                    self.set_icb(Decimal::ZERO);
                    true
                }
            }
        }
    }
}
