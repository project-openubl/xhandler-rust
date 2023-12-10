use crate::models::common::Detalle;
use crate::models::credit_note::CreditNote;
use crate::models::debit_note::DebitNote;
use crate::models::invoice::Invoice;

pub mod cantidad;
pub mod icb;
pub mod icb_aplica;
pub mod icb_tasa;
pub mod igv;
pub mod igv_base_imponible;
pub mod igv_tasa;
pub mod igv_tipo;
pub mod isc;
pub mod isc_base_imponible;
pub mod isc_tasa;
pub mod isc_tipo;
pub mod precio;
pub mod precio_con_impuestos;
pub mod precio_referencia;
pub mod precio_referencia_tipo;
pub mod total_impuestos;
pub mod unidad_medida;

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
