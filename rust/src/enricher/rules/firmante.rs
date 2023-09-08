use crate::content::models::common::Firmante;
use crate::content::models::invoice::Invoice;
use crate::enricher::rules::proveedor::ProveedorGetterSetter;

pub trait FirmanteRule {
    fn enrich_firmante(&mut self) -> bool;
}

pub trait FirmanteGetterSetter {
    fn get_firmante(&self) -> &Option<Firmante>;
    fn set_firmante(&mut self, val: Option<Firmante>);
}

impl<T> FirmanteRule for T
    where T: FirmanteGetterSetter + ProveedorGetterSetter, {
    fn enrich_firmante(&mut self) -> bool {
        match &self.get_firmante() {
            Some(..) => false,
            None => {
                self.set_firmante(Some(Firmante {
                    ruc: self.get_proveedor().ruc.to_string(),
                    razon_social: self.get_proveedor().razon_social.to_string(),
                }));
                true
            }
        }
    }
}

impl FirmanteGetterSetter for Invoice {
    fn get_firmante(&self) -> &Option<Firmante> {
        &self.firmante
    }

    fn set_firmante(&mut self, val: Option<Firmante>) {
        self.firmante = val;
    }
}

