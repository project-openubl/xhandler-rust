use crate::content::catalogs::catalog::Catalog;

enum Catalog1 {
    Factura,
    Boleta,
    NotaCredito,
    NotaDebito,
}

impl Catalog for Catalog1 {
    fn code(&self) -> &str {
        match &self {
            Self::Factura => "01",
            Self::Boleta => "03",
            Self::NotaCredito => "07",
            Self::NotaDebito => "08",
        }
    }
}

pub fn a() {
    let a = Catalog1::Boleta;
}