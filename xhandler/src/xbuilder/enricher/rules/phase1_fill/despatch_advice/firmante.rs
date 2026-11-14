use anyhow::Result;

use crate::xbuilder::enricher::bounds::despatch_advice::DespatchAdviceRemitenteGetter;
use crate::xbuilder::enricher::bounds::firmante::{FirmanteGetter, FirmanteSetter};
use crate::xbuilder::models::common::Firmante;

/// Fills firmante from remitente (not proveedor) for DespatchAdvice.
pub trait DespatchAdviceFirmanteFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> DespatchAdviceFirmanteFillRule for T
where
    T: FirmanteGetter + FirmanteSetter + DespatchAdviceRemitenteGetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.firmante() {
            Some(_) => Ok(false),
            None => {
                let firmante = Firmante {
                    ruc: self.remitente().ruc.clone(),
                    razon_social: self.remitente().razon_social.clone(),
                };
                self.set_firmante(firmante);
                Ok(true)
            }
        }
    }
}
