use anyhow::Result;
use rust_decimal::Decimal;

use crate::catalogs::{Catalog7, Catalog7Group, FromCode};
use crate::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::enricher::bounds::detalle::isc::{DetalleISCSetter, DetalleIscGetter};
use crate::enricher::bounds::detalle::isc_base_imponible::DetalleIscBaseImponibleGetter;
use crate::enricher::bounds::detalle::isc_tasa::DetalleIscTasaGetter;

pub trait DetalleISCProcessRule {
    fn process(&mut self) -> Result<bool>;
}

impl<T> DetalleISCProcessRule for T
where
    T: DetalleIscGetter
        + DetalleISCSetter
        + DetalleIscBaseImponibleGetter
        + DetalleIscTasaGetter
        + DetalleIgvTipoGetter,
{
    fn process(&mut self) -> Result<bool> {
        match (
            &self.get_isc(),
            &self.get_isc_base_imponible(),
            &self.get_isc_tasa(),
            &self.get_igv_tipo(),
        ) {
            (None, Some(isc_base_imponible), Some(isc_tasa), Some(igv_tipo)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
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
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}
