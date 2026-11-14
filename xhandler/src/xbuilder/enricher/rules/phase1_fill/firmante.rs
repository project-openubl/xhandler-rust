use anyhow::Result;

use crate::xbuilder::enricher::bounds::firmante::{FirmanteGetter, FirmanteSetter};
use crate::xbuilder::enricher::bounds::proveedor::ProveedorGetter;

pub trait FirmanteFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> FirmanteFillRule for T
where
    T: FirmanteGetter + FirmanteSetter + ProveedorGetter,
{
    /// Fills the firmante, or its empty fields, from the proveedor
    fn fill(&mut self) -> Result<bool> {
        let proveedor = self.proveedor();
        let mut firmante = self.firmante().clone().unwrap_or_default();
        if firmante.ruc.is_empty() {
            firmante.ruc = proveedor.ruc.clone();
        }
        if firmante.razon_social.is_empty() {
            firmante.razon_social = proveedor.razon_social.clone();
        }

        let changed = self.firmante().as_ref() != Some(&firmante);
        if changed {
            self.set_firmante(firmante);
        }
        Ok(changed)
    }
}
