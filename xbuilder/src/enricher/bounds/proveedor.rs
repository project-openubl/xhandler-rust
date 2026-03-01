use crate::models::common::{Direccion, Proveedor};
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;
use crate::models::perception::Perception;
use crate::models::retention::Retention;
use crate::models::summary_documents::SummaryDocuments;
use crate::models::voided_documents::VoidedDocuments;

pub trait ProveedorGetter {
    fn get_proveedor(&self) -> &Proveedor;
}

pub trait ProveedorSetter {
    fn set_proveedor_direccion(&mut self, val: Direccion);
}

impl ProveedorGetter for Invoice {
    fn get_proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}

impl ProveedorGetter for CreditNote {
    fn get_proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}

impl ProveedorGetter for DebitNote {
    fn get_proveedor(&self) -> &Proveedor {
        &self.proveedor
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

impl ProveedorGetter for VoidedDocuments {
    fn get_proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}

impl ProveedorGetter for SummaryDocuments {
    fn get_proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}

impl ProveedorGetter for Perception {
    fn get_proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}

impl ProveedorGetter for Retention {
    fn get_proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}
