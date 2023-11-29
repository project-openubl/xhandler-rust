use crate::catalogs::{catalog7_value_of_code, Catalog7, Catalog7Group};
use crate::enricher::rules::phase1fill::detalle::detalles::DetalleDefaults;
use crate::models::traits::detalle::igvtasa::{DetalleIGVTasaGetter, DetalleIGVTasaSetter};
use crate::models::traits::detalle::igvtipo::DetalleIGVTipoGetter;
use rust_decimal_macros::dec;

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
                if let Some(catalog) = catalog7_value_of_code(igv_tipo) {
                    let tasa = match catalog {
                        Catalog7::GravadoIvap => defaults.ivap_tasa,
                        _ => match catalog.group() {
                            Catalog7Group::Gravado => defaults.igv_tasa,
                            Catalog7Group::Exonerado => dec!(0),
                            Catalog7Group::Inafecto => dec!(0),
                            Catalog7Group::Exportacion => dec!(0),
                            Catalog7Group::Gratuita => defaults.igv_tasa,
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
