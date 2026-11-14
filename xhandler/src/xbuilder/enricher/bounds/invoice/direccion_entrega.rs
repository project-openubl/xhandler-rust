use crate::xbuilder::models::common::Direccion;
use crate::xbuilder::models::invoice::Invoice;

pub trait InvoiceDireccionEntregaGetter {
    fn direccion_entrega(&self) -> &Option<Direccion>;
}

impl InvoiceDireccionEntregaGetter for Invoice {
    fn direccion_entrega(&self) -> &Option<Direccion> {
        &self.direccion_entrega
    }
}
