use crate::models::common::Detalle;

pub trait DetalleIcbAplicaGetter {
    fn icb_aplica(&self) -> bool;
}

pub trait DetalleIcbAplicaSetter {
    fn set_icb_aplica(&mut self, val: bool);
}

impl DetalleIcbAplicaGetter for Detalle {
    fn icb_aplica(&self) -> bool {
        self.icb_aplica
    }
}

impl DetalleIcbAplicaSetter for Detalle {
    fn set_icb_aplica(&mut self, val: bool) {
        self.icb_aplica = val;
    }
}
