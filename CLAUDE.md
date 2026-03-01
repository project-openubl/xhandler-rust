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

Rust 1.83.0 pinned in `rust-toolchain.toml`. Clippy allows unwrap/expect in tests.

## Crates

- **xbuilder** — XML generation via Tera templates + 3-phase enrichment pipeline
- **xsigner** — RSA-SHA256 XML digital signatures (libxml2/openssl C14N)
- **xsender** — ZIP, SOAP send to SUNAT, CDR response parsing, file analysis
- **xhandler** — umbrella re-export

## XBuilder Enrichment Pipeline

User provides minimal input; pipeline fills defaults and computes values:
1. **Phase 1 Fill** (`rules/phase1fill/`) — defaults: tax rates, currency, doc types, units
2. **Phase 2 Process** (`rules/phase2process/`) — compute: tax amounts, prices, line totals
3. **Phase 3 Summary** (`rules/phase3summary/`) — document totals, detractions, legends

Rules implement traits from `enricher/bounds/`. Output rendered via Tera templates (`xbuilder/resources/templates/`, embedded at build time via `build.rs`).

## Domain Terms

IGV=VAT, ISC=excise tax, ICB=bag tax, CDR=SUNAT receipt, detalle=line item, proveedor=supplier, firmante=signer, leyenda=legend, catálogo=SUNAT code tables.

## Tests

xbuilder integration tests in `xbuilder/tests/`, snapshot-compare against `xbuilder/tests/resources/`. xsender/xsigner use inline `#[cfg(test)]`.

## Internal language

- `Invoice` = `Boleta`|`Factura`
- `CreditNote` = `NotaDeCredito`
- `DebitNote` = `NotaDeCredito`
