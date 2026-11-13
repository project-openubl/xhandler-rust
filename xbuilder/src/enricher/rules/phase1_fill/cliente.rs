use anyhow::Result;

use crate::catalogs::{code_for_name, Catalog6};
use crate::enricher::bounds::cliente::{
    ClienteTipoDocumentoIdentidadGetter, ClienteTipoDocumentoIdentidadSetter,
};

/// Accepts the document type by name, e.g. "DNI" or "RUC", and stores its code.
pub trait ClienteTipoDocumentoIdentidadFillRule {
    fn fill(&mut self) -> Result<bool>;
}

impl<T> ClienteTipoDocumentoIdentidadFillRule for T
where
    T: ClienteTipoDocumentoIdentidadGetter + ClienteTipoDocumentoIdentidadSetter,
{
    fn fill(&mut self) -> Result<bool> {
        match code_for_name::<Catalog6>(self.cliente_tipo_documento_identidad()) {
            Some(code) => {
                self.set_cliente_tipo_documento_identidad(&code);
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
