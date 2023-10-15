use crate::models::common::Firmante;
use crate::models::traits::firmante::{FirmanteGetter, FirmanteSetter};
use crate::models::traits::proveedor::ProveedorGetter;

pub trait FirmanteEnrichRule {
    fn fill(&mut self) -> bool;
}

impl<T> FirmanteEnrichRule for T
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
