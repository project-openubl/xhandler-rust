use crate::models::general::{Detalle, ICBTipo};

pub trait DetalleICBGetter {
    fn get_icb(&self) -> &ICBTipo;
}

pub trait DetalleICBSetter {
    fn set_icb(&mut self, val: ICBTipo);
}

impl DetalleICBGetter for Detalle {
    fn get_icb(&self) -> &ICBTipo {
        &self.icb
    }
}

impl DetalleICBSetter for Detalle {
    fn set_icb(&mut self, val: ICBTipo) {
        self.icb = val;
    }
}
