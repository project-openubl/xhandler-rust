use anyhow::Result;
use log::trace;
use rust_decimal::Decimal;

use crate::xbuilder::catalogs::{Catalog7, Catalog7Group, FromCode};
use crate::xbuilder::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::xbuilder::enricher::bounds::detalle::isc_tasa::{
    DetalleIscTasaGetter, DetalleIscTasaSetter,
};
use crate::xbuilder::enricher::rules::phase1_fill::detalle::detalles::DetalleDefaults;

pub trait DetalleIscTasaFillRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> Result<bool>;
}

impl<T> DetalleIscTasaFillRule for T
where
    T: DetalleIscTasaGetter + DetalleIscTasaSetter + DetalleIgvTipoGetter,
{
    fn fill(&mut self, _: &DetalleDefaults) -> Result<bool> {
        match (&self.isc_tasa(), &self.igv_tipo()) {
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
                        trace!("DetalleIscTasaFillRule: isc_tasa changed to {tasa}");
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
