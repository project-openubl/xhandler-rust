use regex::Regex;

use crate::{BOLETA_SERIE_REGEX, FACTURA_SERIE_REGEX};
use crate::catalogs::{Catalog, Catalog1};
use crate::models::traits::invoice::tipocomprobante::{
    InvoiceTipoComprobanteGetter, InvoiceTipoComprobanteSetter,
};
use crate::models::traits::serienumero::SerieNumeroGetter;

pub trait InvoiceTipoComprobanteEnrichRule {
    fn enrich(&mut self) -> bool;
}

impl<T> InvoiceTipoComprobanteEnrichRule for T
where
    T: InvoiceTipoComprobanteGetter + InvoiceTipoComprobanteSetter + SerieNumeroGetter,
{
    fn enrich(&mut self) -> bool {
        match &self.get_tipocomprobante() {
            Some(..) => false,
            None => {
                if Regex::new(FACTURA_SERIE_REGEX)
                    .unwrap()
                    .is_match(self.get_serienumero())
                {
                    self.set_tipocomprobante(Catalog1::Factura.code());

                    return true;
                } else if Regex::new(BOLETA_SERIE_REGEX)
                    .unwrap()
                    .is_match(self.get_serienumero())
                {
                    self.set_tipocomprobante(Catalog1::Boleta.code());

                    return true;
                }

                false
            }
        }
    }
}
