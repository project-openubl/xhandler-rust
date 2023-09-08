use chrono::NaiveDate;
use rust::content::models::common::{ProveedorBuilder};
use rust::content::models::invoice::InvoiceBuilder;
use rust::enricher::enricher::{Defaults, enrich};

fn main() {
    println!("Hello, world!");
    let invoice = InvoiceBuilder::default()
        .proveedor(ProveedorBuilder::default()
            .ruc("123456789012".to_string())
            .razon_social("OpenUBL".to_string())
            .build()
            .unwrap()
        )
        .build();
    match invoice {
        Ok(mut invoice) => {
            let defaults = Defaults {
                date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                icb_tasa: 1,
                igv_tasa: 1,
                ivap_tasa: 1,
            };
            enrich(&mut invoice, defaults);
            println!("Moneda {}", invoice.moneda.unwrap());
            println!("Fecha {}", invoice.fecha_emision.unwrap());
        }
        Err(error) => {
            println!("Error {}", error)
        }
    };
}
