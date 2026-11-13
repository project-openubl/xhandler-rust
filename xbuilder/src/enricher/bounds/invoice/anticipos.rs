use crate::models::common::Anticipo;
use crate::models::invoice::Invoice;

pub trait InvoiceAnticiposGetter {
    fn anticipos(&self) -> &[Anticipo];
    fn anticipos_mut(&mut self) -> &mut Vec<Anticipo>;
}

impl InvoiceAnticiposGetter for Invoice {
    fn anticipos(&self) -> &[Anticipo] {
        &self.anticipos
    }

    fn anticipos_mut(&mut self) -> &mut Vec<Anticipo> {
        &mut self.anticipos
    }
}

//

pub trait AnticipoGetter {
    fn tipo(&self) -> &Option<String>;
    fn comprobante_tipo(&self) -> &Option<String>;
    fn comprobante_serie_numero(&self) -> &str;
}

pub trait AnticipoSetter {
    fn set_tipo(&mut self, val: &str);
    fn set_comprobante_tipo(&mut self, val: &str);
}

impl AnticipoGetter for Anticipo {
    fn tipo(&self) -> &Option<String> {
        &self.tipo
    }

    fn comprobante_tipo(&self) -> &Option<String> {
        &self.comprobante_tipo
    }

    fn comprobante_serie_numero(&self) -> &str {
        &self.comprobante_serie_numero
    }
}

impl AnticipoSetter for Anticipo {
    fn set_tipo(&mut self, val: &str) {
        self.tipo = Some(val.to_string());
    }

    fn set_comprobante_tipo(&mut self, val: &str) {
        self.comprobante_tipo = Some(val.to_string());
    }
}
