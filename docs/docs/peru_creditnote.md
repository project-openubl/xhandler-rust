---
id: peru_creditnote
title: CreditNote (Nota de crédito)
---

## Create _CreditNoteType_

To create a `CreditNoteType` you only need to create an instance of `CreditNoteInputModel`.

```java
Config config;
SystemClock clock;

// Create the POJO
CreditNoteInputModel pojo = CreditNoteInputModel.Builder.aCreditNoteInputModel()
        .withSerie("FC01")
        .withNumero(1)
        .withSerieNumeroComprobanteAfectado("F001-1")
        .withDescripcionSustento("mi sustento")
        .withProveedor(ProveedorInputModel.Builder.aProveedorInputModel()
                .withRuc("12345678912")
                .withRazonSocial("Softgreen S.A.C.")
                .build()
        )
        .withCliente(ClienteInputModel.Builder.aClienteInputModel()
                .withNombre("Carlos Feria")
                .withNumeroDocumentoIdentidad("12121212121")
                .withTipoDocumentoIdentidad(Catalog6.RUC.toString())
                .build()
        )
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withDescripcion("Item1")
                        .withCantidad(new BigDecimal(10))
                        .withPrecioUnitario(new BigDecimal(100))
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withDescripcion("Item2")
                        .withCantidad(new BigDecimal(10))
                        .withPrecioUnitario(new BigDecimal(100))
                        .build())
        )
        .build();

// Create XML
DocumentWrapper<CreditNoteOutputModel> result = DocumentManager.createXML(pojo, config, systemClock);
CreditNoteOutputModel output = result.getOutput();
String xml = result.getXml();
```
