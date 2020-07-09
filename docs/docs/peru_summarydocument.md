---
id: peru_summarydocument
title: SummaryDocument (Resumen diario)
---

## Create _SummaryDocumentType_

To create a SummaryDocument you only need to create an instance of `SummaryDocumentInputModel`.

```java
Config config;
SystemClock clock;

// Create the POJO
Calendar calendar = Calendar.getInstance();
calendar.set(2019, Calendar.DECEMBER, 1, 20, 30, 59);

SummaryDocumentInputModel pojo = SummaryDocumentInputModel.Builder.aSummaryDocumentInputModel()
        .withNumero(1)
        .withFechaEmisionDeComprobantesAsociados(calendar.getTimeInMillis())
        .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
                .withRuc("12345678912")
                .withRazonSocial("Softgreen S.A.C.")
                .build()
        )
        .withDetalle(Collections.singletonList(
                SummaryDocumentLineInputModel.Builder.aSummaryDocumentLineInputModel()
                        .withTipoOperacion(Catalog19.ADICIONAR.toString())
                        .withComprobante(SummaryDocumentComprobanteInputModel.Builder.aSummaryDocumentComprobanteInputModel()
                                .withTipo(Catalog1.BOLETA.toString())
                                .withSerieNumero("B001-1")
                                .withCliente(ClienteInputModel.Builder.aClienteInputModel()
                                        .withNombre("Carlos Feria")
                                        .withNumeroDocumentoIdentidad("12345678")
                                        .withTipoDocumentoIdentidad(Catalog6.DNI.toString())
                                        .build()
                                )
                                .withImpuestos(SummaryDocumentImpuestosInputModel.Builder.aSummaryDocumentImpuestosInputModel()
                                        .withIgv(new BigDecimal("100"))
                                        .build()
                                )
                                .withValorVenta(SummaryDocumentComprobanteValorVentaInputModel.Builder.aSummaryDocumentComprobanteValorVentaInputModel()
                                        .withImporteTotal(new BigDecimal("118"))
                                        .withGravado(new BigDecimal("100"))
                                        .build()
                                )
                                .build()
                        )
                        .build()
        ))
        .build();

// When
DocumentWrapper<SummaryDocumentOutputModel> result = DocumentManager.createXML(pojo, config, systemClock);
SummaryDocumentOutputModel output = result.getOutput();
String xml = result.getXml();
```

## How the _documentID_ is calculated

The _documentID_ has the structure:

- _code-date-number_. E.g. `RC-20190101-1`

`XBuilder` will create the correct _documentID_ for you, you only need to define the `number` of _SummaryDocuments_ created during the day.
