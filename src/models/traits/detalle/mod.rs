use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::general::Detalle;
use crate::models::invoice::Invoice;

pub mod cantidad;
pub mod icb;
pub mod icbaplica;
pub mod icbtasa;
pub mod igv;
pub mod igvbaseimponible;
pub mod igvtasa;
pub mod igvtipo;
pub mod isc;
pub mod iscbaseimponible;
pub mod isctasa;
pub mod isctipo;
pub mod precio;
pub mod precioconimpuestos;
pub mod precioreferencia;
pub mod precioreferenciatipo;
pub mod totalimpuestos;
pub mod unidadmedida;

pub trait DetallesGetter {
    fn get_detalles(&mut self) -> &mut Vec<Detalle>;
}

impl DetallesGetter for Invoice {
    fn get_detalles(&mut self) -> &mut Vec<Detalle> {
        &mut self.detalles
    }
}

impl DetallesGetter for CreditNote {
    fn get_detalles(&mut self) -> &mut Vec<Detalle> {
        &mut self.detalles
    }
}

impl DetallesGetter for DebitNote {
    fn get_detalles(&mut self) -> &mut Vec<Detalle> {
        &mut self.detalles
    }
}
