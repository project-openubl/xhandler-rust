use crate::models::credit_note::CreditNote;

pub trait CreditNoteTipoGetter {
    fn get_tipo_nota(&self) -> &Option<String>;
}

pub trait CreditNoteTipoSetter {
    fn set_tipo_nota(&mut self, val: &str);
}

impl CreditNoteTipoGetter for CreditNote {
    fn get_tipo_nota(&self) -> &Option<String> {
        &self.tipo_nota
    }
}

impl CreditNoteTipoSetter for CreditNote {
    fn set_tipo_nota(&mut self, val: &str) {
        self.tipo_nota = Some(val.to_string());
    }
}
