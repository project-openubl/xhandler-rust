use crate::models::credit_note::CreditNote;

pub trait TipoNotaCreditoGetter {
    fn get_tipo_nota_credito(&self) -> &Option<&'static str>;
}

pub trait TipoNotaCreditoSetter {
    fn set_tipo_nota_credito(&mut self, val: &'static str);
}

impl TipoNotaCreditoGetter for CreditNote {
    fn get_tipo_nota_credito(&self) -> &Option<&'static str> {
        &self.tipo_nota
    }
}

impl TipoNotaCreditoSetter for CreditNote {
    fn set_tipo_nota_credito(&mut self, val: &'static str) {
        self.tipo_nota = Some(val);
    }
}
