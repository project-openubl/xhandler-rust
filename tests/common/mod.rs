use std::collections::HashMap;
use std::fs;

use chrono::NaiveDate;

use xbuilder::prelude::*;

pub fn defaults_base() -> Defaults {
    Defaults {
        date: NaiveDate::from_ymd_opt(2019, 12, 24).unwrap(),
        icb_tasa: 0.2f64,
        igv_tasa: 0.18f64,
        ivap_tasa: 0.04f64,
    }
}

pub fn invoice_base() -> Invoice {
    Invoice {
        leyendas: HashMap::new(),

        serie_numero: "F001-1",
        tipo_comprobante: None,
        tipo_operacion: None,

        igv_tasa: None,
        icb_tasa: None,
        ivap_tasa: None,

        moneda: None,
        fecha_emision: None,
        hora_emision: None,
        fecha_vencimiento: None,
        forma_de_pago: None,
        direccion_entrega: None,
        observaciones: None,

        total_impuestos: None,
        total_importe: None,

        firmante: None,
        proveedor: proveedor_base(),
        cliente: cliente_base(),

        percepcion: None,
        detraccion: None,

        anticipos: vec![],
        descuentos: vec![],
        detalles: vec![
            Detalle {
                precio: Some(100f64),
                ..detalle_base("Item1", 10f64)
            },
            Detalle {
                precio: Some(100f64),
                ..detalle_base("Item2", 10f64)
            },
        ],

        guias: vec![],
        documentos_relacionados: vec![],

        orden_de_compra: None,
    }
}

pub fn proveedor_base() -> Proveedor {
    Proveedor {
        ruc: "12345678912",
        razon_social: "Softgreen S.A.C.",
        nombre_comercial: None,
        direccion: None,
        contacto: None,
    }
}

pub fn cliente_base() -> Cliente {
    Cliente {
        tipo_documento_identidad: Catalog6::RUC.code(),
        numero_documento_identidad: "12121212121",
        nombre: "Carlos Feria",
        direccion: None,
        contacto: None,
    }
}

pub fn detalle_base(descripcion: &'static str, cantidad: f64) -> Detalle {
    Detalle {
        descripcion,
        cantidad,
        unidad_medida: None,

        precio: None,
        precio_con_impuestos: None,
        precio_referencia: None,
        precio_referencia_tipo: None,

        igv_tasa: None,
        icb_tasa: None,
        isc_tasa: None,

        igv_tipo: None,
        isc_tipo: None,

        icb_aplica: false,
        icb: None,

        igv: None,
        igv_base_imponible: None,

        isc: None,
        isc_base_imponible: None,

        total_impuestos: None,
    }
}

pub fn assert_invoice(invoice: &mut Invoice, snapshot_filename: &str) {
    let defaults = defaults_base();
    invoice.enrich(&defaults);

    let result = render_invoice(invoice);
    assert!(result.is_ok());

    assert_snapshot(result.ok().unwrap(), snapshot_filename)
}

fn assert_snapshot(expected: String, snapshot_filename: &str) {
    let snapshot_file_content = fs::read_to_string(snapshot_filename);
    assert!(snapshot_file_content.is_ok());

    assert_eq!(
        expected,
        snapshot_file_content.unwrap(),
        "File {} does not match",
        snapshot_filename
    );
}
