use crate::models::general::Detalle;

pub trait DetalleICBAplicaGetter {
    fn get_icbaplica(&self) -> bool;
}

pub trait DetalleICBAplicaSetter {
    fn set_icbaplica(&mut self, val: bool);
}

impl DetalleICBAplicaGetter for Detalle {
    fn get_icbaplica(&self) -> bool {
        self.icb_aplica
    }
}

impl DetalleICBAplicaSetter for Detalle {
    fn set_icbaplica(&mut self, val: bool) {
        self.icb_aplica = val;
    }
}
