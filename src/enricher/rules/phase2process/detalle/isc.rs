use crate::models::traits::detalle::isc::{DetalleISCGetter, DetalleISCSetter};
use crate::models::traits::detalle::iscbaseimponible::DetalleISCBaseImponibleGetter;
use crate::models::traits::detalle::isctasa::DetalleISCTasaGetter;

pub trait DetalleISCProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetalleISCProcessRule for T
where
    T: DetalleISCGetter + DetalleISCSetter + DetalleISCBaseImponibleGetter + DetalleISCTasaGetter,
{
    fn process(&mut self) -> bool {
        match (
            &self.get_isc(),
            &self.get_iscbaseimponible(),
            &self.get_isctasa(),
        ) {
            (None, Some(isc_base_imponible), Some(isc_tasa)) => {
                let isc = isc_base_imponible * *isc_tasa as f64;
                self.set_isc(isc);
                true
            }
            _ => false,
        }
    }
}
