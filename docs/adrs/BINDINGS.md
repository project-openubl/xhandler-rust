# ADR: The Go, Python and Java bindings follow the Rust models

## Status

Accepted

## Context

The Go, Python and Java libraries don't reimplement xhandler. They run the
Rust core compiled to WebAssembly (`xhandler-wasm`) and exchange JSON with it.
Each one syncs a *released* core with `make sync CORE=<version>`, which copies
the following out of `xhandler-dist-<version>.tar.gz`:

- the wasm and its VERSION
- the generated model types
- the conformance cases

That covered the documents only. The answers of the SUNAT services were built
in `sender.rs` with untyped `json!{}` and re-typed by hand in each binding. So
when #612 added `document_description` to the CDR, none of the three bindings
followed:

- **Python** would fail: its models reject unknown fields, and the core
  always sends `"document_description": null`.
- **Go** dropped the field silently.
- **Java** dropped it silently too.
- **The conformance cases** stayed green, because they compare raw JSON, not
  the types users get.

## Decision

Every model the bindings exchange with the core is a Rust type. Rust derives
the JSON Schema from those types, and the bindings' types are generated from
that schema. A field is written down once, in Rust.

### Flow of a model change

1. **A PR in xhandler-rust adds a field** to a document (xbuilder) or to an
   answer (`xhandler-wasm/src/answer.rs`). The bindings don't change yet:
   they still run the previous release, whose models match it.
2. **The release** (release-tools `release-xhandler`) builds `dist/`. It
   contains the schema and the types regenerated from it, in
   `dist/models/<lang>/` (documents) and `dist/models/<lang>/xsender/`
   (answers).
3. **release-tools opens a sync PR in each binding.** `make sync` copies the
   new wasm, the regenerated types and the conformance cases. The new field
   is in the diff, and the binding's CI checks it.
4. **CI passes, the PRs merge, and the bindings are tagged and published**,
   with the field and without anyone editing them.

There is no step between releases: a binding always pairs a released core
with the types generated from that same core.

### What is generated

`codegen.py <schema> <package> <lang> <out>` generates two packages:

- **xbuilder**: the documents and their defaults, which the bindings send.
  - Strict: a misspelt field fails.
- **xsender**: the answers `parse` gives back (`SendFileAnswer`,
  `VerifyResult`, `ConsultResult`, and their `CDR`, `Fault`, `NoCDR`,
  `ConsultStatus`, `Status`).
  - Lenient: Python `extra="ignore"`, Java
    `@JsonIgnoreProperties(ignoreUnknown = true)`, Go by default.
  - So a binding never breaks on a field a newer core adds.

The root of the schema (`examples/schema.rs`) lists what each package
generates. A root property that no package claims, or a type reachable from
both, stops codegen. Unsupported schema constructs keep failing loudly.

`sender.rs` builds the answers from these types, so it can't emit a field the
schema lacks. The conformance cases are recorded from what `invoke` returns.
When the answers became typed, the cases stayed byte for byte the same.

### What stays hand-written in the bindings

- **The sender inputs.** These are URLs, credentials, targets and consult
  criteria. They carry binding behaviour (secret redaction, default URLs,
  helpers), and a new input is an API choice rather than plain data.
- **The tokens of the guías**, used only inside the bindings.
- **Composition around the generated types.** For example, `SendResult` is
  `SendFileAnswer` plus the targets the file went to.
- **Java's public `SunatResponse` / `Metadata` / `Sunat` / `Status`.** That
  API is inherited from the old Java xsender, so it stays and is mapped from
  the generated types. Its Spanish `Status` constants differ from the
  generated `ACCEPTED` / `IN_PROGRESS`. When Java switches, `SunatResponse`
  will also expose the generated `CDR` (`getCdr()`), so later CDR fields
  reach Java users with no hand edit. `Sunat.getDocumentDescription()` stays
  as the legacy API's way to read that field.

### Compatibility

| Change in Rust | Bindings |
|---|---|
| New answer field, new optional document field | Arrives with the next release's sync PRs; no binding work |
| New value of an answer enum (e.g. `Status`) | Breaking: Python and Java reject values they don't know (only Go accepts them) |
| Removed or renamed field, changed type, new required field | Breaking: the sync PR's CI fails until the binding adapts, and the release notes call it out |
| New answer type or op | Needs a binding API, written by hand |

## Rollout

Bindings only sync released cores, so the change lands in this order:

1. **Before 6.0.0-alpha.2:** each binding reads `document_description`, and
   Python's answers ignore unknown fields. Without this, alpha.2's Python
   sync PR would fail.
2. **This change in xhandler-rust:** typed answers, the schema, and the
   xsender package in codegen.
3. **Release 6.0.0-alpha.2.**
4. **Each binding** copies `dist/models/<lang>/xsender/` in `make sync` and
   replaces its hand-written answers with the generated ones. The fields and
   names users read stay the same, with these differences:
   - `SendResult` becomes `SendFileAnswer` plus the targets (see above).
   - In Go, the values that can be absent become pointers: `CDR.StatusCode`
     and `Fault.HTTPStatus`.
   - The Go JSON tags gain `omitempty`, so re-encoding an answer leaves out
     its absent members instead of writing `null`.
   - Java keeps its public API and only maps from the generated types.

## Alternatives considered

- **Generate whole clients.** The bindings would read like no idiomatic Go,
  Python or Java library. Only data is generated.
- **Keep hand-written answers and decode the conformance cases through
  them.** Drift would be caught, but every field would still take three
  hand edits.
- **Protobuf or another IDL.** It would add a second source of truth next to
  the Rust types, and a toolchain to each binding.
- **Sync the bindings on every merge to xhandler-rust main.** The bindings
  would run unreleased cores, which their `verify-core` / `verify-release`
  checks rightly reject.
