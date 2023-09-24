use crate::models::general::ICBTipo;
use crate::models::traits::detalle::cantidad::DetalleCantidadGetter;
use crate::models::traits::detalle::icb::{DetalleICBGetter, DetalleICBSetter};
use crate::models::traits::detalle::icbtasa::DetalleICBTasaGetter;

pub trait DetalleICBProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetalleICBProcessRule for T
where
    T: DetalleICBGetter + DetalleICBSetter + DetalleCantidadGetter + DetalleICBTasaGetter,
{
    fn process(&mut self) -> bool {
        match &self.get_icb() {
            ICBTipo::IcbAplica(icb_aplica) => {
                if icb_aplica == &true {
                    if let Some(icb_tasa) = self.get_icbtasa() {
                        let icb = self.get_cantidad() * *icb_tasa as f64;
                        self.set_icb(ICBTipo::IcbMonto(icb));
                        true
                    } else {
                        false
                    }
                } else {
                    self.set_icb(ICBTipo::IcbMonto(0f64));
                    true
                }
            }
            ICBTipo::IcbMonto(_) => false,
        }
    }
}
