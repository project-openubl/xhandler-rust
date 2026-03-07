# Plan: Add Missing Fields from Greenter to XBuilder

## Context

Comparing Greenter (PHP) UBL 2.1 templates with our Rust XBuilder reveals gaps in field coverage. Only UBL 2.1 features are in scope (UBL 2.0-only features like embedded dispatch are excluded). Greenter requires all values pre-computed; our system auto-calculates via the 3-phase enrichment pipeline. When adding fields, `Option<T>` fields can be auto-filled if `None`, preserving any user-provided value.

This plan covers **11 work items** across Invoice/CreditNote/DebitNote line items and document-level features. Despatch Advice, Perception/Retention multi-operations, and Summary Documents are deferred to a future plan.

---

## Work Items

### WI-1: Product Codes on Line Items
**Type:** Pure user input — no enrichment rules needed

Add three optional fields to `Detalle`:
- `codigo: Option<&'static str>` — seller's item code
- `codigo_sunat: Option<&'static str>` — SUNAT classification code
- `codigo_gs1: Option<&'static str>` — GS1/GTIN barcode

**Files to modify:**
- `xbuilder/src/models/common.rs` — add fields to `Detalle`
- `xbuilder/resources/templates/ubl/standard/include/document-line.xml` — render inside `<cac:Item>`:
  ```xml
  <cac:SellersItemIdentification><cbc:ID>...</cbc:ID></cac:SellersItemIdentification>
  <cac:StandardItemIdentification><cbc:ID schemeID="9999">...</cbc:ID></cac:StandardItemIdentification>
  <cac:CommodityClassification><cbc:ItemClassificationCode ...>...</cbc:ItemClassificationCode></cac:CommodityClassification>
  ```
- New test + snapshot XML

---

### WI-2: Item Attributes on Line Items
**Type:** Pure user input — no enrichment rules needed

Create new struct `Atributo` and add `atributos: Vec<Atributo>` to `Detalle`.

```rust
pub struct Atributo {
    pub nombre: &'static str,
    pub codigo: &'static str,
    pub valor: Option<&'static str>,
    pub fecha_inicio: Option<NaiveDate>,
    pub fecha_fin: Option<NaiveDate>,
    pub duracion: Option<u32>,
}
```

**Files to modify:**
- `xbuilder/src/models/common.rs` — new struct + field on `Detalle`
- `xbuilder/resources/templates/ubl/standard/include/document-line.xml` — render `<cac:AdditionalItemProperty>` with optional `<cac:UsabilityPeriod>` inside `<cac:Item>`
- New test + snapshot XML

---

### WI-3: Line-Level Discounts
**Type:** Auto-calculable — Phase 1 fill defaults, Phase 2 adjusts `igv_base_imponible`

Add `descuentos: Vec<Descuento>` to `Detalle` (reuses existing `Descuento` struct).

**Auto-calculation logic:**
- Phase 1: if `tipo` is `None` → default to `"00"` (line discount affecting IGV base). If `factor` is `None` → `1`. If `monto_base` is `None` → `monto`.
- Phase 2: subtract line discounts (tipo `"00"`) from `igv_base_imponible`: `base = (cantidad * precio) - sum(descuentos) + isc`

**Files to modify:**
- `xbuilder/src/models/common.rs` — add `descuentos: Vec<Descuento>` to `Detalle`
- New `xbuilder/src/enricher/bounds/detalle/descuentos.rs` — getter trait
- New `xbuilder/src/enricher/rules/phase1fill/detalle/descuentos.rs` — fill defaults
- `xbuilder/src/enricher/rules/phase1fill/detalle/detalles.rs` — register fill rule
- `xbuilder/src/enricher/rules/phase2process/detalle/igv_base_imponible.rs` — adjust formula
- `xbuilder/resources/templates/ubl/standard/include/document-line.xml` — render `<cac:AllowanceCharge>` with `ChargeIndicator=false` before `<cac:TaxTotal>`
- New test + snapshot XML

---

### WI-4: Line-Level Charges + `Cargo` Struct
**Type:** Auto-calculable — same pattern as WI-3

Create new `Cargo` struct (same shape as `Descuento`: tipo, monto, monto_base, factor). Add `cargos: Vec<Cargo>` to `Detalle`.

**Auto-calculation logic:**
- Phase 1: default `tipo` to `"47"` (charge affecting IGV base). Same factor/monto_base defaults.
- Phase 2: add line charges (tipo `"47"`) to `igv_base_imponible`: `base = (cantidad * precio) - descuentos + cargos + isc`

**Files to modify:**
- `xbuilder/src/models/common.rs` — new `Cargo` struct + add `cargos: Vec<Cargo>` to `Detalle`
- New `xbuilder/src/enricher/bounds/detalle/cargos.rs` — getter trait
- New `xbuilder/src/enricher/rules/phase1fill/detalle/cargos.rs` — fill defaults
- `xbuilder/src/enricher/rules/phase1fill/detalle/detalles.rs` — register fill rule
- `xbuilder/src/enricher/rules/phase2process/detalle/igv_base_imponible.rs` — extend formula
- `xbuilder/resources/templates/ubl/standard/include/document-line.xml` — render `<cac:AllowanceCharge>` with `ChargeIndicator=true`
- New test + snapshot XML

---

### WI-5: Document-Level Charges on Invoice
**Type:** Auto-calculable — Phase 1 fill defaults, Phase 3 adjusts totals

Add `cargos: Vec<Cargo>` to `Invoice` (reuses `Cargo` from WI-4).

**Auto-calculation logic:**
- Phase 1: same defaults as WI-4 but with document-level catalog code `"50"`.
- Phase 3: incorporate cargos into `TotalImporteInvoice` computation. Add `cargos: Decimal` field to `TotalImporteInvoice`.

**Files to modify:**
- `xbuilder/src/models/invoice.rs` — add `cargos: Vec<Cargo>`
- `xbuilder/src/models/common.rs` — add `cargos: Decimal` to `TotalImporteInvoice`
- New `xbuilder/src/enricher/bounds/invoice/cargos.rs` — getter/setter traits
- New `xbuilder/src/enricher/rules/phase1fill/invoice/cargos.rs` — fill defaults
- `xbuilder/src/enricher/fill.rs` — register in `FillInvoice`
- `xbuilder/src/enricher/rules/phase3summary/invoice/total_importe.rs` — incorporate cargos in total calculation
- `xbuilder/resources/templates/renderer/invoice.xml` — render doc-level `<cac:AllowanceCharge>` with `ChargeIndicator=true` + `<cbc:ChargeTotalAmount>` in `<cac:LegalMonetaryTotal>`
- New test + snapshot XML

---

### WI-6: Rounding (`redondeo`)
**Type:** User input — passed through to totals, adjusts `PayableAmount`

Add `redondeo: Option<Decimal>` to `Invoice`. Add `redondeo: Option<Decimal>` to `TotalImporteInvoice`.

**Auto-calculation logic:**
- Phase 3: if user provides `redondeo`, store it in `TotalImporteInvoice` and adjust `importe = computed_total + redondeo`.

**Files to modify:**
- `xbuilder/src/models/invoice.rs` — add `redondeo: Option<Decimal>`
- `xbuilder/src/models/common.rs` — add `redondeo: Option<Decimal>` to `TotalImporteInvoice`
- New `xbuilder/src/enricher/bounds/invoice/redondeo.rs` — getter trait
- `xbuilder/src/enricher/rules/phase3summary/invoice/total_importe.rs` — pass through redondeo, adjust importe
- `xbuilder/resources/templates/renderer/invoice.xml` — render `<cbc:PayableRoundingAmount>` before `<cbc:PayableAmount>` in `<cac:LegalMonetaryTotal>`
- New test + snapshot XML

---

### WI-7: Other Taxes (`otroTributo`) at Line + Document Level
**Type:** Auto-calculable — Phase 2 computes, Phase 3 aggregates

Add to `Detalle`: `otro_tributo_tasa`, `otro_tributo_base_imponible`, `otro_tributo` (all `Option<Decimal>`).
Add to `TotalImpuestos`: `otros_importe`, `otros_base_imponible` (both `Decimal`).

**Auto-calculation logic:**
- Phase 2: `otro_tributo = otro_tributo_base_imponible * otro_tributo_tasa` (if `None`). Include in `total_impuestos` sum.
- Phase 3: aggregate across detalles into `TotalImpuestos.otros_*`.

**Files to modify:**
- `xbuilder/src/models/common.rs` — 3 fields on `Detalle`, 2 fields on `TotalImpuestos`
- New `xbuilder/src/enricher/bounds/detalle/otro_tributo.rs` — getter/setter traits
- New `xbuilder/src/enricher/rules/phase2process/detalle/otro_tributo.rs` — compute rule
- `xbuilder/src/enricher/rules/phase2process/detalle/detalles.rs` — register
- `xbuilder/src/enricher/rules/phase2process/detalle/total_impuestos.rs` — include in sum
- `xbuilder/src/enricher/rules/phase3summary/invoice/total_impuestos.rs` — aggregate
- `xbuilder/src/enricher/rules/phase3summary/note/total_impuestos.rs` — aggregate
- `xbuilder/resources/templates/ubl/standard/include/document-line.xml` — TaxSubtotal with code 9999/OTROS/OTH
- `xbuilder/resources/templates/ubl/standard/include/tax-total.xml` — doc-level otros subtotal
- New test + snapshot XML

---

### WI-8: Payment Terms on Credit/Debit Notes
**Type:** Pure user input — reuses existing `FormaDePago` struct and `payment-terms.xml`

Add `forma_de_pago: Option<FormaDePago>` to `CreditNote` and `DebitNote`.

**Files to modify:**
- `xbuilder/src/models/credit_note.rs` — add field
- `xbuilder/src/models/debit_note.rs` — add field
- `xbuilder/resources/templates/renderer/creditNote.xml` — include `payment-terms.xml`
- `xbuilder/resources/templates/renderer/debitNote.xml` — include `payment-terms.xml`
- New tests + snapshot XMLs

---

### WI-9: Installment Currency
**Type:** Pure user input — template-level fallback to document currency

Add `moneda: Option<&'static str>` to `CuotaDePago`.

**Files to modify:**
- `xbuilder/src/models/common.rs` — add field to `CuotaDePago`
- `xbuilder/resources/templates/ubl/standard/include/payment-terms.xml` — use `it.moneda` if present, else `moneda`
- New test + snapshot XML

---

### WI-10: Perception on Credit/Debit Notes
**Type:** Auto-calculable — reuses `Percepcion` struct, needs new summary rule for notes

Add `percepcion: Option<Percepcion>` to `CreditNote` and `DebitNote`.

**Auto-calculation logic:**
- Phase 3: same as invoice (`porcentaje`, `monto`, `monto_base`, `monto_total` auto-computed from each other) but adapted for `TotalImporteNote`.

**Files to modify:**
- `xbuilder/src/models/credit_note.rs` — add field
- `xbuilder/src/models/debit_note.rs` — add field
- New `xbuilder/src/enricher/bounds/note/percepcion.rs` — getter traits for notes
- New `xbuilder/src/enricher/rules/phase3summary/note/percepcion.rs` — summary rule adapted for `TotalImporteNote`
- `xbuilder/src/enricher/summary.rs` — register in `SummaryCreditNote`/`SummaryDebitNote`
- `xbuilder/resources/templates/renderer/creditNote.xml` — PaymentTerms + AllowanceCharge blocks
- `xbuilder/resources/templates/renderer/debitNote.xml` — same
- New tests + snapshot XMLs

---

### WI-11: Delivery Address on Credit/Debit Notes
**Type:** Pure user input — reuses `Direccion` struct and `address.xml` macro

Add `direccion_entrega: Option<Direccion>` to `CreditNote` and `DebitNote`.

**Files to modify:**
- `xbuilder/src/models/credit_note.rs` — add field
- `xbuilder/src/models/debit_note.rs` — add field
- `xbuilder/resources/templates/renderer/creditNote.xml` — render `<cac:Delivery>` block (same as invoice.xml)
- `xbuilder/resources/templates/renderer/debitNote.xml` — same
- New tests + snapshot XMLs

---

## Implementation Order

```
Independent (do first, any order):
  WI-1  Product Codes          (simple, zero risk)
  WI-2  Item Attributes        (simple, zero risk)
  WI-9  Installment Currency   (trivial)
  WI-11 Delivery Address Notes (simple)
  WI-8  Payment Terms Notes    (simple)

Sequential chain:
  WI-3  Line Discounts         (establishes pattern for Detalle Vec fields)
  WI-4  Line Charges           (depends on WI-3: reuses Cargo, same igv_base_imponible)
  WI-5  Doc-Level Charges      (depends on WI-4: reuses Cargo struct)
  WI-6  Rounding               (depends on WI-5: both modify TotalImporteInvoice)

Independent but complex:
  WI-7  Other Taxes            (multi-phase, can be done anytime)
  WI-10 Perception on Notes    (needs new summary rule)
```

## Backward Compatibility

All new fields use `Option<T>` or `Vec<T>` which default to `None`/`[]` via `#[derive(Default)]`. Existing tests and user code will compile and behave identically — new template blocks only render when fields are populated.

## Risk: `igv_base_imponible` Formula Change (WI-3/WI-4)

The formula `base = cantidad * precio + isc` is modified to `base = cantidad * precio - descuentos + cargos + isc`. When `descuentos` and `cargos` are empty (default), the sum is zero, so the formula reduces to the original. All existing tests must still pass.

## Verification

After each WI:
```bash
cargo build                          # compiles
cargo test                           # all existing tests pass
cargo test -p xbuilder <new_test>    # new test passes
cargo fmt --check                    # formatting
cargo clippy --all-targets --all-features -- -D warnings  # lints
```

## Out of Scope (Deferred to future plans)

- Seller party (`seller`) — marketplace-only, rare
- Despatch Advice modernization (multiple drivers, vehicles, indicators, etc.)
- Perception/Retention multi-operations
- Summary Documents gaps (perception, ISC, export, other taxes)
- All UBL 2.0-only features (embedded dispatch, `sac:AdditionalInformation`, etc.)
