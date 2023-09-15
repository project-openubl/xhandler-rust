use regex::Regex;

use crate::{BOLETA_SERIE_REGEX, FACTURA_SERIE_REGEX};
use crate::catalogs::{Catalog, Catalog1};
use crate::models::traits::note::tipocomprobanteafectado::{
    TipoComprobanteAfectadoGetter, TipoComprobanteAfectadoSetter,
};
use crate::models::traits::serienumero::SerieNumeroGetter;

pub trait TipoComprobanteAfectadoRule {
    fn enrich_tipo_comprobante_afectado(&mut self) -> bool;
}

impl<T> TipoComprobanteAfectadoRule for T
where
    T: TipoComprobanteAfectadoGetter + TipoComprobanteAfectadoSetter + SerieNumeroGetter,
{
    fn enrich_tipo_comprobante_afectado(&mut self) -> bool {
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
                    // Could not determine tipo comprobante so no change is applied
                    false
                }
            }
        }
    }
}
