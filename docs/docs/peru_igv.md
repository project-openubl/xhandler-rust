---
id: peru_igv
title: IGV
---

_Inpuesto General a las Ventas (IGV)_ is a type of tax valid in Perú. Whenever you create a **_InvoiceType_, _CreditNoteType_, or _DebitNoteType_** you can define, for each item selled, a specific type of _IGV_. All _IGV_ types might be grouped into 4 groups:

- Gravadas
- Exoneradas
- Inafectas
- Gratuitas

## Examples

### _Invoice (boleta/factura)_

Use the field `tipoIgv` in each item selled:

```java
InvoiceInputModel pojo = InvoiceInputModel.Builder.anInvoiceInputModel()
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withTipoIgv(Catalog7.GRAVADO_OPERACION_ONEROSA.toString())
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withTipoIgv(Catalog7.GRAVADO_OPERACION_ONEROSA.toString())
                        .build())
        )
        .build();
```

### _CreditNote (nota de crédito)_

Use the field `tipoIgv` in each item selled:

```java
CreditNoteInputModel pojo = CreditNoteInputModel.Builder.aCreditNoteInputModel()
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withTipoIgv(Catalog7.GRAVADO_OPERACION_ONEROSA.toString())
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withTipoIgv(Catalog7.GRAVADO_OPERACION_ONEROSA.toString())
                        .build())
        )
        .build();
```

### _DebitNote (nota de débito)_

Use the field `tipoIgv` in each item selled:

```java
DebitNoteInputModel pojo = DebitNoteInputModel.Builder.aDebitNoteInputModel()
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withTipoIgv(Catalog7.GRAVADO_OPERACION_ONEROSA.toString())
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withTipoIgv(Catalog7.GRAVADO_OPERACION_ONEROSA.toString())
                        .build())
        )
        .build();
```

## Values of _tipoIgv_

The list of possible values of _tipoIgv_ can be found in the **Catalog 07** defined by the SUNAT.

| Value                                     | Code |
| ----------------------------------------- | ---- |
| GRAVADO_OPERACION_ONEROSA                 | 10   |
| GRAVADO_RETIRO_POR_PREMIO                 | 11   |
| GRAVADO_RETIRO_POR_DONACION               | 12   |
| GRAVADO_RETIRO                            | 13   |
| GRAVADO_RETIRO_POR_PUBLICIDAD             | 14   |
| GRAVADO_BONIFICACIONES                    | 15   |
| GRAVADO_RETIRO_POR_ENTREGA_A_TRABAJADORES | 16   |
| GRAVADO_IVAP                              | 17   |
| EXONERADO_OPERACION_ONEROSA               | 20   |
| EXONERADO_TRANSFERENCIA_GRATUITA          | 21   |
| INAFECTO_OPERACION_ONEROSA                | 30   |
| INAFECTO_RETIRO_POR_BONIFICACION          | 31   |
| INAFECTO_RETIRO                           | 32   |
| INAFECTO_RETIRO_POR_MUESTRAS_MEDICAS      | 33   |
| INAFECTO_RETIRO_POR_CONVENIO_COLECTIVO    | 34   |
| INAFECTO_RETIRO_POR_PREMIO                | 35   |
| INAFECTO_RETIRO_POR_PUBLICIDAD            | 36   |
| EXPORTACION                               | 40   |

## Default value of _tipoIgv_

If _tipoIgv_ is not defined within the POJO, then a default value defined in the **Config** will be applied. Learn how to configure the config in the [Concepts page](./concepts#config)
