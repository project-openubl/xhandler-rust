# Contribuir a xhandler-rust

Este repositorio es el núcleo de xhandler: las librerías de Go, Python y Java ejecutan este código
compilado a WebAssembly. Un cambio aquí llega a las cuatro librerías, así que los cambios de
comportamiento vienen con tests y un caso de conformidad (ver
[El núcleo WebAssembly](#el-núcleo-webassembly)).

## Entorno

- El toolchain de Rust fijado en `rust-toolchain.toml`: `rustup toolchain install` en el
  repositorio lo instala con el target `wasm32-wasip1`.
- Paquetes del sistema para los tests (la validación XSD usa libxml2):
  - Fedora: `sudo dnf install pkg-config libxml2-devel clang-devel`
  - Ubuntu: `sudo apt-get install pkg-config libxml2-dev libclang-dev`
- Opcional: Python 3 y Go, para `make -C xhandler-wasm dist`.

O abre el repositorio en el [devcontainer](.devcontainer/README.md).

```shell
cargo build
cargo test
cargo test -p xhandler --test xbuilder invoice_   # los tests de xbuilder que coinciden con un nombre
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

`XBUILDER_SKIP_SUNAT=true` evita enviar a la SUNAT beta los documentos de los tests de xbuilder (los
snapshots y la validación XSD se mantienen). Los tests que hablan con la SUNAT beta pueden fallar
cuando la SUNAT está caída o limita las peticiones.

## Estructura

| Ruta | |
|---|---|
| `xhandler/src/xbuilder/` | Modelos, pipeline de enriquecimiento; plantillas Tera en `xhandler/resources/templates/xbuilder/` |
| `xhandler/src/xsigner/` | Firma XML: C14N, RSA-SHA256, llaves PEM/PKCS#12 |
| `xhandler/src/xsender/` | Llamadas a la SUNAT sin IO (`exchange`), cliente reqwest (feature `http`), ZIP, CDR, enrutamiento |
| `xhandler/tests/` | Tests de integración (`xbuilder/` es un solo binario de tests) y sus archivos en `resources/` |
| `cli/` | El crate `xhandler-cli`, con el binario `openubl` |
| `xhandler-wasm/` | El núcleo WebAssembly de las librerías, su JSON Schema, el generador de modelos y el corpus de conformidad |

### xbuilder

El usuario da los datos esenciales; `enrich` los completa en tres fases antes de `render`:

1. **Fill** (`enricher/rules/phase1_fill/`): valores por defecto como moneda, tasas, tipos de
   documento y unidades.
2. **Process** (`phase2_process/`): valores por línea: impuestos, precios, totales de línea.
3. **Summary** (`phase3_summary/`): totales del documento, detracciones, leyendas.

Luego `validate` revisa los códigos de catálogo y los valores obligatorios. Las reglas implementan
los traits de `enricher/bounds/`. Las plantillas se incrustan al compilar (`xhandler/build.rs`).

Los tests de `xhandler/tests/xbuilder/` comparan el XML con snapshots en
`xhandler/tests/resources/xbuilder/e2e/` y lo validan contra los XSD de UBL.
`XBUILDER_SNAPSHOT_UPDATE=true cargo test -p xhandler --test xbuilder` reescribe los snapshots:
revisa el diff.

### xsender

`exchange::Call` arma cada petición HTTP (`request`) y lee cada respuesta (`parse`) sin hacer IO.
`SunatClient` (feature `http`, activa por defecto) envía las llamadas con reqwest y se encarga de los
reintentos y del caché de tokens OAuth de las guías. Las librerías usan `exchange` a través del
núcleo WebAssembly con sus propios clientes HTTP. Sin la feature `http`, xhandler debe compilar para
`wasm32-wasip1` (el CI lo verifica).

## El núcleo WebAssembly

`xhandler-wasm` expone las librerías a Go, Python y Java con un solo punto de entrada JSON (ver
[su README](xhandler-wasm/README.md) para la ABI y las operaciones).

```shell
make -C xhandler-wasm dist
```

escribe `dist/`:

- `xhandler.wasm` y `VERSION`
- `xhandler.schema.json`: JSON Schema de los modelos (`schemars`, feature `schema` de xhandler)
- `models/`: los tipos de los modelos para Go, Python y Java, generados por
  `xhandler-wasm/codegen.py`
- `conformance/`: los casos que toda librería debe pasar, `<op>/<nombre>.json` con `input` y
  `expected`

El corpus sale de los snapshots de xbuilder (cada test vuelca su entrada), las firmas exactas de
xsigner y los archivos de prueba de xsender. `make dist` falla si el código nativo no coincide con un
snapshot, y el CI lo ejecuta.

Cuando un cambio toca los modelos o el comportamiento de una operación:

1. Agrega o actualiza los tests de Rust y, si las librerías deben verlo, un caso de conformidad en
   `xhandler-wasm/examples/corpus.rs`.
2. Ejecuta `make -C xhandler-wasm dist`.
3. En cada librería ejecuta `make sync RUST_REPO=../xhandler-rust` y sus tests, y abre allí un PR
   con los archivos copiados.

## Convenciones

- Nombres, idioma (español para los conceptos de la SUNAT, inglés para el resto) y estilo están en
  [CLAUDE.md](CLAUDE.md).
- Commits y títulos de PR con [Conventional Commits](https://www.conventionalcommits.org/)
  (`feat:`, `fix:`, `refactor:`, `docs:`, `ci:`, `chore:`...); el CI revisa el título del PR. Los
  cambios incompatibles llevan `!` (`feat(xsender)!: ...`).
- Checks requeridos: `cargo-lint` y `cargo-test`. Los PRs de Dependabot se fusionan solos cuando
  pasan.

## Licencia

Al contribuir aceptas que tus contribuciones se distribuyan bajo la
[Apache License, Version 2.0](LICENSE).
