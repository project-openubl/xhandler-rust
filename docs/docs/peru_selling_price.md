---
id: peru_selling_price
title: Selling price
---

Whenever an item is selled and registered through an **_InvoiceType_, _CreditNoteType_, or _DebitNoteType_** there are two different prices that must be included inside the XML file:

- Price before taxes
- Price after taxes

Besides the _pricing_ it is also required to have the _quantity_ of the item you are selling so we are able to have a formula like:

> QUANTITY \* PRICE BEFORE TAXES = SELLING PRICE

You only need 2 of the 3 variables defined in the formula above; the third variable will be calculated by `XBuilder`.

- **Case 1** - You have `Quantity` and `price before taxes`.
- **Case 2** - You have `Quantity` and `price after taxes`.

## Case 1 - Quantity and Price before taxes

You have `Quantity` and `price before taxes`. This is the recommended set of variables you need to use.

You need to use `cantidad` and `precioUnitario`.

### _Invoice (boleta/factura)_

```java
InvoiceInputModel pojo = InvoiceInputModel.Builder.anInvoiceInputModel()
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withCantidad(new BigDecimal(10))
                        .withPrecioUnitario(new BigDecimal(100))
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withCantidad(new BigDecimal(10))
                        .withPrecioUnitario(new BigDecimal(100))
                        .build())
        )
        .build();
```

### _CreditNote (nota de crédito)_

```java
CreditNoteInputModel pojo = CreditNoteInputModel.Builder.aCreditNoteInputModel()
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withCantidad(new BigDecimal(10))
                        .withPrecioUnitario(new BigDecimal(100))
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withCantidad(new BigDecimal(10))
                        .withPrecioUnitario(new BigDecimal(100))
                        .build())
        )
        .build();
```

### _DebitNote (nota de débito)_

```java
DebitNoteInputModel pojo = DebitNoteInputModel.Builder.aDebitNoteInputModel()
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withCantidad(new BigDecimal(10))
                        .withPrecioUnitario(new BigDecimal(100))
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withCantidad(new BigDecimal(10))
                        .withPrecioUnitario(new BigDecimal(100))
                        .build())
        )
        .build();
```

## Case 2 - Quantity and Price after taxes

You have `Quantity` and `price after taxes`.

It is not easy to determine the _price before taxes_ starting from the _price after taxes_ because every single tax is applied in a different way and using a different formula and values. Having said that, `XBuilder` is able to determine the _price before taxes_ starting from the _price with IGV_.

You need to use `cantidad` and `precioConIgv`.

### _Invoice (boleta/factura)_

```java
InvoiceInputModel pojo = InvoiceInputModel.Builder.anInvoiceInputModel()
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withCantidad(new BigDecimal(10))
                        .withPrecioConIgv(new BigDecimal(118))
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withCantidad(new BigDecimal(10))
                        .withPrecioConIgv(new BigDecimal(118))
                        .build())
        )
        .build();
```

### _CreditNote (nota de crédito)_

```java
CreditNoteInputModel pojo = CreditNoteInputModel.Builder.aCreditNoteInputModel()
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withCantidad(new BigDecimal(10))
                        .withPrecioConIgv(new BigDecimal(118))
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withCantidad(new BigDecimal(10))
                        .withPrecioConIgv(new BigDecimal(118))
                        .build())
        )
        .build();
```

### _DebitNote (nota de débito)_

```java
DebitNoteInputModel pojo = DebitNoteInputModel.Builder.aDebitNoteInputModel()
        .withDetalle(Arrays.asList(
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withCantidad(new BigDecimal(10))
                        .withPrecioConIgv(new BigDecimal(118))
                        .build(),
                DocumentLineInputModel.Builder.aDocumentLineInputModel()
                        .withCantidad(new BigDecimal(10))
                        .withPrecioConIgv(new BigDecimal(118))
                        .build())
        )
        .build();
```

## How the third variable is calculated

Regarding of the way you are following [Case 1](#case-1---quantity-and-price-before-taxes) or [Case 2](#case-2---quantity-and-price-after-taxes) you need to know that the thrid variable, calculated out of the box by `XBuilder`, is affected by the value of `igv` within [Config](./concepts#config)

When creating the `Config` instance you should define also the value of `igv`. The interface `Config` defines a method `getIgv` which should be implemented by you.

```java
public interface Config {
    BigDecimal getIgv();
    ...
}
```
