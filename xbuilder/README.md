# XBuilder

Create an XML based on UBL and the standards of Peru is as easy as following this example:

```rust
use xbuilder::prelude::*;

fn main() {
    // Defaults to be use while enriching the object
    let defaults = Defaults {
        date: NaiveDate::from_ymd_opt(2019, 12, 24).unwrap(),
        icb_tasa: dec!(0.2),
        igv_tasa: dec!(0.18),
        ivap_tasa: dec!(0.04),
    };

    // Object to be use for creating the XML
    let mut invoice = Invoice {
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
        proveedor: Proveedor {
            ruc: "12345678912",
            razon_social: "OpenUBL S.A.C.",
            nombre_comercial: None,
            direccion: None,
            contacto: None,
        },
        cliente: Cliente {
            tipo_documento_identidad: Catalog6::RUC.code(),
            numero_documento_identidad: "12121212121",
            nombre: "Cliente nombre",
            direccion: None,
            contacto: None,
        },

        percepcion: None,
        detraccion: None,

        anticipos: vec![],
        descuentos: vec![],
        detalles: vec![],

        guias: vec![],
        documentos_relacionados: vec![],

        orden_de_compra: None,
        detalles: vec![
            Detalle {
                descripcion: "Item1",
                cantidad: dec!(10),
                unidad_medida: None,

                precio: Some(dec!(100)),
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
            },
            Detalle {
                descripcion: "Item2",
                cantidad: dec!(10),
                unidad_medida: None,

                precio: Some(dec!(100)),
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
            },
        ],
    };

    // Enrich object
    invoice.enrich(&defaults);

    // Render XML
    let xml = render_invoice(invoice);
}
```
