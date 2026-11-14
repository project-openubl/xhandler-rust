use anyhow::Result;

use crate::xbuilder::enricher::bounds::despatch_advice::{
    DespatchAdviceTipoComprobanteGetter, DespatchAdviceTipoComprobanteSetter,
};
use crate::xbuilder::enricher::bounds::serie_numero::SerieNumeroGetter;

pub trait DespatchAdviceTipoComprobanteFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> DespatchAdviceTipoComprobanteFillRule for T
where
    T: DespatchAdviceTipoComprobanteGetter
        + DespatchAdviceTipoComprobanteSetter
        + SerieNumeroGetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.tipo_comprobante() {
            Some(_) => Ok(false),
            None => {
                let tipo = match self.serie_numero().chars().next() {
                    Some('T') | Some('t') => Some("09"),
                    Some('V') | Some('v') => Some("31"),
                    _ => None,
                };
                if let Some(val) = tipo {
                    self.set_tipo_comprobante(val);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
}
