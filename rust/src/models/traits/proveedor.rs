use crate::models::common::{Direccion, Proveedor};
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait ProveedorGetter {
    fn get_proveedor(&self) -> &Proveedor;
    fn get_proveedor_direccion(&self) -> &Option<Direccion>;
}

pub trait ProveedorSetter {
    fn set_proveedor_direccion(&mut self, val: Direccion);
}

impl ProveedorGetter for Invoice {
    fn get_proveedor(&self) -> &Proveedor {
        &self.proveedor
    }

    fn get_proveedor_direccion(&self) -> &Option<Direccion> {
        &self.proveedor.direccion
    }
}

impl ProveedorGetter for CreditNote {
    fn get_proveedor(&self) -> &Proveedor {
        &self.proveedor
    }

    fn get_proveedor_direccion(&self) -> &Option<Direccion> {
        &self.proveedor.direccion
    }
}

impl ProveedorGetter for DebitNote {
    fn get_proveedor(&self) -> &Proveedor {
        &self.proveedor
    }

    fn get_proveedor_direccion(&self) -> &Option<Direccion> {
        &self.proveedor.direccion
    }
}

impl ProveedorSetter for Invoice {
    fn set_proveedor_direccion(&mut self, val: Direccion) {
        self.proveedor.direccion = Some(val);
    }
}

impl ProveedorSetter for CreditNote {
    fn set_proveedor_direccion(&mut self, val: Direccion) {
        self.proveedor.direccion = Some(val);
    }
}

impl ProveedorSetter for DebitNote {
    fn set_proveedor_direccion(&mut self, val: Direccion) {
        self.proveedor.direccion = Some(val);
    }
}
