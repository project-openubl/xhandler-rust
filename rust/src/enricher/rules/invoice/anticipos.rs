use log::warn;
use regex::Regex;

use crate::catalogs::{Catalog, Catalog12, Catalog53};
use crate::models::traits::invoice::anticipos::{AnticipoGetter, AnticipoSetter, AnticiposGetter};
use crate::{BOLETA_SERIE_REGEX, FACTURA_SERIE_REGEX};

pub trait AnticiposRule {
    fn enrich_anticipos(&mut self) -> bool;
}

impl<T> AnticiposRule for T
where
    T: AnticiposGetter,
{
    fn enrich_anticipos(&mut self) -> bool {
        self.get_anticipos()
            .iter_mut()
            .map(|anticipo| {
                let results = vec![
                    anticipo.enrich_anticipo_tipo(),
                    anticipo.enrich_anticipo_comprobantetipo(),
                ];
                results.contains(&true)
            })
            .any(|changed| changed)
    }
}

//

pub trait AnticipoRule {
    fn enrich_anticipo_tipo(&mut self) -> bool;
    fn enrich_anticipo_comprobantetipo(&mut self) -> bool;
}

impl<T> AnticipoRule for T
where
    T: AnticipoGetter + AnticipoSetter,
{
    fn enrich_anticipo_tipo(&mut self) -> bool {
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

    fn enrich_anticipo_comprobantetipo(&mut self) -> bool {
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
                    warn!("No se pudo determinar el tipocomprobante de anticipo")
                }

                false
            }
        }
    }
}
