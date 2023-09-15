use crate::models::debit_note::DebitNote;

pub trait TipoNotaDebitoGetter {
    fn get_tipo_nota_debito(&self) -> &Option<&'static str>;
}

pub trait TipoNotaDebitoSetter {
    fn set_tipo_nota_debito(&mut self, val: &'static str);
}

impl TipoNotaDebitoGetter for DebitNote {
    fn get_tipo_nota_debito(&self) -> &Option<&'static str> {
        &self.tipo_nota
    }
}

impl TipoNotaDebitoSetter for DebitNote {
    fn set_tipo_nota_debito(&mut self, val: &'static str) {
        self.tipo_nota = Some(val);
    }
}
