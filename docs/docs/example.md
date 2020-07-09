---
id: example
title: Example
---

Every time you want to create a XML file based on UBL you need to have 3 things:

- **Config** - Default values to apply in case those values are not defined by the user.
- **System clock** - A clock which should give the current system time and the time zone.
- **POJO**- The POJO which contains the data to be used to create the XML file.

## Create invoice

```java
// Create the Config
Config config = new DefaultConfig();

// Create the SystemClock
SystemClock clock = new SystemClock() {
    @Override
    public TimeZone getTimeZone() {
        return TimeZone.getTimeZone("America/Lima");
    }

    @Override
    public Calendar getCalendarInstance() {
        return Calendar.getInstance();
    }
};

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
