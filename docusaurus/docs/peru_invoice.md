---
id: peru_invoice
title: Invoice (Boleta/Factura)
---

## Create *InvoiceType*

To create an invoice you only need to create an instance of `InvoiceInputModel`.

```java
Config config;
SystemClock clock;

// Create the POJO
InvoiceInputModel input = InvoiceInputModel.Builder.anInvoiceInputModel()
    .withSerie("F001")
    .withNumero(1)
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

// Create Invoice
DocumentWrapper<InvoiceOutputModel> result = DocumentManager.createXML(input, config, systemClock);

InvoiceOutputModel output = result.getOutput(); // XML Var values
String xml = result.getXml(); // XML content
```
