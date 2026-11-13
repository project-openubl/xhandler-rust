use anyhow::Result;
use log::warn;
use regex::Regex;

use crate::catalogs::{code_for_name, Catalog, Catalog12, Catalog53};
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
        let result = self.anticipos_mut().iter_mut().any(|anticipo| {
            let results = [
                AnticipoTipoFillRule::fill(anticipo).unwrap_or_default(),
                AnticipoComprobanteTipoFillRule::fill(anticipo).unwrap_or_default(),
            ];
            results.contains(&true)
        });
        Ok(result)
    }
}

//

pub trait AnticipoTipoFillRule {
    fn fill(&mut self) -> Result<bool>;
}

pub trait AnticipoComprobanteTipoFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> AnticipoTipoFillRule for T
where
    T: AnticipoGetter + AnticipoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match self.tipo() {
            Some(value) => match code_for_name::<Catalog53>(value) {
                Some(code) => {
                    self.set_tipo(&code);
                    Ok(true)
                }
                None => Ok(false),
            },
            None => {
                self.set_tipo(
                    Catalog53::DescuentoGlobalPorAnticiposGravadosAfectaBaseImponibleIgvIvap.code(),
                );
                Ok(false)
            }
        }
    }
}

impl<T> AnticipoComprobanteTipoFillRule for T
where
    T: AnticipoGetter + AnticipoSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match self.comprobante_tipo() {
            Some(value) => match code_for_name::<Catalog12>(value) {
                Some(code) => {
                    self.set_comprobante_tipo(&code);
                    Ok(true)
                }
                None => Ok(false),
            },
            None => {
                if Regex::new(FACTURA_SERIE_REGEX)?.is_match(self.comprobante_serie_numero()) {
                    self.set_comprobante_tipo(Catalog12::FacturaEmitidaPorAnticipos.code());
                    return Ok(true);
                } else if Regex::new(BOLETA_SERIE_REGEX)?.is_match(self.comprobante_serie_numero())
                {
                    self.set_comprobante_tipo(Catalog12::BoletaDeVentaEmitidaPorAnticipos.code());
                    return Ok(true);
                } else {
                    warn!("could not infer comprobante_tipo of anticipo from comprobante_serie_numero")
                }

                Ok(false)
            }
        }
    }
}
