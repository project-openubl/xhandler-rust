use crate::xbuilder::models::credit_note::CreditNote;

pub trait CreditNoteTipoNotaGetter {
    fn tipo_nota(&self) -> &Option<String>;
}

pub trait CreditNoteTipoNotaSetter {
    fn set_tipo_nota(&mut self, val: &str);
}

impl CreditNoteTipoNotaGetter for CreditNote {
    fn tipo_nota(&self) -> &Option<String> {
        &self.tipo_nota
    }
}

impl CreditNoteTipoNotaSetter for CreditNote {
    fn set_tipo_nota(&mut self, val: &str) {
        self.tipo_nota = Some(val.to_string());
    }
}
