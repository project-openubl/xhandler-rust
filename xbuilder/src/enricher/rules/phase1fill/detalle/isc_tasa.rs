use anyhow::Result;
use log::trace;
use rust_decimal::Decimal;

use crate::catalogs::{Catalog7, Catalog7Group, FromCode};
use crate::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::enricher::bounds::detalle::isc_tasa::{DetalleIscTasaGetter, DetalleIscTasaSetter};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;

pub trait DetalleISCTasaFillRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> Result<bool>;
}

impl<T> DetalleISCTasaFillRule for T
where
    T: DetalleIscTasaGetter + DetalleIscTasaSetter + DetalleIgvTipoGetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> Result<bool> {
        match (&self.get_isc_tasa(), &self.get_igv_tipo()) {
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
                        self.set_isc_tasa(tasa);
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            (None, _) => {
                self.set_isc_tasa(Decimal::ZERO);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
