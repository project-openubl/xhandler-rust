# xhandler-rust

[![License](https://img.shields.io/github/license/project-openubl/xhandler-rust?logo=apache)](https://www.apache.org/licenses/LICENSE-2.0)
[![CI](https://github.com/project-openubl/xhandler-rust/actions/workflows/ci-repo.yml/badge.svg)](https://github.com/project-openubl/xhandler-rust/actions/workflows/ci-repo.yml)
[![Project Chat](https://img.shields.io/badge/zulip-join_chat-brightgreen.svg?logo=zulip)](https://projectopenubl.zulipchat.com/)

Crea, firma y envía los comprobantes electrónicos (XML UBL) de la SUNAT del Perú: facturas,
boletas, notas de crédito y débito, guías de remisión, percepciones, retenciones, resúmenes
diarios, comunicaciones de baja y reversiones.

Se usa como librería de Rust (`xhandler`) o con la herramienta de línea de comandos `openubl`. El mismo núcleo
es la base de las librerías de [Go](https://github.com/project-openubl/xhandler-go),
[Python](https://github.com/project-openubl/xhandler-python) y
[Java](https://github.com/project-openubl/xhandler-java).

| Módulo de `xhandler` | Qué hace |
|---|---|
| `xbuilder` | Crea el XML a partir de los datos esenciales: completa los valores por defecto y calcula impuestos y totales |
| `xsigner` | Firma el XML (RSA-SHA256, certificados PEM o PKCS#12) |
| `xsender` | Envía a la SUNAT (servicios SOAP y API REST de guías) y lee el CDR, tickets y errores |

## Línea de comandos: `openubl`

### Instalación

```shell
cargo install --locked --git https://github.com/project-openubl/xhandler-rust xhandler-cli
```

Necesita [Rust](https://rustup.rs) y un compilador de C.

### Uso

Describe el comprobante en YAML o JSON:

```yaml
# factura.yaml
kind: Invoice
spec:
  serie_numero: "F001-1"
  proveedor:
    ruc: "12345678912"
    razon_social: "Softgreen S.A.C."
  cliente:
    tipo_documento_identidad: "6"
    numero_documento_identidad: "12121212121"
    nombre: "Carlos Feria"
  detalles:
    - descripcion: "Item1"
      cantidad: 10
      precio: 100
```

Los otros tipos (`kind`) son `CreditNote`, `DebitNote`, `DespatchAdvice`, `Perception`,
`Retention`, `SummaryDocuments`, `VoidedDocuments` y `Reversion` (ver [`cli/examples`](cli/examples)).

```shell
# Todo en un paso: crear, firmar y enviar. --beta usa el entorno de pruebas de la SUNAT,
# con credenciales y certificado de prueba.
openubl apply -f factura.yaml --beta

# O paso a paso
openubl create -f factura.yaml -o factura.xml
openubl sign -f factura.xml --pfx certificado.pfx --pfx-password clave -o firmado.xml
openubl send -f firmado.xml --username 20123456789USUARIO --password clave

# Los resúmenes y las bajas devuelven un ticket: consúltalo después
openubl verify-ticket --ticket 1703154974517 --beta
```

- La salida es JSON. El código de salida es 0 si la SUNAT acepta el comprobante (o devuelve un
  ticket) y 2 si lo rechaza.
- Las credenciales y certificados pueden venir de variables de entorno, p. ej.
  `OPENUBL_USERNAME`, `OPENUBL_PASSWORD`, `OPENUBL_PFX` y `OPENUBL_PFX_PASSWORD`.
- Las guías de remisión también necesitan `--client-id` y `--client-secret`. La SUNAT no tiene beta
  para ellas.

`openubl <comando> --help` muestra todas las opciones.

## Librería

### Instalación

```toml
[dependencies]
xhandler = { git = "https://github.com/project-openubl/xhandler-rust" }
```

La feature `http` (activa por defecto) envía con reqwest. Sin ella (`default-features = false`),
`xhandler::xsender::exchange` arma las peticiones a la SUNAT y lee sus respuestas para que las
envíes con tu propio cliente HTTP.

El ejemplo también usa `rust_decimal`, `rust_decimal_macros`, `chrono`, `tokio` y `anyhow`.

### Uso

```rust
use rust_decimal_macros::dec;
use xhandler::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Crear: solo los datos esenciales, xbuilder calcula el resto
    let mut invoice = Invoice {
        serie_numero: "F001-1".into(),
        proveedor: Proveedor {
            ruc: "12345678912".into(),
            razon_social: "Softgreen S.A.C.".into(),
            ..Default::default()
        },
        cliente: Cliente {
            tipo_documento_identidad: "6".into(), // RUC (Catalog6)
            numero_documento_identidad: "12121212121".into(),
            nombre: "Carlos Feria".into(),
            ..Default::default()
        },
        detalles: vec![Detalle {
            descripcion: "Item".into(),
            cantidad: dec!(2),
            precio: Some(dec!(100)),
            ..Default::default()
        }],
        ..Default::default()
    };
    invoice.enrich(&Defaults::new(chrono::Local::now().date_naive()))?;
    let xml = invoice.render()?;

    // Firmar
    let key = RsaKeyPair::from_pkcs12(&std::fs::read("certificado.pfx")?, "password")?;
    let signed = XSigner { xml_document: xml }.sign(&key)?;

    // Enviar
    let sender = FileSender {
        urls: Urls::beta(),
        credentials: Credentials {
            username: "12345678959MODDATOS".into(),
            password: "MODDATOS".into(),
            ..Default::default()
        },
    };
    match sender.send_file(&UblFile::from_bytes(signed)?).await?.response {
        SendFileAggregatedResponse::Cdr(_, cdr) => println!("{} {}", cdr.status(), cdr.description),
        SendFileAggregatedResponse::Ticket(ticket) => println!("consultar el ticket {ticket} después"),
        SendFileAggregatedResponse::Fault(fault) => println!("{} {}", fault.code, fault.message),
    }
    Ok(())
}
```

## Contribuir

Revisa [CONTRIBUTING.md](CONTRIBUTING.md). Preguntas e ideas en el
[chat de Zulip](https://projectopenubl.zulipchat.com/). El servidor web y la interfaz están en
[xhandler-ui](https://github.com/project-openubl/xhandler-ui).

## Licencia

[Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
