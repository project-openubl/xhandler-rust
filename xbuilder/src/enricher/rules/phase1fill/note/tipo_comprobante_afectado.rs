use log::warn;
use regex::Regex;

use crate::catalogs::{Catalog, Catalog1};
use crate::enricher::bounds::note::tipo_comprobante_afectado::{
    NoteTipoComprobanteAfectadoGetter, NoteTipoComprobanteAfectadoSetter,
};
use crate::enricher::bounds::serie_numero::SerieNumeroGetter;
use crate::{BOLETA_SERIE_REGEX, FACTURA_SERIE_REGEX};

pub trait NoteComprobanteAfectadoTipoFillRule {
    fn fill(&mut self) -> bool;
}

impl<T> NoteComprobanteAfectadoTipoFillRule for T
where
    T: NoteTipoComprobanteAfectadoGetter + NoteTipoComprobanteAfectadoSetter + SerieNumeroGetter,
{
    fn fill(&mut self) -> bool {
        match &self.get_comprobante_afectado_tipo() {
            Some(..) => false,
            None => {
                if Regex::new(FACTURA_SERIE_REGEX)
                    .unwrap()
                    .is_match(self.get_serie_numero())
                {
                    self.set_comprobante_afectado_tipo(Catalog1::Factura.code());
                    true
                } else if Regex::new(BOLETA_SERIE_REGEX)
                    .unwrap()
                    .is_match(self.get_serie_numero())
                {
                    self.set_comprobante_afectado_tipo(Catalog1::Boleta.code());
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
