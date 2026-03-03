[![License](https://img.shields.io/github/license/project-openubl/xbuilder?logo=apache)](https://www.apache.org/licenses/LICENSE-2.0)
![CI](https://github.com/project-openubl/xbuilder/workflows/CI/badge.svg)

[![Project Chat](https://img.shields.io/badge/zulip-join_chat-brightgreen.svg?style=for-the-badge&logo=zulip)](https://projectopenubl.zulipchat.com/)

## Libraries

XMLs basados en UBL y SUNAT

- [x] Crear
- [x] Firmar
- [x] Enviar

## Server

```shell
cargo run --bin server
```

- The server is running at http://localhost:8080
- You can see Swagger UI at http://localhost:8080/swagger-ui

## Server UI

```shell
npm run dev --prefix server/ui
```

- The UI running at http://localhost:3000

## License

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
