use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait ClienteTipoDocumentoIdentidadGetter {
    fn cliente_tipo_documento_identidad(&self) -> &str;
}

pub trait ClienteTipoDocumentoIdentidadSetter {
    fn set_cliente_tipo_documento_identidad(&mut self, val: &str);
}

impl ClienteTipoDocumentoIdentidadGetter for Invoice {
    fn cliente_tipo_documento_identidad(&self) -> &str {
        &self.cliente.tipo_documento_identidad
    }
}

impl ClienteTipoDocumentoIdentidadGetter for CreditNote {
    fn cliente_tipo_documento_identidad(&self) -> &str {
        &self.cliente.tipo_documento_identidad
    }
}

impl ClienteTipoDocumentoIdentidadGetter for DebitNote {
    fn cliente_tipo_documento_identidad(&self) -> &str {
        &self.cliente.tipo_documento_identidad
    }
}

impl ClienteTipoDocumentoIdentidadSetter for Invoice {
    fn set_cliente_tipo_documento_identidad(&mut self, val: &str) {
        self.cliente.tipo_documento_identidad = val.to_string();
    }
}

impl ClienteTipoDocumentoIdentidadSetter for CreditNote {
    fn set_cliente_tipo_documento_identidad(&mut self, val: &str) {
        self.cliente.tipo_documento_identidad = val.to_string();
    }
}

impl ClienteTipoDocumentoIdentidadSetter for DebitNote {
    fn set_cliente_tipo_documento_identidad(&mut self, val: &str) {
        self.cliente.tipo_documento_identidad = val.to_string();
    }
}
