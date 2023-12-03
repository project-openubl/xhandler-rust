use rust_decimal::Decimal;

use crate::catalogs::{catalog7_value_of_code, Catalog7Group};
use crate::models::traits::detalle::igvtipo::DetalleIGVTipoGetter;
use crate::models::traits::detalle::isc::{DetalleISCGetter, DetalleISCSetter};
use crate::models::traits::detalle::iscbaseimponible::DetalleISCBaseImponibleGetter;
use crate::models::traits::detalle::isctasa::DetalleISCTasaGetter;

pub trait DetalleISCProcessRule {
    fn process(&mut self) -> bool;
}

impl<T> DetalleISCProcessRule for T
where
    T: DetalleISCGetter
        + DetalleISCSetter
        + DetalleISCBaseImponibleGetter
        + DetalleISCTasaGetter
        + DetalleIGVTipoGetter,
{
    fn process(&mut self) -> bool {
        match (
            &self.get_isc(),
            &self.get_iscbaseimponible(),
            &self.get_isctasa(),
            &self.get_igvtipo(),
        ) {
            (None, Some(isc_base_imponible), Some(isc_tasa), Some(igv_tipo)) => {
                if let Some(catalog) = catalog7_value_of_code(igv_tipo) {
                    let tasa = if catalog.onerosa() {
                        match catalog.group() {
                            Catalog7Group::Gravado => *isc_tasa,
                            Catalog7Group::Exonerado => Decimal::ZERO,
                            Catalog7Group::Inafecto => Decimal::ZERO,
                            Catalog7Group::Exportacion => Decimal::ZERO,
                            Catalog7Group::Gratuita => *isc_tasa,
                        }
                    } else {
                        Decimal::ZERO
                    };

                    let isc = isc_base_imponible * tasa;
                    self.set_isc(isc);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
