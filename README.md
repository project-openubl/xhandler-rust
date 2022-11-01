[![License](https://img.shields.io/github/license/project-openubl/xbuilder?logo=apache)](https://www.apache.org/licenses/LICENSE-2.0)
![CI](https://github.com/project-openubl/xbuilder/workflows/CI/badge.svg)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=project-openubl_xbuilder&metric=alert_status)](https://sonarcloud.io/dashboard?id=project-openubl_xbuilder)

[![Project Chat](https://img.shields.io/badge/zulip-join_chat-brightgreen.svg?style=for-the-badge&logo=zulip)](https://projectopenubl.zulipchat.com/)
[![Supported JVM Versions](https://img.shields.io/badge/JVM-11--17-brightgreen.svg?style=for-the-badge&logo=Java)](https://github.com/project-openubl/xbuilder/actions/runs/472762588/)

### XBuilder
[![Maven Central](https://img.shields.io/maven-central/v/io.github.project-openubl/xbuilder)](https://search.maven.org/artifact/io.github.project-openubl/xbuilder/)

### Quarkus XBuilder
[![Maven Central](https://img.shields.io/maven-central/v/io.github.project-openubl/quarkus-xbuilder)](https://search.maven.org/artifact/io.github.project-openubl/quarkus-xbuilder/)

# XBuilder

Librería Java para crear XMLs basados en UBL y los estándares de la SUNAT respecto a la facturación electrónica.

XBuilder esta diseñado para que puedas crear XMLs fácilmente.

- Crea XMLs sin que necesites conocer nada sobre manejo de archivos XMLs.
- Hace cálculos internos por ti.
- Requiere solamente datos mínimos.

## ¿Qué puedes hacer con XBuilder?

- Crear XMLs
- Firmar XMLs

## Getting started

- [Documentación](https://project-openubl.github.io)
- [Discusiones](https://github.com/project-openubl/xsender/discussions)

## Developers

### Update snapshots

```shell
mvn clean test -Dxbuilder.snapshot.update
```