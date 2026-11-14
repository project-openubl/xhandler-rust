use crate::xbuilder::models::common::{Direccion, Proveedor};
use crate::xbuilder::models::credit_note::CreditNote;
use crate::xbuilder::models::debit_note::DebitNote;
use crate::xbuilder::models::invoice::Invoice;
use crate::xbuilder::models::perception_retention::{Perception, Retention};
use crate::xbuilder::models::summary_documents::SummaryDocuments;
use crate::xbuilder::models::voided_documents::VoidedDocuments;

pub trait ProveedorGetter {
    fn proveedor(&self) -> &Proveedor;
}

pub trait ProveedorDireccionSetter {
    fn set_proveedor_direccion(&mut self, val: Direccion);
}

impl ProveedorGetter for Invoice {
    fn proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}

impl ProveedorGetter for CreditNote {
    fn proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}

impl ProveedorGetter for DebitNote {
    fn proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}

impl ProveedorDireccionSetter for Invoice {
    fn set_proveedor_direccion(&mut self, val: Direccion) {
        self.proveedor.direccion = Some(val);
    }
}

impl ProveedorDireccionSetter for CreditNote {
    fn set_proveedor_direccion(&mut self, val: Direccion) {
        self.proveedor.direccion = Some(val);
    }
}

impl ProveedorDireccionSetter for DebitNote {
    fn set_proveedor_direccion(&mut self, val: Direccion) {
        self.proveedor.direccion = Some(val);
    }
}

impl ProveedorGetter for VoidedDocuments {
    fn proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}

impl ProveedorGetter for SummaryDocuments {
    fn proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}

impl ProveedorGetter for Perception {
    fn proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}

impl ProveedorGetter for Retention {
    fn proveedor(&self) -> &Proveedor {
        &self.proveedor
    }
}
