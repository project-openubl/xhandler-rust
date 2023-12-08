use crate::enricher::bounds::firmante::{FirmanteGetter, FirmanteSetter};
use crate::enricher::bounds::proveedor::ProveedorGetter;
use crate::models::common::Firmante;

pub trait FirmanteFillRule {
    fn fill(&mut self) -> bool;
}

impl<T> FirmanteFillRule for T
where
    T: FirmanteGetter + FirmanteSetter + ProveedorGetter,
{
    fn fill(&mut self) -> bool {
        match &self.get_firmante() {
            Some(..) => false,
            None => {
                let firmante = Firmante {
                    ruc: self.get_proveedor().ruc,
                    razon_social: self.get_proveedor().razon_social,
                };
                self.set_firmante(firmante);
                true
            }
        }
    }
}
