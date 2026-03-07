# ADR: `openubl` CLI Binary

## Status

Proposed

## Context

The xhandler-rust project provides three Rust library crates for Peru SUNAT electronic invoicing:

- **xbuilder** — Creates UBL XML from Rust structs via a 3-phase enrichment pipeline and Tera templates
- **xsigner** — Signs XML documents with RSA-SHA256 digital signatures (libxml2/openssl C14N)
- **xsender** — Sends signed XML to SUNAT via SOAP/REST, parses CDR responses

Currently, users must write Rust code to use these libraries. There is no standalone CLI tool that allows a user to define a document in a JSON or YAML file, generate UBL XML, sign it, and send it to SUNAT — all from the command line.

This ADR defines a kubectl-inspired CLI called `openubl` that accepts declarative document definitions as input files and orchestrates the full invoicing pipeline.

## Decision

### 1. CLI Binary and Crate

- **Binary name**: `openubl`
- **Crate**: `cli/` at workspace root
- **Framework**: `clap` with derive features (already used in the server crate)

```
cli/
  Cargo.toml            # name = "openubl-cli", [[bin]] name = "openubl"
  src/
    main.rs             # Entry point, clap parsing
    commands/
      mod.rs
      create.rs         # create subcommand
      sign.rs           # sign subcommand
      send.rs           # send subcommand
      apply.rs          # full pipeline subcommand
      verify_ticket.rs  # async ticket verification
    input.rs            # DocumentInput enum, file loading, format detection
    output.rs           # Output formatting (json/yaml/xml/table)
```

Add `"cli"` to workspace members and `serde_yaml` to workspace dependencies in root `Cargo.toml`.

### 2. Subcommands

#### `openubl create`

Generates UBL XML from a JSON/YAML document definition. Runs the xbuilder enrichment pipeline (fill defaults, compute taxes, summarize totals) and renders the XML via Tera templates.

| Flag | Short | Required | Env Var | Description |
|------|-------|----------|---------|-------------|
| `--file <PATH>` | `-f` | Yes | — | Input JSON/YAML file. Use `-` for stdin. |
| `--output <PATH>` | `-o` | No | — | Output XML file path. Writes to stdout if omitted. |
| `--format <FMT>` | — | No | — | Input format when reading from stdin: `json`, `yaml`. Auto-detected from file extension otherwise. |
| `--dry-run` | — | No | — | Validate and enrich input without rendering XML. Prints the enriched document as JSON. |

```bash
# Generate XML to a file
openubl create -f invoice.yaml -o invoice.xml

# Generate XML to stdout
openubl create -f invoice.yaml

# Validate without rendering
openubl create -f invoice.yaml --dry-run

# Read from stdin
cat invoice.json | openubl create -f - --format json -o invoice.xml
```

#### `openubl sign`

Signs a UBL XML document with RSA-SHA256. Reads the XML, applies the digital signature using the provided private key and certificate, and writes the signed XML.

| Flag | Short | Required | Env Var | Description |
|------|-------|----------|---------|-------------|
| `--file <PATH>` | `-f` | Yes | — | Input unsigned XML file. Use `-` for stdin. |
| `--output <PATH>` | `-o` | No | — | Output signed XML file path. Writes to stdout if omitted. |
| `--private-key <PATH>` | — | Yes | `OPENUBL_PRIVATE_KEY` | Path to PKCS#1 PEM private key file. |
| `--certificate <PATH>` | — | Yes | `OPENUBL_CERTIFICATE` | Path to X.509 PEM certificate file. |

```bash
# Sign to a file
openubl sign -f invoice.xml --private-key key.pem --certificate cert.pem -o signed.xml

# Sign to stdout
openubl sign -f invoice.xml --private-key key.pem --certificate cert.pem

# Sign reading from stdin
cat invoice.xml | openubl sign -f - --private-key key.pem --certificate cert.pem -o signed.xml
```

#### `openubl send`

Sends a signed XML to SUNAT. Automatically determines the correct endpoint and protocol (SOAP/REST) based on the document type extracted from the XML metadata (via xsender's analyzer).

| Flag | Short | Required | Env Var | Description |
|------|-------|----------|---------|-------------|
| `--file <PATH>` | `-f` | Yes | — | Signed XML file to send. |
| `--output <PATH>` | `-o` | No | — | Output file for CDR XML response. If omitted, CDR is not saved to disk. |
| `--username <USER>` | — | Yes | `OPENUBL_USERNAME` | SUNAT SOL username (e.g., `20123456789MODDATOS`). |
| `--password <PASS>` | — | Yes | `OPENUBL_PASSWORD` | SUNAT SOL password. |
| `--url-invoice <URL>` | — | No | `OPENUBL_URL_INVOICE` | SUNAT invoice SOAP endpoint. |
| `--url-perception-retention <URL>` | — | No | `OPENUBL_URL_PERCEPTION_RETENTION` | SUNAT perception/retention SOAP endpoint. |
| `--url-despatch <URL>` | — | No | `OPENUBL_URL_DESPATCH` | SUNAT despatch REST endpoint. |
| `--beta` | — | No | — | Use SUNAT beta/test URLs instead of production. |

**Output behavior** depends on the SUNAT response type:

- **CDR (immediate response)** — For Invoice, CreditNote, DebitNote, DespatchAdvice, Perception, Retention:
  - Writes CDR XML to the path specified by `-o` (if provided)
  - Prints CDR metadata to stdout as JSON: `{ "response_code": "0", "description": "La Factura...", "notes": [...] }`

- **Ticket (async response)** — For SummaryDocuments, VoidedDocuments:
  - Prints ticket ID to stdout: `{ "ticket": "1234567890" }`
  - Use `openubl verify-ticket` to check status later

- **Error** — SUNAT rejection:
  - Prints error to stderr: `{ "code": "2109", "message": "El contribuyente no está activo" }`
  - Exits with non-zero code

```bash
# Send and save CDR
openubl send -f signed.xml --username USER --password PASS --beta -o cdr.xml

# Send without saving CDR (just see the metadata)
openubl send -f signed.xml --username USER --password PASS --beta

# Send with explicit URLs
openubl send -f signed.xml --username USER --password PASS \
  --url-invoice "https://e-factura.sunat.gob.pe/ol-ti-itcpfegem/billService"
```

#### `openubl apply`

Full pipeline: create + sign + send in one command. Accepts all flags from `create`, `sign`, and `send`.

| Flag | Short | Required | Env Var | Description |
|------|-------|----------|---------|-------------|
| `--file <PATH>` | `-f` | Yes | — | Input JSON/YAML document definition. |
| `--output <PATH>` | `-o` | No | — | Output file for CDR XML response. |
| `--private-key <PATH>` | — | Yes | `OPENUBL_PRIVATE_KEY` | Path to PKCS#1 PEM private key file. |
| `--certificate <PATH>` | — | Yes | `OPENUBL_CERTIFICATE` | Path to X.509 PEM certificate file. |
| `--username <USER>` | — | Yes | `OPENUBL_USERNAME` | SUNAT SOL username. |
| `--password <PASS>` | — | Yes | `OPENUBL_PASSWORD` | SUNAT SOL password. |
| `--beta` | — | No | — | Use SUNAT beta/test URLs. |
| `--save-xml <PATH>` | — | No | — | Save intermediate unsigned XML to file. |
| `--save-signed-xml <PATH>` | — | No | — | Save intermediate signed XML to file. |
| `--dry-run` | — | No | — | Run create + sign but do not send. Saves signed XML to `--save-signed-xml` or prints to stdout. |

```bash
# Full pipeline
openubl apply -f invoice.yaml \
  --private-key key.pem --certificate cert.pem \
  --username USER --password PASS --beta

# Full pipeline saving all intermediates
openubl apply -f invoice.yaml \
  --private-key key.pem --certificate cert.pem \
  --username USER --password PASS --beta \
  --save-xml unsigned.xml --save-signed-xml signed.xml -o cdr.xml

# Dry run: create + sign without sending
openubl apply -f invoice.yaml \
  --private-key key.pem --certificate cert.pem \
  --dry-run --save-signed-xml signed.xml
```

#### `openubl verify-ticket`

Checks the status of an async submission (SummaryDocuments, VoidedDocuments).

| Flag | Short | Required | Env Var | Description |
|------|-------|----------|---------|-------------|
| `--ticket <ID>` | — | Yes | — | Ticket number from a previous `send` response. |
| `--output <PATH>` | `-o` | No | — | Output file for CDR XML response. |
| `--username <USER>` | — | Yes | `OPENUBL_USERNAME` | SUNAT SOL username. |
| `--password <PASS>` | — | Yes | `OPENUBL_PASSWORD` | SUNAT SOL password. |
| `--url-invoice <URL>` | — | No | `OPENUBL_URL_INVOICE` | SUNAT invoice SOAP endpoint (used for ticket verification). |
| `--beta` | — | No | — | Use SUNAT beta/test URLs. |

```bash
# Verify a ticket
openubl verify-ticket --ticket 1234567890 --username USER --password PASS --beta

# Verify and save the CDR
openubl verify-ticket --ticket 1234567890 --username USER --password PASS --beta -o cdr.xml
```

### 3. Unix Pipe Composability

When `-o` is omitted, each subcommand writes its primary output to stdout. This enables pipeline composition:

```bash
# Full pipeline via unix pipes
openubl create -f invoice.yaml \
  | openubl sign --private-key key.pem --certificate cert.pem \
  | openubl send -f - --username USER --password PASS --beta

# Create and sign, save result
openubl create -f invoice.yaml \
  | openubl sign --private-key key.pem --certificate cert.pem -o signed.xml

# Batch processing
for f in documents/*.yaml; do
  openubl apply -f "$f" \
    --private-key key.pem --certificate cert.pem \
    --username USER --password PASS --beta \
    --save-signed-xml "signed/$(basename "$f" .yaml).xml"
done
```

Diagnostics, progress messages, and warnings are written to stderr so stdout remains clean for piping.

### 4. Environment Variables

All credential and certificate flags support environment variable fallback using clap's `#[arg(env = "...")]`:

```bash
export OPENUBL_PRIVATE_KEY=key.pem
export OPENUBL_CERTIFICATE=cert.pem
export OPENUBL_USERNAME=20123456789MODDATOS
export OPENUBL_PASSWORD=moddatos

# Now these are equivalent:
openubl apply -f invoice.yaml --beta
openubl apply -f invoice.yaml \
  --private-key key.pem --certificate cert.pem \
  --username 20123456789MODDATOS --password moddatos --beta
```

Precedence (highest to lowest):
1. CLI flags
2. Environment variables

### 5. Input File Format

Documents are defined using a kubectl-inspired `kind`/`spec` structure. The `kind` field discriminates the document type, and `spec` contains the document fields matching the xbuilder model structs.

Format is auto-detected from file extension (`.json`, `.yaml`, `.yml`). When reading from stdin, use `--format` to specify.

```rust
#[derive(Deserialize)]
#[serde(tag = "kind")]
enum DocumentInput {
    Invoice { spec: Invoice },
    CreditNote { spec: CreditNote },
    DebitNote { spec: DebitNote },
    DespatchAdvice { spec: DespatchAdvice },
    Perception { spec: Perception },
    Retention { spec: Retention },
    SummaryDocuments { spec: SummaryDocuments },
    VoidedDocuments { spec: VoidedDocuments },
}
```

#### Invoice (Boleta / Factura)

```yaml
kind: Invoice
spec:
  serie_numero: "F001-1"
  moneda: "PEN"
  fecha_emision: "2024-01-15"
  tipo_operacion: "0101"          # Catalog51: venta interna
  proveedor:
    ruc: "20123456789"
    razon_social: "Mi Empresa S.A.C."
    nombre_comercial: "Mi Empresa"
    direccion:
      ubigeo: "150101"
      direccion: "Av. Los Olivos 123"
  cliente:
    tipo_documento_identidad: "6"  # Catalog6: RUC
    numero_documento_identidad: "20987654321"
    nombre: "Cliente Corp S.A."
  detalles:
    - descripcion: "Servicio de consultoría"
      cantidad: 1
      precio: 1000.00
      unidad_medida: "ZZ"          # servicio
    - descripcion: "Laptop HP ProBook"
      cantidad: 2
      precio: 3500.00
      unidad_medida: "NIU"         # unidad
      igv_tipo: "10"               # Catalog7: gravado - operación onerosa
  forma_de_pago:
    tipo: Credito
    total: 8000.00
    cuotas:
      - importe: 4000.00
        fecha_pago: "2024-02-15"
      - importe: 4000.00
        fecha_pago: "2024-03-15"
```

#### CreditNote (Nota de Credito)

```yaml
kind: CreditNote
spec:
  serie_numero: "FC01-1"
  moneda: "PEN"
  fecha_emision: "2024-01-20"
  tipo_nota: "01"                              # Catalog9: anulacion de la operacion
  comprobante_afectado_serie_numero: "F001-1"
  sustento_descripcion: "Anulación de operación por error en datos"
  proveedor:
    ruc: "20123456789"
    razon_social: "Mi Empresa S.A.C."
  cliente:
    tipo_documento_identidad: "6"
    numero_documento_identidad: "20987654321"
    nombre: "Cliente Corp S.A."
  detalles:
    - descripcion: "Servicio de consultoría"
      cantidad: 1
      precio: 1000.00
```

#### DebitNote (Nota de Debito)

```yaml
kind: DebitNote
spec:
  serie_numero: "FD01-1"
  moneda: "PEN"
  fecha_emision: "2024-01-20"
  tipo_nota: "01"                              # Catalog10: interes por mora
  comprobante_afectado_serie_numero: "F001-1"
  sustento_descripcion: "Intereses por mora en pago"
  proveedor:
    ruc: "20123456789"
    razon_social: "Mi Empresa S.A.C."
  cliente:
    tipo_documento_identidad: "6"
    numero_documento_identidad: "20987654321"
    nombre: "Cliente Corp S.A."
  detalles:
    - descripcion: "Intereses por mora"
      cantidad: 1
      precio: 150.00
```

#### DespatchAdvice (Guia de Remision)

```yaml
kind: DespatchAdvice
spec:
  serie_numero: "T001-1"
  fecha_emision: "2024-01-15"
  remitente:
    ruc: "20123456789"
    razon_social: "Mi Empresa S.A.C."
  destinatario:
    tipo_documento_identidad: "1"    # DNI
    numero_documento_identidad: "12345678"
    nombre: "Juan Perez"
  envio:
    tipo_traslado: "01"              # Catalog20: venta
    peso_total: 25.5
    peso_total_unidad_medida: "KGM"
    tipo_modalidad_traslado: "02"    # Catalog18: privado
    fecha_traslado: "2024-01-16"
    partida:
      ubigeo: "150101"
      direccion: "Av. Los Olivos 123, Lima"
    destino:
      ubigeo: "040101"
      direccion: "Jr. Arequipa 456, Arequipa"
    transportista:
      tipo_documento_identidad: "6"
      numero_documento_identidad: "20111222333"
      nombre: "Transportes Rapidos S.A."
      placa_del_vehiculo: "ABC-123"
      chofer_tipo_documento_identidad: "1"
      chofer_numero_documento_identidad: "87654321"
  detalles:
    - unidad_medida: "NIU"
      cantidad: 10
      codigo: "PROD001"
      descripcion: "Laptop HP ProBook"
    - unidad_medida: "KGM"
      cantidad: 15.5
      codigo: "PROD002"
      descripcion: "Cable de red Cat6"
```

#### Perception (Comprobante de Percepcion)

```yaml
kind: Perception
spec:
  serie: "P001"
  numero: 1
  fecha_emision: "2024-01-31"
  proveedor:
    ruc: "20123456789"
    razon_social: "Mi Empresa S.A.C."
  cliente:
    tipo_documento_identidad: "6"
    numero_documento_identidad: "20987654321"
    nombre: "Cliente Corp S.A."
  tipo_regimen: "01"                  # Catalog22: percepciones venta interna
  tipo_regimen_porcentaje: 2
  importe_total_percibido: 10.00
  importe_total_cobrado: 210.00
  operacion:
    numero_operacion: 1
    fecha_operacion: "2024-01-31"
    importe_operacion: 100.00
    comprobante:
      tipo_comprobante: "01"
      serie_numero: "F001-1"
      fecha_emision: "2024-01-15"
      importe_total: 200.00
      moneda: "PEN"
```

#### Retention (Comprobante de Retencion)

```yaml
kind: Retention
spec:
  serie: "R001"
  numero: 1
  fecha_emision: "2024-01-31"
  proveedor:
    ruc: "20123456789"
    razon_social: "Mi Empresa S.A.C."
  cliente:
    tipo_documento_identidad: "6"
    numero_documento_identidad: "20987654321"
    nombre: "Proveedor del Servicio S.A."
  tipo_regimen: "01"                  # Catalog23: tasa 3%
  tipo_regimen_porcentaje: 3
  importe_total_retenido: 30.00
  importe_total_pagado: 970.00
  operacion:
    numero_operacion: 1
    fecha_operacion: "2024-01-31"
    importe_operacion: 1000.00
    comprobante:
      tipo_comprobante: "01"
      serie_numero: "F001-50"
      fecha_emision: "2024-01-10"
      importe_total: 1000.00
      moneda: "PEN"
```

#### SummaryDocuments (Resumen Diario de Boletas)

```yaml
kind: SummaryDocuments
spec:
  numero: 1
  fecha_emision: "2024-01-15"
  fecha_emision_comprobantes: "2024-01-14"
  moneda: "PEN"
  proveedor:
    ruc: "20123456789"
    razon_social: "Mi Empresa S.A.C."
  comprobantes:
    - tipo_operacion: "1"              # Catalog19: adicionar
      comprobante:
        tipo_comprobante: "03"         # Catalog1: boleta
        serie_numero: "B001-1"
        cliente:
          tipo_documento_identidad: "1"
          numero_documento_identidad: "12345678"
        valor_venta:
          importe_total: 118.00
          gravado: 100.00
        impuestos:
          igv: 18.00
          igv_tasa: 18
    - tipo_operacion: "3"              # Catalog19: anulado
      comprobante:
        tipo_comprobante: "03"
        serie_numero: "B001-2"
        cliente:
          tipo_documento_identidad: "1"
          numero_documento_identidad: "87654321"
        valor_venta:
          importe_total: 50.00
          gravado: 42.37
        impuestos:
          igv: 7.63
          igv_tasa: 18
```

#### VoidedDocuments (Comunicacion de Baja)

```yaml
kind: VoidedDocuments
spec:
  numero: 1
  fecha_emision: "2024-01-15"
  fecha_emision_comprobantes: "2024-01-14"
  proveedor:
    ruc: "20123456789"
    razon_social: "Mi Empresa S.A.C."
  comprobantes:
    - serie: "F001"
      numero: 1
      descripcion_sustento: "Error en el documento"
    - serie: "F001"
      numero: 2
      descripcion_sustento: "Documento duplicado"
```

#### JSON Format

The same documents can be expressed in JSON:

```json
{
  "kind": "Invoice",
  "spec": {
    "serie_numero": "F001-1",
    "moneda": "PEN",
    "proveedor": {
      "ruc": "20123456789",
      "razon_social": "Mi Empresa S.A.C."
    },
    "cliente": {
      "tipo_documento_identidad": "6",
      "numero_documento_identidad": "20987654321",
      "nombre": "Cliente Corp S.A."
    },
    "detalles": [
      {
        "descripcion": "Servicio de consultoría",
        "cantidad": 1,
        "precio": 1000.00
      }
    ]
  }
}
```

### 6. Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error (invalid input, file not found, parse error) |
| `2` | SUNAT rejection (CDR response_code != "0") |
| `3` | Network/connection error |
| `4` | Signing error (invalid certificate or key) |

### 7. Combined Usage Examples

```bash
# === Individual commands with file I/O ===

# Step 1: Create XML from YAML definition
openubl create -f factura.yaml -o factura.xml

# Step 2: Sign the XML
openubl sign -f factura.xml \
  --private-key key.pem --certificate cert.pem \
  -o factura-signed.xml

# Step 3: Send to SUNAT beta and save CDR
openubl send -f factura-signed.xml \
  --username 20123456789MODDATOS --password moddatos \
  --beta -o cdr-response.xml


# === Full pipeline in one command ===

openubl apply -f factura.yaml \
  --private-key key.pem --certificate cert.pem \
  --username 20123456789MODDATOS --password moddatos \
  --beta -o cdr-response.xml


# === Full pipeline saving all intermediate files ===

openubl apply -f factura.yaml \
  --private-key key.pem --certificate cert.pem \
  --username 20123456789MODDATOS --password moddatos \
  --beta \
  --save-xml factura.xml \
  --save-signed-xml factura-signed.xml \
  -o cdr-response.xml


# === Unix pipes ===

openubl create -f factura.yaml \
  | openubl sign --private-key key.pem --certificate cert.pem \
  | openubl send -f - --username USER --password PASS --beta


# === Dry run: validate + enrich only ===

openubl create -f factura.yaml --dry-run
# Output: enriched document as JSON (with computed taxes, totals, defaults filled in)


# === Dry run: create + sign without sending ===

openubl apply -f factura.yaml \
  --private-key key.pem --certificate cert.pem \
  --dry-run \
  --save-signed-xml factura-signed.xml


# === Async documents: send + verify ticket ===

# Send a SummaryDocuments (returns a ticket)
openubl send -f resumen-signed.xml \
  --username USER --password PASS --beta
# Output: { "ticket": "1698267890123" }

# Later, verify the ticket
openubl verify-ticket --ticket 1698267890123 \
  --username USER --password PASS --beta -o cdr-response.xml


# === Environment variables for credentials ===

export OPENUBL_PRIVATE_KEY=key.pem
export OPENUBL_CERTIFICATE=cert.pem
export OPENUBL_USERNAME=20123456789MODDATOS
export OPENUBL_PASSWORD=moddatos

openubl apply -f factura.yaml --beta
openubl apply -f nota-credito.yaml --beta
openubl apply -f guia-remision.yaml --beta


# === Batch processing ===

for f in documents/*.yaml; do
  echo "Processing $f..."
  openubl apply -f "$f" \
    --private-key key.pem --certificate cert.pem \
    --username USER --password PASS --beta \
    --save-signed-xml "signed/$(basename "$f" .yaml).xml" \
    -o "cdr/$(basename "$f" .yaml)-cdr.xml"
done
```

## Implementation Phases

1. **Phase 1: CLI scaffold + `create`** — Set up `cli/` crate, clap command structure, `DocumentInput` enum with `kind`/`spec` deserialization, `create` subcommand calling xbuilder's enrichment pipeline and renderer.

2. **Phase 2: `sign` + `send`** — Add `sign` subcommand wrapping xsigner's `RsaKeyPair` and `XSigner`, and `send` subcommand wrapping xsender's `FileSender`. Handle CDR/Ticket/Error response variants.

3. **Phase 3: `apply` pipeline** — Combine create + sign + send into a single command with `--save-xml`, `--save-signed-xml`, and `--dry-run` support.

4. **Phase 4: `verify-ticket` + polish** — Add ticket verification, example YAML files in `cli/examples/`, and documentation.

## Consequences

### Positive

- Provides a low-barrier entry point for users who do not need the full server — accountants, developers, CI/CD scripts can process invoices with a single command.
- kubectl-inspired `kind`/`spec` format is familiar and well-understood for declarative configuration.
- Unix pipe composability enables integration into shell scripts and automation workflows.
- Environment variable support simplifies credential management in CI/CD environments.
- Separated subcommands allow users to integrate at any point in the pipeline (e.g., only sign XML generated by another system).

### Negative

- The xbuilder model structs currently use `&'static str` and only derive `Serialize`. Deserialization from JSON/YAML requires owned string types and `Deserialize` support. This is a prerequisite that must be resolved before the CLI can be implemented.
- Adding a binary crate increases build times and binary size due to additional dependencies (`serde_yaml`, `clap`, `tokio` runtime).
- The CLI must stay in sync with model changes in xbuilder — adding fields to model structs requires updating example files and documentation.
