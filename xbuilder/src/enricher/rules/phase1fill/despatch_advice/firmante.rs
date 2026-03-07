use anyhow::Result;

use crate::enricher::bounds::despatch_advice::DespatchAdviceRemitenteGetter;
use crate::enricher::bounds::firmante::{FirmanteGetter, FirmanteSetter};
use crate::models::common::Firmante;

/// Fills firmante from remitente (not proveedor) for DespatchAdvice.
pub trait DespatchAdviceFirmanteFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> DespatchAdviceFirmanteFillRule for T
where
    T: FirmanteGetter + FirmanteSetter + DespatchAdviceRemitenteGetter,
{
    fn fill(&mut self) -> Result<bool> {
        match &self.get_firmante() {
            Some(_) => Ok(false),
            None => {
                let firmante = Firmante {
                    ruc: self.get_remitente().ruc.clone(),
                    razon_social: self.get_remitente().razon_social.clone(),
                };
                self.set_firmante(firmante);
                Ok(true)
            }
        }
    }
}
