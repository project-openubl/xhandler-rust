use std::collections::HashMap;

use chrono::NaiveDate;
use rust_decimal_macros::dec;

use xbuilder::catalogs::{Catalog, Catalog6};
use xbuilder::enricher::enrich::{Defaults, EnrichTrait};
use xbuilder::models::common::{Cliente, Proveedor};
use xbuilder::models::credit_note::CreditNote;
use xbuilder::models::invoice::Invoice;
use xbuilder::renderer::render_invoice;

fn main() {
    let defaults = Defaults {
        date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        icb_tasa: dec!(0.3),
        igv_tasa: dec!(0.18),
        ivap_tasa: dec!(0.04),
    };

    let mut invoice = Invoice {
        leyendas: HashMap::new(),
        serie_numero: "F001-1",
        icb_tasa: None,
        igv_tasa: None,
        ivap_tasa: None,
        moneda: None,
        fecha_emision: None,
        hora_emision: None,
        fecha_vencimiento: None,
        firmante: None,
        proveedor: Proveedor {
            ruc: "123456789012",
            razon_social: "OpenUBL S.A.C.",
            nombre_comercial: None,
            direccion: None,
            contacto: None,
        },
        cliente: Cliente {
            tipo_documento_identidad: Catalog6::DNI.code(),
            numero_documento_identidad: "1234568",
            nombre: "Carlos Feria",
            direccion: None,
            contacto: None,
        },
        detraccion: None,
        percepcion: None,
        direccion_entrega: None,
        forma_de_pago: None,
        tipo_comprobante: None,
        tipo_operacion: None,
        anticipos: vec![],
        descuentos: vec![],
        detalles: vec![],
        total_impuestos: None,
        total_importe: None,

        guias: vec![],
        documentos_relacionados: vec![],
        observaciones: None,
        orden_de_compra: None,
    };

    let mut credit_note = CreditNote {
        leyendas: HashMap::new(),
        serie_numero: "FC01-1",
        icb_tasa: None,
        igv_tasa: None,
        ivap_tasa: None,
        moneda: None,
        fecha_emision: None,
        firmante: None,
        proveedor: Proveedor {
            ruc: "123456789012",
            razon_social: "OpenUBL S.A.C.",
            nombre_comercial: None,
            direccion: None,
            contacto: None,
        },
        cliente: Cliente {
            tipo_documento_identidad: Catalog6::DNI.code(),
            numero_documento_identidad: "1234568",
            nombre: "Carlos Feria",
            direccion: None,
            contacto: None,
        },

        tipo_nota: None,
        comprobante_afectado_serie_numero: "F001-1",
        comprobante_afectado_tipo: None,
        sustento_descripcion: "mis razones",

        detalles: vec![],

        total_impuestos: None,
        total_importe: None,

        guias: vec![],
        documentos_relacionados: vec![],

        orden_de_compra: None,
    };

    invoice.enrich(&defaults);
    credit_note.enrich(&defaults);

    println!("ICB {}", invoice.icb_tasa.unwrap());
    println!("IGV {}", invoice.igv_tasa.unwrap());
    println!("IVAP {}", invoice.ivap_tasa.unwrap());
    println!("Moneda {}", invoice.moneda.unwrap());
    println!("Fecha {}", invoice.fecha_emision.unwrap());
    println!("Proveedor {:?}", invoice.proveedor);
    // println!("Firmante {:?}", invoice.firmante.unwrap());

    let a = &invoice;
    let xml = render_invoice(a).unwrap();
    println!("xml {}", xml);
}
