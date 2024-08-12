use anyhow::Result;
use regex::Regex;

use crate::catalogs::{Catalog, Catalog1};
use crate::enricher::bounds::invoice::tipo_comprobante::{
    InvoiceTipoComprobanteGetter, InvoiceTipoComprobanteSetter,
};
use crate::enricher::bounds::serie_numero::SerieNumeroGetter;
use crate::{BOLETA_SERIE_REGEX, FACTURA_SERIE_REGEX};

pub trait InvoiceTipoComprobanteFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> InvoiceTipoComprobanteFillRule for T
where
    T: InvoiceTipoComprobanteGetter + InvoiceTipoComprobanteSetter + SerieNumeroGetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_tipo_comprobante() {
            Some(..) => Ok(false),
            None => {
                if Regex::new(FACTURA_SERIE_REGEX)?.is_match(self.get_serie_numero()) {
                    self.set_tipo_comprobante(Catalog1::Factura.code());

                    return Ok(true);
                } else if Regex::new(BOLETA_SERIE_REGEX)?.is_match(self.get_serie_numero()) {
                    self.set_tipo_comprobante(Catalog1::Boleta.code());

                    return Ok(true);
                }

                Ok(false)
            }
        }
    }
}
