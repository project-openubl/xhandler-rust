use regex::Regex;

use crate::catalogs::{Catalog, Catalog1};
use crate::models::traits::invoice::tipocomprobante::{
    TipoComprobanteGetter, TipoComprobanteSetter,
};
use crate::models::traits::serienumero::SerieNumeroGetter;
use crate::{BOLETA_SERIE_REGEX, FACTURA_SERIE_REGEX};

pub trait TipoComprobanteRule {
    fn enrich_tipocomprobante(&mut self) -> bool;
}

impl<T> TipoComprobanteRule for T
where
    T: TipoComprobanteGetter + TipoComprobanteSetter + SerieNumeroGetter,
{
    fn enrich_tipocomprobante(&mut self) -> bool {
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
