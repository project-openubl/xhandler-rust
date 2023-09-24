use log::warn;
use regex::Regex;

use crate::catalogs::{Catalog, Catalog12, Catalog53};
use crate::models::traits::invoice::anticipos::{
    AnticipoGetter, AnticipoSetter, InvoiceAnticiposGetter,
};
use crate::{BOLETA_SERIE_REGEX, FACTURA_SERIE_REGEX};

pub trait InvoiceAnticiposRule {
    fn enrich(&mut self) -> bool;
}

impl<T> InvoiceAnticiposRule for T
where
    T: InvoiceAnticiposGetter,
{
    fn enrich(&mut self) -> bool {
        self.get_anticipos()
            .iter_mut()
            .map(|anticipo| {
                let results = vec![
                    AnticipoTipoRule::enrich(anticipo),
                    AnticipoComprobanteTipoRule::enrich(anticipo),
                ];
                results.contains(&true)
            })
            .any(|changed| changed)
    }
}

//

pub trait AnticipoTipoRule {
    fn enrich(&mut self) -> bool;
}

pub trait AnticipoComprobanteTipoRule {
    fn enrich(&mut self) -> bool;
}

impl<T> AnticipoTipoRule for T
where
    T: AnticipoGetter + AnticipoSetter,
{
    fn enrich(&mut self) -> bool {
        match self.get_tipo() {
            Some(..) => false,
            None => {
                self.set_tipo(
                    Catalog53::DescuentoGlobalPorAnticiposGravadosAfectaBaseImponibleIgvIvap.code(),
                );
                false
            }
        }
    }
}

impl<T> AnticipoComprobanteTipoRule for T
where
    T: AnticipoGetter + AnticipoSetter,
{
    fn enrich(&mut self) -> bool {
        match self.get_comprobante_tipo() {
            Some(..) => false,
            None => {
                if Regex::new(FACTURA_SERIE_REGEX)
                    .unwrap()
                    .is_match(self.get_comprobante_serie_numero())
                {
                    self.set_comprobante_tipo(Catalog12::FacturaEmitidaPorAnticipos.code());
                    return true;
                } else if Regex::new(BOLETA_SERIE_REGEX)
                    .unwrap()
                    .is_match(self.get_comprobante_serie_numero())
                {
                    self.set_comprobante_tipo(Catalog12::BoletaDeVentaEmitidaPorAnticipos.code());
                    return true;
                } else {
                    warn!("No se pudo determinar el tipocomprobante de detalle")
                }

                false
            }
        }
    }
}
