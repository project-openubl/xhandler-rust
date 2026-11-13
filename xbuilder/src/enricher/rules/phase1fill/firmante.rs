use anyhow::Result;

use crate::enricher::bounds::firmante::{FirmanteGetter, FirmanteSetter};
use crate::enricher::bounds::proveedor::ProveedorGetter;
use crate::models::common::Firmante;

pub trait FirmanteFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> FirmanteFillRule for T
where
    T: FirmanteGetter + FirmanteSetter + ProveedorGetter,
{
    /// Fills the firmante, or its empty fields, from the proveedor
    fn fill(&mut self) -> Result<bool> {
        let current = self.get_firmante().as_ref();
        let proveedor = self.get_proveedor();
        let or_proveedor = |value: Option<&String>, default: &String| {
            value
                .filter(|value| !value.is_empty())
                .unwrap_or(default)
                .clone()
        };

        let firmante = Firmante {
            ruc: or_proveedor(current.map(|f| &f.ruc), &proveedor.ruc),
            razon_social: or_proveedor(current.map(|f| &f.razon_social), &proveedor.razon_social),
        };
        let changed = current.is_none_or(|current| {
            current.ruc != firmante.ruc || current.razon_social != firmante.razon_social
        });
        if changed {
            self.set_firmante(firmante);
        }
        Ok(changed)
    }
}
