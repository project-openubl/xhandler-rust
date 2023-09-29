use crate::models::traits::detalle::igv::{DetalleIGVGetter, DetalleIGVSetter};
use crate::models::traits::detalle::igvbaseimponible::DetalleIGVBaseImponibleGetter;
use crate::models::traits::detalle::igvtasa::DetalleIGVTasaGetter;

pub trait DetalleIGVProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetalleIGVProcessRule for T
where
    T: DetalleIGVGetter + DetalleIGVSetter + DetalleIGVBaseImponibleGetter + DetalleIGVTasaGetter,
{
    fn process(&mut self) -> bool {
        match (
            &self.get_igv(),
            &self.get_igvbaseimponible(),
            &self.get_igvtasa(),
        ) {
            (None, Some(igv_base_imponible), Some(igv_tasa)) => {
                let igv = igv_base_imponible * *igv_tasa as f64;
                self.set_igv(igv);
                true
            }
            _ => false,
        }
    }
}
