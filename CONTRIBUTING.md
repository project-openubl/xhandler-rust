# Contribuir a xhandler-rust

## Requisitos

- [Rust](https://rustup.rs): `rustup toolchain install` en el repositorio instala la versión de
  `rust-toolchain.toml` con el target `wasm32-wasip1`.
- libxml2 para los tests (validación XSD):
  - Fedora: `sudo dnf install pkg-config libxml2-devel clang-devel`
  - Ubuntu: `sudo apt-get install pkg-config libxml2-dev libclang-dev`
- Python 3 (opcional, para `make -C xhandler-wasm dist`).

O abre el repositorio en el [devcontainer](.devcontainer/README.md).

## Comandos

```shell
cargo build
XBUILDER_SKIP_SUNAT=true cargo test                 # sin enviar los documentos de xbuilder a la SUNAT beta
cargo test -p xhandler --test xbuilder invoice_     # tests de xbuilder que coinciden con un nombre
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
make -C xhandler-wasm dist                          # núcleo WebAssembly de las librerías de Go, Python y Java
```

`XBUILDER_SNAPSHOT_UPDATE=true cargo test -p xhandler --test xbuilder` reescribe los snapshots de
`xhandler/tests/resources/xbuilder/e2e/`: revisa el diff.

## Hacer un cambio

- El código está en `xhandler/src/` (`xbuilder`, `xsigner`, `xsender`), la CLI en `cli/` y el
  núcleo WebAssembly en `xhandler-wasm/`.
- Un cambio de comportamiento viene con tests. Si las librerías de Go, Python y Java deben verlo,
  agrega un caso en `xhandler-wasm/examples/corpus.rs`, ejecuta `make -C xhandler-wasm dist` y, en
  cada librería, `make sync RUST_REPO=../xhandler-rust`.
- Nombres y estilo: [CLAUDE.md](CLAUDE.md).

## Pull requests

- Título con [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`,
  `docs:`...); los cambios incompatibles llevan `!`.
- Deben pasar `cargo-lint` y `cargo-test`.

Al contribuir aceptas que tus contribuciones se distribuyan bajo la
[Apache License, Version 2.0](LICENSE).
