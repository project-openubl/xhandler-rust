use crate::models::common::Detalle;

pub trait DetalleICBAplicaGetter {
    fn get_icb_aplica(&self) -> bool;
}

pub trait DetalleIcbAplicaSetter {
    fn set_icb_aplica(&mut self, val: bool);
}

impl DetalleICBAplicaGetter for Detalle {
    fn get_icb_aplica(&self) -> bool {
        self.icb_aplica
    }
}

impl DetalleIcbAplicaSetter for Detalle {
    fn set_icb_aplica(&mut self, val: bool) {
        self.icb_aplica = val;
    }
}
