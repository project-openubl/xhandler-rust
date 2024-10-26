[![License](https://img.shields.io/github/license/project-openubl/xbuilder?logo=apache)](https://www.apache.org/licenses/LICENSE-2.0)
![CI](https://github.com/project-openubl/xbuilder/workflows/CI/badge.svg)

[![Project Chat](https://img.shields.io/badge/zulip-join_chat-brightgreen.svg?style=for-the-badge&logo=zulip)](https://projectopenubl.zulipchat.com/)

# XBuilder y XSender

- Crea XML basados en SUNAT y UBL
- Envia los XMLs a la SUNAT usando SOAP y REST

### Create Pull Requests

Create all Pull Requests using the prefixes of the following table:

| Category      | Prefix    |
|---------------|-----------|
| Features      | feat:     |
| Fixes         | fix:      |
| Changes       | perf:     |
| Changes       | refactor: |
| Changes       | revert:   |
| Changes       | style:    |
| Tasks         | chore:    |
| Build         | test:     |
| Build         | build:    |
| Build         | ci:       |
| Documentation | docs:     |

### Fedora

Instructions to setup:

```shell
sudo dnf groupinstall "Development Tools"
sudo dnf install libxml2-devel xmlsec1-devel pkg-config clang-devel
```

## License

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
