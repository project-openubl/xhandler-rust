# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What

Rust libraries for Peru SUNAT electronic invoicing: create UBL XML, sign, send via SOAP.

## Commands

```bash
# System deps (Ubuntu): sudo apt-get install pkg-config libssl-dev libxml2-dev libclang-dev
# Fedora: sudo dnf install pkg-config libxml2-devel xmlsec1-devel xmlsec1-openssl clang-devel
cargo build
cargo test
cargo test -p xbuilder test_name    # single test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

Rust 1.100.0 pinned in `rust-toolchain.toml`. Clippy allows unwrap/expect in tests.

## Crates

- **xbuilder** — XML generation via Tera templates + 3-phase enrichment pipeline
- **xsigner** — RSA-SHA256 XML digital signatures (libxml2/openssl C14N)
- **xsender** — ZIP, SOAP send to SUNAT, CDR response parsing, file analysis
- **xhandler** — umbrella re-export

## XBuilder Enrichment Pipeline

User provides minimal input; pipeline fills defaults and computes values:
1. **Phase 1 Fill** (`rules/phase1_fill/`) — defaults: tax rates, currency, doc types, units
2. **Phase 2 Process** (`rules/phase2_process/`) — compute: tax amounts, prices, line totals
3. **Phase 3 Summary** (`rules/phase3_summary/`) — document totals, detractions, legends

Rules implement traits from `enricher/bounds/`. Output rendered via Tera templates (`xbuilder/resources/templates/`, embedded at build time via `build.rs`).

## Domain Terms

IGV=VAT, ISC=excise tax, ICB=bag tax, CDR=SUNAT receipt, detalle=line item, proveedor=supplier, firmante=signer, leyenda=legend, catálogo=SUNAT code tables.

## Tests

xbuilder integration tests in `xbuilder/tests/`, snapshot-compare against `xbuilder/tests/resources/`. xsender/xsigner use inline `#[cfg(test)]`.

xbuilder test switches: `XBUILDER_SKIP_SUNAT=true` skips sending to SUNAT beta (snapshot + XSD still run); `XBUILDER_SNAPSHOT_UPDATE=true` rewrites snapshots with the rendered XML.

## Internal language

- `Invoice` = `Boleta`|`Factura`
- `CreditNote` = `NotaDeCredito`
- `DebitNote` = `NotaDeDebito`

## Naming conventions

Follows RFC 430 and the Rust API Guidelines (C-CASE, C-CONV, C-GETTER, C-WORD-ORDER).

- **Acronyms** are one CamelCase word: `Igv`, `Isc`, `Icb`, `Ivap`, `Sunat`, `Cdr`, `Xml`, `Ruc`, `Dni`, `Gs1`. Enforced by clippy (`.clippy.toml`).
- **Modules/files**: snake_case with every word separated (`credit_note`, `phase1_fill`); the file stem is the concept it holds. No `utils`/`manager` when a domain name fits.
- **Language**: SUNAT/domain concepts in Spanish (`Detalle`, `Percepcion`, `tipo_comprobante`); UBL document names in English (`Invoice`, `DespatchAdvice`); infrastructure in English (verbs, suffixes like `Getter`/`FillRule`/`Error`/`Client`, connectors, status enums, error and log messages). `Perception`/`Retention` = the standalone documents; `Percepcion` = the charge on an invoice.
- **Domain field order**: Spanish head noun first, without `de`/`del` (`tipo_comprobante`, `forma_pago`, `descripcion_sustento`). When several fields describe the same sub-entity, the shared prefix wins (`igv_tipo`/`igv_tasa`, `comprobante_afectado_tipo`/`comprobante_afectado_serie_numero`). A Catalog1 document type is `tipo_comprobante`; other catalogs (12, 61) use `tipo_documento`.
- **Enricher traits**: bounds are `<Scope><Field>Getter`/`Setter` with methods `field()`, `field_mut()`, `set_field(val)`; rules are `<Scope><Field><Phase>Rule` (`Fill`/`Process`/`Summary`). Scope is the struct the trait is implemented on (`Detalle`, `Invoice`, `Percepcion`, ...); traits shared by several documents omit it (`MonedaGetter`, `ProveedorGetter`), and whole-struct accessors on a sub-entity omit the field (`AnticipoGetter`, `DescuentoGetter`). Implement an existing shared trait instead of adding a per-document copy.
- **Errors** are `XxxError`. Conversions use `as_`/`to_`/`into_` and std `FromStr`/`TryFrom`; constructors `new`/`from_*`; booleans `is_*`/`has_*`.
- **Tests**: fns are `<doc>_<scenario>` (no `test_` prefix); snapshots live in `xbuilder/tests/resources/e2e/renderer/<test_file_stem>/<snapshot>.xml`, all snake_case.
- **Templates**: snake_case file names (`renderer/credit_note.xml`, `include/document_line.xml`).
