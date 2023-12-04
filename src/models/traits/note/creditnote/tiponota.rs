use crate::models::credit_note::CreditNote;

pub trait CreditNoteTipoGetter {
    fn get_tipo_nota_credito(&self) -> &Option<&'static str>;
}

pub trait CreditNoteTipoSetter {
    fn set_tipo_nota_credito(&mut self, val: &'static str);
}

impl CreditNoteTipoGetter for CreditNote {
    fn get_tipo_nota_credito(&self) -> &Option<&'static str> {
        &self.tipo_nota
    }
}

impl CreditNoteTipoSetter for CreditNote {
    fn set_tipo_nota_credito(&mut self, val: &'static str) {
        self.tipo_nota = Some(val);
    }
}
