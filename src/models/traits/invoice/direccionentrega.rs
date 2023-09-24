use crate::models::common::Direccion;
use crate::models::invoice::Invoice;

pub trait InvoiceDireccionEntregaGetter {
    fn get_direccionentrega(&self) -> &Option<Direccion>;
}

impl InvoiceDireccionEntregaGetter for Invoice {
    fn get_direccionentrega(&self) -> &Option<Direccion> {
        &self.direccion_entrega
    }
}
