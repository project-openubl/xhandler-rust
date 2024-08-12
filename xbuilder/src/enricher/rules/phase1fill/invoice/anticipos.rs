use anyhow::Result;
use log::warn;
use regex::Regex;

use crate::catalogs::{Catalog, Catalog12, Catalog53};
use crate::enricher::bounds::invoice::anticipos::{
    AnticipoGetter, AnticipoSetter, InvoiceAnticiposGetter,
};
use crate::{BOLETA_SERIE_REGEX, FACTURA_SERIE_REGEX};

pub trait InvoiceAnticiposFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> InvoiceAnticiposFillRule for T
where
    T: InvoiceAnticiposGetter,
{
    fn fill(&mut self) -> Result<bool> {
        let result = self
            .get_anticipos()
            .iter_mut()
            .map(|anticipo| {
                let results = [
                    AnticipoTipoRule::fill(anticipo).unwrap_or_default(),
                    AnticipoComprobanteTipoRule::fill(anticipo).unwrap_or_default(),
                ];
                results.contains(&true)
            })
            .any(|changed| changed);
        Ok(result)
    }
}

//

pub trait AnticipoTipoRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait AnticipoComprobanteTipoRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> AnticipoTipoRule for T
where
    T: AnticipoGetter + AnticipoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match self.get_tipo() {
            Some(..) => Ok(false),
            None => {
                self.set_tipo(
                    Catalog53::DescuentoGlobalPorAnticiposGravadosAfectaBaseImponibleIgvIvap.code(),
                );
                Ok(false)
            }
        }
    }
}

impl<T> AnticipoComprobanteTipoRule for T
where
    T: AnticipoGetter + AnticipoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match self.get_comprobante_tipo() {
            Some(..) => Ok(false),
            None => {
                if Regex::new(FACTURA_SERIE_REGEX)?.is_match(self.get_comprobante_serie_numero()) {
                    self.set_comprobante_tipo(Catalog12::FacturaEmitidaPorAnticipos.code());
                    return Ok(true);
                } else if Regex::new(BOLETA_SERIE_REGEX)?
                    .is_match(self.get_comprobante_serie_numero())
                {
                    self.set_comprobante_tipo(Catalog12::BoletaDeVentaEmitidaPorAnticipos.code());
                    return Ok(true);
                } else {
                    warn!("No se pudo determinar el tipocomprobante de detalle")
                }

                Ok(false)
            }
        }
    }
}
