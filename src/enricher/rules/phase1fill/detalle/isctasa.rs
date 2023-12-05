use log::trace;
use rust_decimal::Decimal;

use crate::catalogs::{Catalog7, Catalog7Group, FromCode};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::igvtipo::DetalleIGVTipoGetter;
use crate::models::traits::detalle::isctasa::{DetalleISCTasaGetter, DetalleISCTasaSetter};

pub trait DetalleISCTasaEnrichRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleISCTasaEnrichRule for T
where
    T: DetalleISCTasaGetter + DetalleISCTasaSetter + DetalleIGVTipoGetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> bool {
        match (&self.get_isctasa(), &self.get_igvtipo()) {
            (Some(isc_tasa), Some(igv_tipo)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    let tasa = if !catalog.onerosa() {
                        Decimal::ZERO
                    } else {
                        match catalog.group() {
                            Catalog7Group::Gravado | Catalog7Group::Gratuita => *isc_tasa,
                            Catalog7Group::Exonerado
                            | Catalog7Group::Inafecto
                            | Catalog7Group::Exportacion => Decimal::ZERO,
                        }
                    };

                    if &tasa != isc_tasa {
                        trace!("DetalleISCTasaEnrichRule: isc_tasa changed to {tasa}");
                        self.set_isctasa(tasa);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            (None, _) => {
                self.set_isctasa(Decimal::ZERO);
                true
            }
            _ => false,
        }
    }
}
