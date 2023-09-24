use crate::models::invoice::Invoice;

pub trait TipoOperacionGetter {
    fn get_tipooperacion(&self) -> &Option<&'static str>;
}

pub trait TipoOperacionSetter {
    fn set_tipooperacion(&mut self, val: &'static str);
}

impl TipoOperacionGetter for Invoice {
    fn get_tipooperacion(&self) -> &Option<&'static str> {
        &self.tipo_operacion
    }
}

impl TipoOperacionSetter for Invoice {
    fn set_tipooperacion(&mut self, val: &'static str) {
        self.tipo_operacion = Some(val);
    }
}
