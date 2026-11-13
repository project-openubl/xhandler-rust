use anyhow::Result;

use crate::enricher::bounds::despatch_advice::{
    DespatchAdviceSerieNumeroGetter, DespatchAdviceTipoComprobanteGetter,
    DespatchAdviceTipoComprobanteSetter,
};

pub trait DespatchAdviceTipoComprobanteFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> DespatchAdviceTipoComprobanteFillRule for T
where
    T: DespatchAdviceTipoComprobanteGetter
        + DespatchAdviceTipoComprobanteSetter
        + DespatchAdviceSerieNumeroGetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_tipo_comprobante() {
            Some(_) => Ok(false),
            None => {
                let tipo = match self.get_serie_numero().chars().next() {
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
