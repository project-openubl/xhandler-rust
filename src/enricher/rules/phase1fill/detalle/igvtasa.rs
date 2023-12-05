use rust_decimal::Decimal;

use crate::catalogs::{Catalog7, Catalog7Group, FromCode};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::igvtasa::{DetalleIGVTasaGetter, DetalleIGVTasaSetter};
use crate::models::traits::detalle::igvtipo::DetalleIGVTipoGetter;

pub trait DetalleIGVTasaEnrichRule {
    fn fill(&mut self, defaults: &DetalleDefaults) -> bool;
}

impl<T> DetalleIGVTasaEnrichRule for T
where
    T: DetalleIGVTasaGetter + DetalleIGVTasaSetter + DetalleIGVTipoGetter,
{
    fn fill(&mut self, defaults: &DetalleDefaults) -> bool {
        match (self.get_igvtasa(), *self.get_igvtipo()) {
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
                    self.set_igvtasa(tasa);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
