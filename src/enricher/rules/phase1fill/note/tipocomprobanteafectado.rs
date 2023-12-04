use log::warn;
use regex::Regex;

use crate::catalogs::{Catalog, Catalog1};
use crate::models::traits::note::tipocomprobanteafectado::{
    NoteTipoComprobanteAfectadoGetter, NoteTipoComprobanteAfectadoSetter,
};
use crate::models::traits::serienumero::SerieNumeroGetter;
use crate::{BOLETA_SERIE_REGEX, FACTURA_SERIE_REGEX};

pub trait NoteTipoComprobanteAfectadoEnrichRule {
    fn fill(&mut self) -> bool;
}

impl<T> NoteTipoComprobanteAfectadoEnrichRule for T
where
    T: NoteTipoComprobanteAfectadoGetter + NoteTipoComprobanteAfectadoSetter + SerieNumeroGetter,
{
    fn fill(&mut self) -> bool {
        match &self.get_tipo_comprobante_afectado() {
            Some(..) => false,
            None => {
                if Regex::new(FACTURA_SERIE_REGEX)
                    .unwrap()
                    .is_match(self.get_serienumero())
                {
                    self.set_tipo_comprobante_afectado(Catalog1::Factura.code());
                    true
                } else if Regex::new(BOLETA_SERIE_REGEX)
                    .unwrap()
                    .is_match(self.get_serienumero())
                {
                    self.set_tipo_comprobante_afectado(Catalog1::Boleta.code());
                    true
                } else {
                    warn!(
                        "tipo_comprobante_afectado no pudo ser inferido a partir de serie-numero"
                    );
                    false
                }
            }
        }
    }
}