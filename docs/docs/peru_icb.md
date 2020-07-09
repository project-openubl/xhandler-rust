---
id: peru_icb
title: ICB
---

_Inpuesto al Consumo de Bolsas plásticas (ICB)_ is a type of tax applied only to plastic bags. Whenever you create an **_InvoiceType_, _CreditNoteType_, or _DebitNoteType_** you can define, for each item selled, if you want to apply the _ICB_ taxes or not.

The current value of _ICB_ is _0.2 Soles_ but this might change in the future. Don't worry you are safe if you use your own [Config](./concepts#config) default values.

## Examples

### _Invoice (boleta/factura)_

Use the field `icb` in each item selled:

```java
InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withIcb(true)
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withIcb(true)
                        .build())
        )
        .build();
```

### _CreditNote (nota de crédito)_

Use the field `icb` in each item selled:

```java
CreditNoteInputModel pojo = CreditNoteInputModel.Builder.aCreditNoteInputModel()
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withIcb(true)
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withIcb(true)
                        .build())
        )
        .build();
```

### _DebitNote (nota de débito)_

Use the field `icb` in each item selled:

```java
DebitNoteInputModel pojo = DebitNoteInputModel.Builder.aDebitNoteInputModel()
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withIcb(true)
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withIcb(true)
                        .build())
        )
        .build();
```

## Value of _ICB_

The value of _ICB_ is defined by the value you setted in the [Config](./concepts#config) object.

## Default value of _ICB_

If _icb_ is not defined or is `false`, then `XBuilder` won't apply this type of tax.
