use rust_decimal::Decimal;

use crate::models::general::Detalle;

pub trait DetalleICBGetter {
    fn get_icb(&self) -> &Option<Decimal>;
}

pub trait DetalleICBSetter {
    fn set_icb(&mut self, val: Decimal);
}

impl DetalleICBGetter for Detalle {
    fn get_icb(&self) -> &Option<Decimal> {
        &self.icb
    }
}

impl DetalleICBSetter for Detalle {
    fn set_icb(&mut self, val: Decimal) {
        self.icb = Some(val);
    }
}
