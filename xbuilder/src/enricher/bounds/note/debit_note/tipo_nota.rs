use crate::models::debit_note::DebitNote;

pub trait DebitNoteTipoGetter {
    fn get_tipo_nota(&self) -> &Option<String>;
}

pub trait DebitNoteTipoSetter {
    fn set_tipo_nota(&mut self, val: &str);
}

impl DebitNoteTipoGetter for DebitNote {
    fn get_tipo_nota(&self) -> &Option<String> {
        &self.tipo_nota
    }
}

impl DebitNoteTipoSetter for DebitNote {
    fn set_tipo_nota(&mut self, val: &str) {
        self.tipo_nota = Some(val.to_string());
    }
}
