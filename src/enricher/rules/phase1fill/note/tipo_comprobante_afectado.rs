use log::warn;
use regex::Regex;

use crate::{BOLETA_SERIE_REGEX, FACTURA_SERIE_REGEX};
use crate::catalogs::{Catalog, Catalog1};
use crate::enricher::bounds::note::tipo_comprobante_afectado::{
    NoteTipoComprobanteAfectadoGetter, NoteTipoComprobanteAfectadoSetter,
};
use crate::enricher::bounds::serie_numero::SerieNumeroGetter;

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
                    .is_match(self.get_serie_numero())
                {
                    self.set_tipo_comprobante_afectado(Catalog1::Factura.code());
                    true
                } else if Regex::new(BOLETA_SERIE_REGEX)
                    .unwrap()
                    .is_match(self.get_serie_numero())
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
