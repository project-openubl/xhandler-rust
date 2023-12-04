use crate::models::debit_note::DebitNote;

pub trait DebitNoteTipoGetter {
    fn get_tipo_nota_debito(&self) -> &Option<&'static str>;
}

pub trait DebitNoteTipoSetter {
    fn set_tipo_nota_debito(&mut self, val: &'static str);
}

impl DebitNoteTipoGetter for DebitNote {
    fn get_tipo_nota_debito(&self) -> &Option<&'static str> {
        &self.tipo_nota
    }
}

impl DebitNoteTipoSetter for DebitNote {
    fn set_tipo_nota_debito(&mut self, val: &'static str) {
        self.tipo_nota = Some(val);
    }
}
