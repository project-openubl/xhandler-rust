use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub trait ClienteTipoDocumentoIdentidadGetter {
    fn get_cliente_tipo_documento_identidad(&self) -> &str;
}

pub trait ClienteTipoDocumentoIdentidadSetter {
    fn set_cliente_tipo_documento_identidad(&mut self, val: &str);
}

macro_rules! impl_cliente_bounds {
    ($($document:ty),*) => {
        $(
            impl ClienteTipoDocumentoIdentidadGetter for $document {
                fn get_cliente_tipo_documento_identidad(&self) -> &str {
                    &self.cliente.tipo_documento_identidad
                }
            }

            impl ClienteTipoDocumentoIdentidadSetter for $document {
                fn set_cliente_tipo_documento_identidad(&mut self, val: &str) {
                    self.cliente.tipo_documento_identidad = val.to_string();
                }
            }
        )*
    };
}

impl_cliente_bounds!(Invoice, CreditNote, DebitNote);
