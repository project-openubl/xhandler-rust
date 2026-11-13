use crate::models::debit_note::DebitNote;

pub trait DebitNoteTipoNotaGetter {
    fn tipo_nota(&self) -> &Option<String>;
}

pub trait DebitNoteTipoNotaSetter {
    fn set_tipo_nota(&mut self, val: &str);
}

impl DebitNoteTipoNotaGetter for DebitNote {
    fn tipo_nota(&self) -> &Option<String> {
        &self.tipo_nota
    }
}

impl DebitNoteTipoNotaSetter for DebitNote {
    fn set_tipo_nota(&mut self, val: &str) {
        self.tipo_nota = Some(val.to_string());
    }
}
