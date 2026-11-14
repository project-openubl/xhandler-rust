# xhandler-wasm

The xhandler core as one WebAssembly module (`wasm32-wasip1`), shared by the Go
([xhandler-go](https://github.com/project-openubl/xhandler-go)), Python
([xhandler-python](https://github.com/project-openubl/xhandler-python)) and Java
([xhandler-java](https://github.com/project-openubl/xhandler-java)) libraries. They run
the same Rust code, so they produce the same XML, signatures and SUNAT requests.

## ABI

Three exports, one JSON entry point:

| Export | |
|---|---|
| `xh_alloc(len) -> ptr` | reserves the request |
| `xh_invoke(ptr, len) -> u64` | runs `{"op", "input"}`; returns `(ptr << 32) \| len` of `{"ok": ...}` or `{"error": {"kind", "message"}}` |
| `xh_free(ptr, len)` | releases (and zeroes) the request and the response |

Bytes travel as base64. An instance runs one call at a time; a trap (a panic) leaves
it unusable, so hosts discard it.

| Op | Input | Ok |
|---|---|---|
| `build` | `{document: {kind, spec}, defaults?}` | the XML |
| `enrich` | same | the enriched spec |
| `sign` | `{xml, key: {pem: {private_key, certificate}} \| {pkcs12: {der, password}}}` | the signed XML |
| `prepare_send_file` | `{xml, urls}` | `{file: {filename, zip}, send_target, verify_ticket_target}` |
| `prepare_send_pack` | `{name, files: [xml], urls}` | same |
| `verify_ticket_target` | `{xml, urls}` | the target of the tickets |
| `request` | `{call, credentials, token?}` | `{method, url, headers, body}` |
| `parse` | `{call, status, body}` | `{cdr}`, `{ticket}`, `{no_cdr}`, `{status}`, `{token}` or `{fault}` |
| `version` | | the version |

`defaults` are optional: today in Peru and the current SUNAT tax rates. The hosts only
send the HTTP requests built by `request` (retrying `rate_limited` answers after 1s and
2s, and REST calls rejected with HTTP 401 once with a new token) and cache the OAuth
tokens of the guías.

Error kinds: `invalid_request`, `invalid_input`, `validation`, `render`, `invalid_key`,
`response`, `rate_limited`.

## Build

```shell
make -C xhandler-wasm dist
```

writes `dist/`: `xhandler.wasm`, `VERSION`, `xhandler.schema.json` (JSON Schema of the
models), `models/` (the Go, Python and Java model types, from `codegen.py`) and
`conformance/` (the cases every binding must pass: `<op>/<name>.json` with `input` and
`expected`). The bindings copy it with `make sync`.
