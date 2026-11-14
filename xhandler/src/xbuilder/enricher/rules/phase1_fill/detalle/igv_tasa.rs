use anyhow::Result;
use rust_decimal::Decimal;

use crate::xbuilder::catalogs::{Catalog7, Catalog7Group, FromCode};
use crate::xbuilder::enricher::bounds::detalle::igv_tasa::{
    DetalleIgvTasaGetter, DetalleIgvTasaSetter,
};
use crate::xbuilder::enricher::bounds::detalle::igv_tipo::DetalleIgvTipoGetter;
use crate::xbuilder::enricher::rules::phase1_fill::detalle::detalles::DetalleDefaults;

pub trait DetalleIgvTasaFillRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> Result<bool>;
}

impl<T> DetalleIgvTasaFillRule for T
where
    T: DetalleIgvTasaGetter + DetalleIgvTasaSetter + DetalleIgvTipoGetter,
{
    fn fill(&mut self, defaults: &DetalleDefaults) -> Result<bool> {
        match (self.igv_tasa(), self.igv_tipo().as_deref()) {
            (None, Some(igv_tipo)) => {
                if let Ok(catalog) = Catalog7::from_code(igv_tipo) {
                    let tasa = match catalog {
                        Catalog7::GravadoIvap => defaults.ivap_tasa,
                        _ => match catalog.group() {
                            Catalog7Group::Gravado | Catalog7Group::Gratuita => defaults.igv_tasa,
                            Catalog7Group::Exonerado
                            | Catalog7Group::Inafecto
                            | Catalog7Group::Exportacion => Decimal::ZERO,
                        },
                    };
                    self.set_igv_tasa(tasa);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}
