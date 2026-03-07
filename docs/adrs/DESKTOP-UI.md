# ADR: `openubl` Desktop UI Application

## Status

Proposed

## Context

The xhandler-rust project provides three Rust library crates for Peru SUNAT electronic invoicing:

- **xbuilder** — Creates UBL XML from Rust structs via a 3-phase enrichment pipeline and Tera templates
- **xsigner** — Signs XML documents with RSA-SHA256 digital signatures (libxml2/openssl C14N)
- **xsender** — Sends signed XML to SUNAT via SOAP/REST, parses CDR responses

The `openubl` CLI (see `docs/adrs/CLI.md`) provides a command-line interface for these libraries, accepting JSON/YAML document definitions and orchestrating the full invoicing pipeline. However, the CLI targets developers and automation workflows. Users who are less comfortable with command-line tools — such as accountants, administrative staff, or developers who prefer visual feedback — lack a graphical interface for:

- Editing document definitions with real-time validation and autocomplete
- Previewing generated XML before signing and sending
- Inspecting SUNAT responses (CDR metadata, error details) in a readable format
- Managing credentials and SUNAT endpoint configuration without environment variables

This ADR defines a native desktop application built with Tauri v2 that provides a GUI counterpart to the CLI, reusing the same xhandler-rust crates directly via Rust backend commands.

## Decision

### 1. Application Framework: Tauri v2

The desktop app uses **Tauri v2** with a web-based frontend rendered in the system webview:

- **Small binary size** — Uses the OS webview instead of bundling Chromium (~10-15 MB vs ~150 MB).
- **Rust backend** — Tauri commands are Rust functions, allowing direct calls to xbuilder, xsigner, and xsender without shelling out to the CLI binary.
- **Cross-platform** — Builds for Windows, macOS, and Linux from the same codebase.
- **Security model** — Tauri v2's capability-based permissions restrict frontend access to only the Tauri commands explicitly exposed.

### 2. Frontend: React + PatternFly

The frontend uses **React** with **PatternFly React** (Red Hat's open-source design system):

- **PatternFly** provides production-ready components (buttons, toolbars, modals, drawers, split panels, alerts, dropdowns) with consistent UX patterns and accessibility built in.
- **React** integrates naturally with PatternFly's `CodeEditor` component (`@patternfly/react-code-editor`), which wraps Monaco Editor.
- **TypeScript** for type safety across the frontend codebase.
- **Vite** as the build tool (Tauri v2's recommended bundler for React).

### 3. Crate and Directory Structure

```
desktop/
  src-tauri/
    Cargo.toml            # name = "openubl-desktop", depends on xhandler
    tauri.conf.json       # Window size, title, permissions
    capabilities/         # Tauri v2 capability definitions
    src/
      main.rs             # Tauri entry point, register commands
      commands/
        mod.rs
        create.rs         # create_xml command
        sign.rs           # sign_xml command
        send.rs           # send_xml command
        apply.rs          # apply_pipeline command
        verify_ticket.rs  # verify_ticket command
        schema.rs         # get_schema, get_document_kinds commands
      settings.rs         # Settings persistence (credentials, keys, URLs)
      state.rs            # Tauri managed state (AppSettings)
    build.rs              # Generate JSON schemas at build time
    schemas/              # Build-time generated JSON schemas (gitignored)
  src/                    # Frontend source (React + TypeScript)
    App.tsx
    components/
      DocumentEditor.tsx        # Monaco Editor wrapper
      OutputPanel.tsx           # Side panel for step outputs
      StepBreadcrumb.tsx        # Breadcrumb to revisit previous outputs
      DocumentKindSelector.tsx  # Dropdown for kind selection
      SettingsModal.tsx         # Settings dialog
      Toolbar.tsx               # Action buttons (Create, Sign, Send, Apply)
    hooks/
      useDocument.ts      # State for editor content and parsed state
      useSettings.ts      # State for app settings
      usePipeline.ts      # State for pipeline steps and outputs
    schemas/              # Embedded JSON schemas (copied from build output)
    tauri.ts              # Typed wrappers around Tauri invoke() calls
  package.json
  vite.config.ts
  tsconfig.json
```

Add `"desktop/src-tauri"` to workspace members in the root `Cargo.toml`.

### 4. Tauri Commands (Rust Backend)

Five Tauri commands mirror the CLI subcommands. Each is an `async` function annotated with `#[tauri::command]` that calls the xhandler-rust library crates directly.

```rust
#[tauri::command]
async fn create_xml(document_json: String) -> Result<String, String>;

#[tauri::command]
async fn sign_xml(
    xml_content: String,
    private_key_pem: String,
    certificate_pem: String,
) -> Result<String, String>;

#[tauri::command]
async fn send_xml(
    signed_xml: String,
    username: String,
    password: String,
    urls: UrlsConfig,
) -> Result<SendResponse, String>;

#[tauri::command]
async fn apply_pipeline(
    document_json: String,
    private_key_pem: String,
    certificate_pem: String,
    username: String,
    password: String,
    urls: UrlsConfig,
) -> Result<ApplyResponse, String>;

#[tauri::command]
async fn verify_ticket(
    ticket: String,
    username: String,
    password: String,
    urls: UrlsConfig,
) -> Result<VerifyResponse, String>;
```

Additional utility commands:

```rust
/// Returns the JSON schema for a given document kind
#[tauri::command]
fn get_schema(kind: String) -> Result<String, String>;

/// Returns the list of available document kinds
#[tauri::command]
fn get_document_kinds() -> Vec<String>;
```

**Key difference from CLI**: The Tauri commands receive credentials and key material as string arguments (loaded from the settings store or file picker), rather than as file paths with environment variable fallbacks.

### 5. JSON Schema Generation

JSON schemas are derived from the Rust model structs at build time to power Monaco's validation, autocomplete, and error markers.

1. Add the `schemars` crate (with `JsonSchema` derive) to xbuilder's dependencies behind a `schema` feature flag.
2. Derive `JsonSchema` on the model structs: `Invoice`, `CreditNote`, `DebitNote`, `DespatchAdvice`, `Perception`, `Retention`, `SummaryDocuments`, `VoidedDocuments`, and their nested types (`Proveedor`, `Cliente`, `Detalle`, `Direccion`, etc.).
3. In `desktop/src-tauri/build.rs`, generate one JSON schema file per document kind and write them to `desktop/src-tauri/schemas/`. These are also copied to `desktop/src/schemas/` for Monaco.
4. Each schema wraps the `spec` fields inside a top-level object with `kind` as a const string and `spec` as the document schema — matching the `kind`/`spec` input format.

**Catalog code enums**: SUNAT catalog values (Catalog1, Catalog6, Catalog7, etc.) are annotated with `#[schemars(with = "CatalogNSchema")]` helper types that produce `oneOf` schemas with descriptions, enabling Monaco autocomplete to show labels like `"01" — Factura`, `"03" — Boleta de Venta`.

### 6. Text Editor: PatternFly CodeEditor

**PatternFly CodeEditor** (`@patternfly/react-code-editor`) wraps Monaco Editor with PatternFly-styled chrome and built-in toolbar actions. It provides a consistent look-and-feel with the rest of the PatternFly UI and reduces boilerplate for common editor features.

The document editor (editable) uses:

| Feature | Implementation |
|---------|----------------|
| Syntax highlighting | Built-in JSON and YAML language support via `Language.json`, `Language.yaml` |
| Schema validation | `onEditorDidMount` callback configures `monaco.languages.json.jsonDefaults.setDiagnosticsOptions()` with generated JSON schemas |
| Inline error markers | Automatic from Monaco's JSON schema validation — red squiggles on missing/invalid fields |
| Autocomplete | Automatic from JSON schema — suggests field names, enum values with descriptions |
| Catalog code hints | Schema `enum` + `enumDescriptions` produce suggestions like `"01" — Factura` |
| Copy button | Built-in via `isCopyEnabled` prop |
| Download button | Built-in via `isDownloadEnabled` prop |
| Upload/drag-drop | Built-in via `isUploadEnabled` prop — allows dragging a JSON/YAML file into the editor |

The output panel (read-only) uses the same `CodeEditor` component with `isReadOnly`, `isCopyEnabled`, and `isDownloadEnabled` to display generated XML, signed XML, and SUNAT responses with appropriate syntax highlighting (`Language.xml`, `Language.json`, `Language.plaintext`).

**YAML support**: Monaco does not natively validate YAML against JSON schemas. YAML editing uses `yaml-language-server` integration or a custom approach: parse YAML to JSON on change (debounced), validate the JSON against the schema, and map diagnostics back to YAML line positions.

### 7. UI Layout

The application is a single page with a split-panel layout using PatternFly's `DrawerContent` component:

```
+----------------------------------------------------------------------+
| Toolbar                                                              |
| [Kind v] [JSON/YAML v] [Create XML] [Sign XML] [Send] [Apply] [Settings] [Beta o/o] |
+-------------------------------+--------------------------------------+
|                               |                                      |
|  Document Editor              |  Output Panel                        |
|  (PatternFly CodeEditor)      |  [Step breadcrumb: Create > Sign >   |
|                               |   Send]                              |
|  JSON/YAML with schema        |                                      |
|  validation, autocomplete,    |  Output content for the current      |
|  inline error markers,        |  step (XML syntax highlighted via    |
|  copy/download/upload buttons |  read-only CodeEditor, or            |
|                               |  formatted JSON for SUNAT responses) |
|                               |                                      |
|                               |                                      |
+-------------------------------+--------------------------------------+
| Status bar: [Validation: 0 errors] [Mode: Beta] [Ready]             |
+----------------------------------------------------------------------+
```

### 8. Progressive Button Flow

The toolbar buttons follow a progressive enablement pattern:

| Button | Enabled when | Action | Output in side panel |
|--------|-------------|--------|---------------------|
| **Create XML** | Editor has valid content (0 schema errors) | Calls `create_xml` | Unsigned UBL XML (read-only CodeEditor with XML highlighting) |
| **Sign XML** | Create XML succeeded | Calls `sign_xml` with XML from previous step + credentials from settings | Signed XML |
| **Send** | Sign XML succeeded | Calls `send_xml` with signed XML + credentials from settings | SUNAT response: CDR metadata JSON, ticket ID, or error details |
| **Apply** | Editor has valid content (0 schema errors) | Calls `apply_pipeline` (full create + sign + send) | Final SUNAT response (with intermediate outputs accessible via breadcrumb) |
| **Verify Ticket** | Shown only when a ticket ID exists from a previous Send | Calls `verify_ticket` | CDR metadata or status |

**Side panel behavior**:

- The side panel replaces its content with each step's output.
- A **breadcrumb** (`StepBreadcrumb`) at the top of the side panel shows completed steps: `Create > Sign > Send`. Clicking a previous step shows that step's cached output.
- When the user modifies the editor content, the pipeline state resets — all buttons after "Create XML" become disabled again, and cached outputs are cleared.
- The **Apply** button runs the full pipeline and populates all breadcrumb steps at once. The side panel shows the final result, but the user can click back through the breadcrumb to see intermediate XML.

### 9. Settings Modal

A PatternFly `Modal` dialog for configuring credentials and options. Settings are persisted using `tauri-plugin-store` (key-value storage in the app's data directory).

| Setting | PatternFly Component | Description |
|---------|---------------------|-------------|
| Private Key | `FileUpload` or file path `TextInput` with browse button | Path to PKCS#1 PEM private key |
| Certificate | `FileUpload` or file path `TextInput` with browse button | Path to X.509 PEM certificate |
| Username | `TextInput` | SUNAT SOL username |
| Password | `TextInput` (type=password) | SUNAT SOL password |
| SUNAT Mode | `Switch` | Beta / Production toggle |
| Invoice URL | `TextInput` (optional, expandable) | Custom SUNAT invoice endpoint override |
| Perception/Retention URL | `TextInput` (optional, expandable) | Custom endpoint override |
| Despatch URL | `TextInput` (optional, expandable) | Custom endpoint override |

**Beta mode**: When active, the toolbar shows a visible "BETA" badge (PatternFly `Label`). Sign and send commands use built-in beta certificates and credentials as defaults (same as CLI's `--beta` flag). SUNAT URLs switch to beta endpoints automatically.

### 10. File Operations

File operations use `tauri-plugin-dialog` for native Open/Save dialogs:

| Operation | Trigger | Behavior |
|-----------|---------|----------|
| Open document | Ctrl+O or toolbar menu | Opens `.json`/`.yaml` file into the editor |
| Save document | Ctrl+S | Saves editor content to current file (or prompts on first save) |
| Save As | Ctrl+Shift+S | Prompts for new path and saves |
| Export XML | Button in side panel output | Saves the displayed XML to a chosen path |
| Export CDR | Button in side panel (SUNAT response) | Saves the CDR ZIP to a chosen path |

### 11. Document Kind Selector

A PatternFly `Select` dropdown in the toolbar lists the 8 document kinds:

- Invoice, CreditNote, DebitNote, DespatchAdvice, Perception, Retention, SummaryDocuments, VoidedDocuments

Changing the kind:

1. Updates the `kind` field in the editor content.
2. Loads the corresponding JSON schema into Monaco's validation and autocomplete.
3. If the editor is empty, inserts a minimal template for the selected kind with required fields as placeholders.

### 12. Error Handling

Errors are surfaced at three levels:

1. **Editor-level (inline)** — The PatternFly CodeEditor (via Monaco internally) produces red squiggly underlines on invalid fields through JSON schema validation. The status bar shows total error count. Errors include missing required fields, invalid types, unknown field names, and invalid enum values for catalog codes.

2. **Operation-level (alerts)** — When a Tauri command fails (e.g., signing error, network error), a PatternFly `Alert` (danger variant) appears at the top of the side panel with the error message.

3. **SUNAT-level (response panel)** — SUNAT rejection responses are displayed in the side panel with error code, message, and notes formatted as a PatternFly `DescriptionList`. Success responses show a green `Alert` with CDR metadata.

**Validation gating**: The Create XML and Apply buttons are disabled when there are schema validation errors, with a PatternFly `Tooltip` explaining why.

### 13. Tauri Plugins

| Plugin | Purpose |
|--------|---------|
| `tauri-plugin-dialog` | Native file open/save dialogs |
| `tauri-plugin-store` | Persistent settings storage |
| `tauri-plugin-fs` | Read/write files for import/export |

### 14. Build Pipeline

1. **Rust build** (`desktop/src-tauri/build.rs`): Generates JSON schemas from model structs using `schemars`, writes to `desktop/src-tauri/schemas/` and `desktop/src/schemas/`.

2. **Frontend build** (Vite): Compiles React/TypeScript, bundles PatternFly components (including CodeEditor with Monaco) and CSS.

```bash
# Install frontend dependencies
cd desktop && npm install

# Dev mode with hot reload
cd desktop && npm run tauri dev

# Production build
cd desktop && npm run tauri build
```

## Implementation Phases

1. **Phase 1: Tauri scaffold + Create command** — Set up `desktop/` with Tauri v2 + React + PatternFly. Implement `create_xml` Tauri command. Build a basic single-page layout with PatternFly CodeEditor (JSON mode) and a side panel showing the generated XML.

2. **Phase 2: JSON Schema validation** — Add `schemars` derives to xbuilder model structs (behind `schema` feature flag). Implement build-time schema generation. Configure CodeEditor's `onEditorDidMount` to set Monaco's JSON diagnostics with the generated schemas. Add the document kind dropdown. Add catalog code autocomplete via schema enums.

3. **Phase 3: Sign + Send + Settings** — Implement `sign_xml`, `send_xml`, and `verify_ticket` Tauri commands. Build the Settings modal with PatternFly form components. Add the beta mode toggle. Implement the progressive button flow and step breadcrumb in the side panel.

4. **Phase 4: Apply pipeline + File operations** — Implement `apply_pipeline` command. Add Open/Save/Export file dialogs. Add YAML editing support with schema validation. Polish the breadcrumb navigation for intermediate outputs.

5. **Phase 5: Polish + Distribution** — Error handling refinements, keyboard shortcuts, app icons, installer configuration (`.msi`, `.dmg`, `.deb`/`.AppImage`).

## Consequences

### Positive

- Provides a visual, low-barrier interface for users who do not use the command line — accountants and administrative staff can create, validate, sign, and send electronic invoices without terminal knowledge.
- PatternFly CodeEditor wraps Monaco with a consistent UI (copy/download/upload buttons) and its built-in JSON schema validation provides immediate feedback (red squiggles, autocomplete) without custom validation code — the JSON schemas drive everything.
- Reuses the same xhandler-rust crates as the CLI, ensuring identical XML generation, signing, and sending behavior — no feature drift between CLI and GUI.
- PatternFly provides a consistent, accessible, production-quality design system with minimal custom CSS.
- Tauri v2's small binary size (~10-15 MB) makes distribution practical.
- The JSON schema generation infrastructure (schemars derives) benefits the project beyond the desktop app — schemas can be published for IDE extensions or third-party integrations.
- Local-only architecture means no server to deploy and no data leaves the user's machine except SUNAT SOAP/REST calls.

### Negative

- Adding `schemars` derives to xbuilder model structs (even behind a feature flag) increases derive macro surface area and may require custom schema implementations for types like `NaiveDate`, `NaiveTime`, and `Decimal`.
- The frontend stack (React, PatternFly, PatternFly CodeEditor, Vite, npm) introduces a JavaScript/TypeScript toolchain into a previously Rust-only project.
- PatternFly CodeEditor bundles Monaco internally (~2 MB+). This is acceptable for a desktop app but increases initial load time compared to lighter editors.
- Tauri v2 depends on the system webview (WebKitGTK on Linux, WebView2 on Windows, WKWebView on macOS). WebKitGTK may require system packages not pre-installed on all Linux distributions.
- The underlying Monaco editor's YAML support does not include built-in schema validation. YAML validation requires parsing to JSON first and mapping diagnostics back to YAML positions, which adds complexity and may produce imprecise error locations for YAML-specific syntax issues.
- The desktop app must stay in sync with model changes in xbuilder — adding or modifying fields requires regenerating schemas.
