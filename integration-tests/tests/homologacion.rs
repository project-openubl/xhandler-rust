#[cfg(test)]
mod homologacion {
    use rust_decimal_macros::dec;
    use xbuilder::prelude::{Catalog, Catalog6, Cliente, Detalle, Invoice, Proveedor};

    pub fn proveedor() -> Proveedor {
        Proveedor {
            ruc: "12345678912",
            razon_social: "Softgreen S.A.C.",
            ..Default::default()
        }
    }

    pub fn cliente() -> Cliente {
        Cliente {
            nombre: "Carlos Feria",
            numero_documento_identidad: "12121212121",
            tipo_documento_identidad: Catalog6::RUC.code(),
            ..Default::default()
        }
    }

    #[test]
    fn factura_1_con_3_items() {
        let mut invoice = Invoice {
            serie_numero: "FF11-1",
            proveedor: proveedor(),
            cliente: cliente(),
            detalles: vec![
                Detalle {
                    descripcion: "Item1",
                    cantidad: dec!(1),
                    precio: Some(dec!(100)),
                    ..Default::default()
                },
                Detalle {
                    descripcion: "Item2",
                    cantidad: dec!(2),
                    precio: Some(dec!(200)),
                    ..Default::default()
                },
                Detalle {
                    descripcion: "Item3",
                    cantidad: dec!(3),
                    precio: Some(dec!(300)),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        // assert_invoice(&mut invoice, &format!("{BASE}/customUnidadMedida.xml"));
    }
}
