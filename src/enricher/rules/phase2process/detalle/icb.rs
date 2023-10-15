use crate::models::traits::detalle::cantidad::DetalleCantidadGetter;
use crate::models::traits::detalle::icb::{DetalleICBGetter, DetalleICBSetter};
use crate::models::traits::detalle::icbaplica::DetalleICBAplicaGetter;
use crate::models::traits::detalle::icbtasa::DetalleICBTasaGetter;

pub trait DetalleICBProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetalleICBProcessRule for T
where
    T: DetalleICBGetter
        + DetalleICBSetter
        + DetalleICBAplicaGetter
        + DetalleCantidadGetter
        + DetalleICBTasaGetter,
{
    fn process(&mut self) -> bool {
        match &self.get_icb() {
            Some(..) => false,
            None => {
                if self.get_icbaplica() {
                    if let Some(icb_tasa) = self.get_icbtasa() {
                        let icb = self.get_cantidad() * *icb_tasa;
                        self.set_icb(icb);
                        true
                    } else {
                        false
                    }
                } else {
                    self.set_icb(0f64);
                    true
                }
            }
        }
    }
}
