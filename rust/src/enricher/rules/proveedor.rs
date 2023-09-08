use crate::content::models::common::{Direccion, DireccionBuilder, Proveedor};
use crate::content::models::invoice::Invoice;

pub trait ProveedorRule {
    fn enrich_proveedor(&mut self) -> bool;
}

pub trait ProveedorGetterSetter {
    fn get_proveedor(&self) -> &Proveedor;
}

impl<T> ProveedorRule for T
    where T: ProveedorGetterSetter, {
    fn enrich_proveedor(&mut self) -> bool {
        match &self.get_proveedor().direccion {
            Some(..) => {}
            None => {
                self.get_proveedor().direccion = DireccionBuilder::default()
                    .build()
                    .unwrap();
            }
        }
        false
    }
}

impl ProveedorGetterSetter for Invoice {
    fn get_proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}
