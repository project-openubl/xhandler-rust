use crate::models::common::Direccion;
use crate::models::invoice::Invoice;

pub trait DireccionEntregaGetter {
    fn get_direccionentrega(&self) -> &Option<Direccion>;
}

impl DireccionEntregaGetter for Invoice {
    fn get_direccionentrega(&self) -> &Option<Direccion> {
        &self.direccion_entrega
    }
}
