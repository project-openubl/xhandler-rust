use anyhow::Result;
use log::warn;
use regex::Regex;

use crate::catalogs::{Catalog, Catalog1};
use crate::enricher::bounds::note::tipo_comprobante_afectado::{
    NoteComprobanteAfectadoTipoGetter, NoteComprobanteAfectadoTipoSetter,
};
use crate::enricher::bounds::serie_numero::SerieNumeroGetter;
use crate::{BOLETA_SERIE_REGEX, FACTURA_SERIE_REGEX};

pub trait NoteComprobanteAfectadoTipoFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> NoteComprobanteAfectadoTipoFillRule for T
where
    T: NoteComprobanteAfectadoTipoGetter + NoteComprobanteAfectadoTipoSetter + SerieNumeroGetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.comprobante_afectado_tipo() {
            Some(..) => Ok(false),
            None => {
                if Regex::new(FACTURA_SERIE_REGEX)?.is_match(self.serie_numero()) {
                    self.set_comprobante_afectado_tipo(Catalog1::Factura.code());
                    Ok(true)
                } else if Regex::new(BOLETA_SERIE_REGEX)?.is_match(self.serie_numero()) {
                    self.set_comprobante_afectado_tipo(Catalog1::Boleta.code());
                    Ok(true)
                } else {
                    warn!(
                        "could not infer comprobante_afectado_tipo from comprobante_afectado_serie_numero"
                    );
                    Ok(false)
                }
            }
        }
    }
}
