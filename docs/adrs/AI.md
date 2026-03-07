# ADR: AI-Assisted Document Creation

## Status

Proposed

## Context

The xhandler-rust project provides three Rust library crates for Peru SUNAT electronic invoicing:

- **xbuilder** — Creates UBL XML from Rust structs via a 3-phase enrichment pipeline and Tera templates
- **xsigner** — Signs XML documents with RSA-SHA256 digital signatures (libxml2/openssl C14N)
- **xsender** — Sends signed XML to SUNAT via SOAP/REST, parses CDR responses

The existing CLI (`openubl`) lets users define documents in YAML/JSON files using a `kind`/`spec` format, run them through the enrichment pipeline, and render/sign/send UBL XML. However, creating correct document definitions still requires expertise in SUNAT catalog codes, document structure, and tax rules. Users need to know which `tipo_operacion` codes exist, which `igv_tipo` values are valid, how detracciones work, etc.

An AI-assisted interface would let users describe invoices in natural language (e.g., "crear una factura para 2 laptops a 3500 soles cada una, vendidas a RUC 20987654321, con pago a crédito en 2 cuotas") and have the system produce a correct document definition. Users should also be able to iteratively refine documents through follow-up prompts ("agregar un descuento del 10%", "cambiar el cliente").

This ADR defines the architecture for two new crates that bring AI capabilities to the project:
1. An MCP (Model Context Protocol) server that exposes xhandler capabilities as tools for any AI client
2. A standalone CLI that embeds LLM calls for users without an MCP client

## Decision

### 1. Architecture Overview

Two separate crates with distinct responsibilities:

```
┌─────────────────────────┐     ┌─────────────────────────┐
│  MCP Client             │     │  openubl-ai             │
│  (Claude Desktop,       │     │  (standalone CLI)       │
│   VS Code, Cursor)      │     │                         │
└────────────┬────────────┘     └────────────┬────────────┘
             │ MCP protocol                  │ LLM API calls
             │ (stdio / HTTP-SSE)            │ (Claude/OpenAI/Ollama)
┌────────────▼────────────┐     ┌────────────▼────────────┐
│  mcp/                   │     │  cli-ai/                │
│  xhandler-mcp           │     │  openubl-ai-cli         │
│  (MCP server)           │     │  (terminal AI assistant)│
└────────────┬────────────┘     └────────────┬────────────┘
             │                               │
             │  lib.rs (shared session +     │
             │  document operations)         │
             └───────────────┬───────────────┘
                             │
                ┌────────────▼────────────┐
                │  Session Manager        │
                │  + Document Operations  │
                └────────────┬────────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
         ┌────▼───┐   ┌─────▼────┐   ┌────▼─────┐
         │xbuilder│   │ xsigner  │   │ xsender  │
         └────────┘   └──────────┘   └──────────┘
```

The `mcp/` crate exposes shared logic (session management, document operations) as a library via `lib.rs`. The `cli-ai/` crate imports it as a dependency and wraps the same operations in an LLM conversation loop.

### 2. MCP Server Crate (`mcp/`)

A pure MCP server — no LLM dependency. It exposes xhandler capabilities as MCP tools that any MCP-compatible client can call. Follows the patterns established by trustify-mcp.

#### Crate Structure

```
mcp/
  Cargo.toml              # name = "xhandler-mcp"
  src/
    lib.rs                # Library root: re-exports session + operations for cli-ai
    bin/
      stdio.rs            # MCP server via stdio transport
      streamhttp.rs       # MCP server via HTTP/SSE transport
      common/
        mod.rs
        handler.rs        # XhandlerMcp struct + #[tool_router] impl
        requests.rs       # Request DTOs with schemars derives
    session.rs            # SessionManager + DocumentState enum (pub)
    operations.rs         # Document operations (pub)
```

Add `"mcp"` to workspace members in the root `Cargo.toml`.

#### Dependencies

```toml
[dependencies]
xhandler = { path = "../xhandler" }
rmcp = { version = "1.1.0", features = ["server", "transport-io", "transport-streamable-http-server"] }
axum = { version = "0.8", features = ["macros"] }
dashmap = "6"
schemars = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt", "rt-multi-thread", "io-std", "signal"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "std", "fmt"] }
uuid = { version = "1", features = ["v4"] }
chrono = "0.4"
```

**Key decisions:**

- **`rmcp` v1.1.0** over a TypeScript MCP server — keeps the project pure Rust, avoids FFI/subprocess overhead, provides native access to xbuilder structs and enrichment pipeline. Same SDK used by trustify-mcp.
- **`schemars`** for JSON Schema generation — `#[derive(JsonSchema)]` on request DTOs with `#[schemars(description = "...")]` for rich field documentation. Schemas are served as MCP tool input schemas, helping LLMs understand field structure and types.
- **`dashmap`** for concurrent session storage — lock-free `DashMap<SessionId, DocumentSession>` allows multiple MCP clients to use the server simultaneously.

#### Transport

Two binaries, one per transport (matching trustify-mcp):

**stdio** (`bin/stdio.rs`) — For local use with Claude Desktop and IDEs. The server runs as a subprocess; communication via stdin/stdout.

```rust
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let service = XhandlerMcp::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
```

**Streamable HTTP** (`bin/streamhttp.rs`) — For remote/web use. Runs as an HTTP server with SSE transport.

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let service = StreamableHttpService::new(
        || Ok(XhandlerMcp::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let router = axum::Router::new().nest_service("/mcp", service);
    let listener = tokio::net::TcpListener::bind("[::]:8082").await?;
    axum::serve(listener, router)
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await?;
    Ok(())
}
```

#### ServerHandler Implementation

```rust
#[tool_handler]
impl ServerHandler for XhandlerMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .build(),
        )
        .with_protocol_version(ProtocolVersion::V_2025_03_26)
        .with_server_info(Implementation::new(
            "xhandler-mcp",
            env!("CARGO_PKG_VERSION"),
        ))
        .with_instructions(
            "MCP server for Peru SUNAT electronic invoicing. \
             Create, validate, sign, and send UBL XML documents \
             (Invoice, CreditNote, DebitNote, DespatchAdvice, \
             Perception, Retention, SummaryDocuments, VoidedDocuments) \
             through conversational tool calls. Use reference tools \
             (list_catalog_codes, describe_document_type) to look up \
             valid SUNAT catalog codes before constructing documents."
        )
    }
}
```

#### Session Management

The MCP server maintains stateful sessions for iterative document construction:

```rust
pub type SessionId = String;

pub enum DocumentState {
    Invoice(Box<Invoice>),
    CreditNote(Box<CreditNote>),
    DebitNote(Box<DebitNote>),
    DespatchAdvice(Box<DespatchAdvice>),
    Perception(Box<Perception>),
    Retention(Box<Retention>),
    SummaryDocuments(Box<SummaryDocuments>),
    VoidedDocuments(Box<VoidedDocuments>),
}

pub struct DocumentSession {
    pub id: SessionId,
    pub state: DocumentState,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub enriched_xml: Option<String>,
    pub signed_xml: Option<String>,
}

pub struct SessionManager {
    sessions: DashMap<SessionId, DocumentSession>,
}
```

- **In-memory only** — sessions are lost on server restart. Acceptable because MCP clients can recreate documents quickly.
- **Cache invalidation** — any mutation invalidates `enriched_xml` and `signed_xml`, forcing re-enrichment on next `render_xml` call.
- **Session TTL** — configurable expiration (default: 30 minutes) with background cleanup.

#### Configuration

Environment variables only (no CLI args), matching trustify-mcp:

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `RUST_LOG` | No | `info` | Log level filter |
| `BIND_ADDRESS` | No | `[::]:8082` | HTTP server bind address (streamhttp only) |
| `SESSION_TTL_SECS` | No | `1800` | Session expiration in seconds |

#### Claude Desktop Configuration

```json
{
  "mcpServers": {
    "xhandler": {
      "command": "xhandler-mcp",
      "args": []
    }
  }
}
```

### 3. MCP Tool Design

Tools are organized by concern. The design provides both granular tools (for iterative construction) and coarse tools (for one-shot generation).

#### 3.1 Document Lifecycle Tools

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| `create_document` | Creates a new session with an empty document of the given type. Returns `session_id`. | `document_type`: Invoice, CreditNote, DebitNote, DespatchAdvice, Perception, Retention, SummaryDocuments, VoidedDocuments |
| `create_document_from_json` | Creates a session pre-populated from a full JSON document (matching the CLI `kind`/`spec` format). | `document_json`: full JSON |
| `get_document` | Returns the current document state as JSON. | `session_id` |
| `list_sessions` | Lists active sessions with their document types and timestamps. | none |
| `delete_session` | Deletes a session and its cached artifacts. | `session_id` |

#### 3.2 Document Mutation Tools

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| `set_serie_numero` | Sets the document series and number. | `session_id`, `serie_numero` (e.g., "F001-1") |
| `set_moneda` | Sets the currency. | `session_id`, `moneda` (e.g., "PEN", "USD") |
| `set_fecha_emision` | Sets the emission date. | `session_id`, `fecha_emision` |
| `set_tipo_operacion` | Sets the operation type (Catalog51). | `session_id`, `tipo_operacion` |
| `set_supplier` | Sets the proveedor (supplier). | `session_id`, `ruc`, `razon_social`, optional: `nombre_comercial`, `direccion` |
| `set_customer` | Sets the cliente (customer). | `session_id`, `tipo_documento_identidad`, `numero_documento_identidad`, `nombre`, optional: `direccion` |
| `set_signer` | Sets the firmante (signer). | `session_id`, `ruc`, `razon_social` |
| `add_line_item` | Adds a detalle (line item). Returns item index. | `session_id`, `descripcion`, `cantidad`, `precio`, optional: `unidad_medida`, `igv_tipo`, `isc_tipo`, `codigo` |
| `update_line_item` | Updates a detalle at a given index. Only provided fields change. | `session_id`, `index`, partial fields |
| `remove_line_item` | Removes a detalle at a given index. | `session_id`, `index` |
| `set_payment_terms` | Sets forma de pago. | `session_id`, `tipo` (Credito/Contado), optional: `total` |
| `add_payment_installment` | Adds a payment installment (cuota). | `session_id`, `importe`, `fecha_pago` |
| `set_detraccion` | Sets detraccion information. | `session_id`, `medio_de_pago`, `cuenta_bancaria`, `tipo_bien_detraido`, `porcentaje` |
| `set_percepcion` | Sets percepcion information. | `session_id`, `tipo`, optional: `porcentaje`, `monto`, `monto_base`, `monto_total` |
| `add_descuento` | Adds a discount. | `session_id`, `monto`, optional: `tipo`, `monto_base`, `factor` |
| `add_anticipo` | Adds a prepayment. | `session_id`, fields per Anticipo struct |
| `update_document_json` | Bulk-updates the document by merging a partial JSON. | `session_id`, `partial_json` |

#### 3.3 Pipeline Tools

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| `enrich_document` | Runs the 3-phase enrichment pipeline (fill defaults, compute taxes, summarize totals). Returns the enriched document as JSON. | `session_id` |
| `validate_document` | Validates document completeness and correctness. Returns errors and warnings. | `session_id` |
| `render_xml` | Enriches and renders the document to UBL XML. Returns the XML string. | `session_id` |
| `sign_xml` | Signs a rendered XML with RSA credentials. Returns signed XML. | `session_id`, `private_key_pem`, `certificate_pem` |
| `send_to_sunat` | Sends a signed document to SUNAT. Returns CDR metadata or ticket. | `session_id`, `username`, `password`, optional: `beta` (default true) |
| `verify_ticket` | Checks status of an async submission. | `ticket_id`, `username`, `password`, optional: `beta` |

#### 3.4 Reference Tools

These tools provide SUNAT domain knowledge to help the LLM construct correct documents without hallucinating catalog codes.

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| `list_catalog_codes` | Lists all valid codes for a SUNAT catalog. Returns `{code, label}` pairs. | `catalog_name` (e.g., "Catalog7", "Catalog51") |
| `get_tax_rates` | Returns current default tax rates (IGV 18%, ICB 0.20, IVAP 4%). | none |
| `describe_document_type` | Returns a description of a document type with required/optional fields and a minimal example. | `document_type` |
| `get_document_schema` | Returns the full JSON Schema for a document type. | `document_type` |

#### 3.5 Tool Implementation Pattern

Following trustify-mcp, tools use `rmcp` attribute macros with `schemars` request DTOs:

```rust
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AddLineItemRequest {
    #[schemars(description = "Session ID from create_document")]
    pub session_id: String,
    #[schemars(description = "Item description")]
    pub descripcion: String,
    #[schemars(description = "Quantity")]
    pub cantidad: f64,
    #[schemars(description = "Unit price without taxes")]
    pub precio: f64,
    #[schemars(description = "Unit of measure code (default: ZZ). NIU=unit, KGM=kilogram")]
    pub unidad_medida: Option<String>,
    #[schemars(description = "IGV tax type - Catalog7 code (default: 10 = gravado oneroso)")]
    pub igv_tipo: Option<String>,
}

#[tool_router]
impl XhandlerMcp {
    #[tool(description = "Add a line item (detalle) to a document. Returns the index of the added item.")]
    async fn add_line_item(
        &self,
        Parameters(params): Parameters<AddLineItemRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        // Dispatch to shared operations in session.rs / operations.rs
    }
}
```

### 4. Conversational Flow Example

**Example: Creating an invoice through Claude Desktop**

1. User: "Crear una factura para mi empresa (RUC 20123456789, Mi Empresa SAC) vendiendo 2 laptops a 3500 soles a RUC 20987654321"
2. Claude calls `create_document(document_type: "Invoice")` → `session_id: "abc123"`
3. Claude calls `set_serie_numero(session_id: "abc123", serie_numero: "F001-1")`
4. Claude calls `set_supplier(session_id: "abc123", ruc: "20123456789", razon_social: "Mi Empresa S.A.C.")`
5. Claude calls `set_customer(session_id: "abc123", tipo_documento_identidad: "6", numero_documento_identidad: "20987654321", nombre: "Cliente Corp S.A.")`
6. Claude calls `add_line_item(session_id: "abc123", descripcion: "Laptop", cantidad: 2, precio: 3500.00, unidad_medida: "NIU")`
7. Claude calls `enrich_document(session_id: "abc123")` → sees computed taxes (IGV 1260.00, total 8260.00)
8. Claude reports: "Factura creada. Total: S/ 8,260.00 (incluye IGV S/ 1,260.00)"

**Follow-up: "Agregar un descuento del 10%"**

9. Claude calls `add_descuento(session_id: "abc123", factor: 0.10)`
10. Claude calls `enrich_document(session_id: "abc123")` → updated totals
11. Claude reports: "Descuento aplicado. Nuevo total: S/ 7,434.00"

**Follow-up: "Generar el XML"**

12. Claude calls `render_xml(session_id: "abc123")` → UBL XML string
13. Claude presents the XML to the user

### 5. CLI-AI Crate (`cli-ai/`)

A standalone terminal binary for users without an MCP client. Similar in structure to the existing `cli/` crate. Embeds LLM calls and dispatches tool calls to the same session/operations logic from `mcp/`.

#### Crate Structure

```
cli-ai/
  Cargo.toml              # name = "openubl-ai-cli", [[bin]] name = "openubl-ai"
  src/
    main.rs               # Entry point, clap parsing
    commands/
      mod.rs
      chat.rs             # Interactive multi-turn conversation
      generate.rs         # Single-shot prompt → document
    provider/
      mod.rs              # LlmProvider trait
      claude.rs           # Anthropic Claude API (tool use)
      openai.rs           # OpenAI API (function calling)
      ollama.rs           # Ollama local models
    conversation.rs       # Conversation loop: prompt → LLM → tool dispatch → response
```

Add `"cli-ai"` to workspace members in the root `Cargo.toml`.

#### Dependencies

```toml
[dependencies]
xhandler-mcp = { path = "../mcp" }   # Shared session + operations
clap = { version = "4", features = ["derive", "env"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
tracing = "0.1"
```

#### Subcommands

##### `openubl-ai chat`

Interactive multi-turn conversation. The user types prompts in the terminal; the CLI calls the LLM API, dispatches tool calls to the session manager, and prints results.

| Flag | Short | Required | Env Var | Description |
|------|-------|----------|---------|-------------|
| `--provider <NAME>` | `-p` | Yes | `OPENUBL_AI_PROVIDER` | LLM provider: `claude`, `openai`, `ollama` |
| `--api-key <KEY>` | — | Conditional | `ANTHROPIC_API_KEY` / `OPENAI_API_KEY` | API key (not needed for Ollama) |
| `--model <MODEL>` | `-m` | No | `OPENUBL_AI_MODEL` | Model name override (default per provider) |
| `--base-url <URL>` | — | No | `OPENUBL_AI_BASE_URL` | API base URL override (for Ollama or proxies) |

```bash
# Interactive chat with Claude
openubl-ai chat --provider claude --api-key $ANTHROPIC_API_KEY

# Interactive chat with local Ollama
openubl-ai chat --provider ollama --model llama3.1

# Interactive chat with OpenAI
openubl-ai chat --provider openai --api-key $OPENAI_API_KEY
```

##### `openubl-ai generate`

Single-shot: takes a prompt, generates a document, outputs as YAML/JSON/XML.

| Flag | Short | Required | Env Var | Description |
|------|-------|----------|---------|-------------|
| `--provider <NAME>` | `-p` | Yes | `OPENUBL_AI_PROVIDER` | LLM provider |
| `--api-key <KEY>` | — | Conditional | Provider-specific | API key |
| `--output <PATH>` | `-o` | No | — | Output file. Writes to stdout if omitted. |
| `--format <FMT>` | — | No | — | Output format: `yaml`, `json`, `xml` (default: `yaml`) |

```bash
# Generate YAML to stdout
openubl-ai generate "Crear una factura para 2 laptops a 3500 soles" \
  --provider claude --api-key $ANTHROPIC_API_KEY

# Generate XML to file
openubl-ai generate "Create an invoice for 10 monitors at 800 USD" \
  --provider openai --api-key $OPENAI_API_KEY --format xml -o invoice.xml
```

#### Provider Abstraction

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn chat(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
    ) -> anyhow::Result<LlmResponse>;
}

pub struct ClaudeProvider { api_key: String, model: String }
pub struct OpenAiProvider { api_key: String, model: String }
pub struct OllamaProvider { base_url: String, model: String }
```

The conversation loop:
1. Send user prompt + tool definitions to the LLM API
2. If the LLM responds with tool calls, dispatch them to `xhandler-mcp` operations in-process
3. Return tool results to the LLM
4. Repeat until the LLM produces a final text response
5. Print the response to the user

### 6. JSON Schema Generation

Adding `schemars = "0.8"` as a dependency to `xbuilder` and `#[derive(JsonSchema)]` to model structs enables automatic JSON Schema generation. These schemas are used by:
- MCP tool input schemas (helping LLMs understand field structure)
- The `get_document_schema` reference tool
- The `cli-ai` provider abstraction (tool definitions sent to LLMs)

This is a low-risk, additive change: `JsonSchema` derive does not affect serialization/deserialization behavior.

### 7. Security Considerations

- **Credentials**: The `sign_xml` and `send_to_sunat` tools accept credentials as parameters. The MCP server does not store them. For MCP clients, the LLM prompts the user for credentials. Environment variables can serve as fallback.
- **SUNAT beta default**: `send_to_sunat` defaults to `beta: true` to prevent accidental production submissions. Users must explicitly set `beta: false`.
- **Private key handling**: Private keys passed to `sign_xml` are held in memory only for the signing operation duration, not stored in the session.
- **Session isolation**: Each session is independent. No cross-session data access.
- **API keys**: In `cli-ai`, API keys are read from environment variables or CLI flags. They are never logged or persisted.

### 8. Separation Rationale

| Concern | `mcp/` | `cli-ai/` |
|---------|--------|-----------|
| Purpose | MCP protocol server | Terminal AI assistant |
| LLM dependency | None (provider-agnostic) | Claude, OpenAI, Ollama |
| Runtime model | Long-running process | Interactive or run-and-exit |
| User interaction | Via MCP client (Claude Desktop, etc.) | Direct terminal I/O |
| Framework | rmcp + axum | clap (like existing `cli/`) |
| Transport | stdio / HTTP-SSE | N/A (direct API calls) |
| Dependencies | rmcp, axum, dashmap | reqwest, clap + xhandler-mcp |

Keeping these separate from the existing `cli/` crate avoids:
- **Dependency bloat**: rmcp, axum, dashmap, schemars are not needed by the core `openubl` CLI.
- **Runtime model mixing**: A run-and-exit CLI tool should not carry MCP server infrastructure.
- **Release coupling**: AI tooling evolves at a different pace than the core invoicing pipeline.

## Implementation Phases

1. **Phase 1: MCP scaffold + session layer** — Create `mcp/` crate. Implement `SessionManager`, `DocumentState`, and lifecycle tools (`create_document`, `get_document`, `delete_session`). Set up `rmcp` server with stdio transport. Verify with MCP Inspector.

2. **Phase 2: Mutation + reference tools** — Implement all document mutation tools and reference tools. Add `schemars` derive to xbuilder model structs for schema generation.

3. **Phase 3: Pipeline tools** — Implement `enrich_document`, `validate_document`, `render_xml`, `sign_xml`, `send_to_sunat`, `verify_ticket`. Add streamable HTTP transport binary.

4. **Phase 4: CLI-AI scaffold** — Create `cli-ai/` crate with clap structure, `chat` and `generate` subcommands, and the `LlmProvider` trait.

5. **Phase 5: Provider implementations** — Implement `ClaudeProvider`, `OpenAiProvider`, `OllamaProvider`. Wire up the conversation loop dispatching tool calls to `xhandler-mcp` operations.

6. **Phase 6: Polish** — End-to-end testing, example prompts, documentation, Claude Desktop config examples.

## Consequences

### Positive

- Any MCP-compatible AI client can generate SUNAT documents without domain expertise. Users describe what they need in natural language.
- The MCP server is provider-agnostic. It works with Claude, GPT, Gemini, local models — any client that speaks MCP.
- The standalone CLI (`openubl-ai`) provides access for users without MCP clients, supporting multiple LLM providers.
- Reuses the existing enrichment pipeline. The AI does not compute taxes or fill defaults — it provides minimal input and calls `enrich_document`.
- Reference tools (`list_catalog_codes`, `describe_document_type`) ground the LLM in factual SUNAT data, reducing hallucination of catalog codes.
- Session-based stateful design enables natural multi-turn conversations ("agregar otro item", "cambiar el cliente").
- Clean separation: `mcp/` has no LLM dependency, `cli-ai/` has no MCP protocol dependency. Each can evolve independently.
- The `schemars` JSON Schema addition to xbuilder model structs benefits other consumers (OpenAPI docs, CLI validation, IDE support).

### Negative

- Adding `schemars` derive to xbuilder model structs increases compile time slightly.
- The `rmcp` crate is relatively new (Rust MCP SDK ecosystem is young as of early 2026). API stability is not guaranteed.
- In-memory sessions are lost on MCP server restart.
- The `cli-ai` provider abstraction must track API changes across Claude, OpenAI, and Ollama — three different tool-calling APIs.
- Two new crates increase the workspace size and CI build matrix.
