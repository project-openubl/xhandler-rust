# Guia de elaboracion de documentos electronicos XML - UBL 2.1
## Boleta Electronica

**Basado en:** UBL 2.1
**Emisor:** SUNAT

---

## 1. Informacion General

The Boleta Electronica (Sales Receipt) is used for sales to final consumers (individuals). It uses the same Invoice UBL schema as Factura but with document type code `03` (BOLETA).

Key differences from Factura:
- Series starts with "B" (e.g., B001, BA01, BG01)
- Document type code is `03` (not `01`)
- Customer identification can use DNI, Carnet de Extranjeria, Pasaporte, or "-" for anonymous
- Sent to SUNAT via daily summary (Resumen Diario) not individually
- ProfileID uses Catalogo 17 (different operation types for boletas)

## 2. Estructura General del XML

Same XML namespaces and structure as Invoice. Root element is `<Invoice>`.

```xml
<?xml version="1.0" encoding="utf-8"?>
<Invoice xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2"
  xmlns:cac="urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"
  xmlns:cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2"
  xmlns:ext="urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2">
```

## 3. Datos del Encabezado

### 3.1 Version UBL (cbc:UBLVersionID)
Valor fijo: `2.1`

### 3.2 Version de la Estructura (cbc:CustomizationID)
Valor fijo: `2.0`

### 3.3 Tipo de Operacion (cbc:ProfileID)

Uses Catalogo No. 51 (via Catalogo 17 for Boleta):

| Codigo | Concepto | Descripcion |
|--------|----------|-------------|
| 0101 | Venta Interna | Venta en el pais de bienes muebles |
| 0102 | Exportacion | Venta de bienes a no domiciliados |
| 0103 | No Domiciliados | Ventas/servicios a no domiciliados en territorio nacional |
| 0104 | Venta Interna - Anticipos | Pagos realizados antes de la entrega |
| 0105 | Venta Itinerante | Bienes trasladados durante recorrido |
| 0106 | Factura Guia | Traslado con boleta en vez de guia de remision |
| 0107 | Venta Arroz Pilado | Operacion sujeta a IVAP |
| 0108 | Factura Comprobante de Percepcion | Cancelacion del integro del precio + percepcion |
| 0110 | Factura - Guia remitente | Traslado con boleta en vez de guia remitente |

Atributos:
- `schemeName`: "SUNAT:Identificador de Tipo de Operacion"
- `schemeAgencyName`: "PE:SUNAT"
- `schemeURI`: "urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo17"

### 3.4 Numeracion (cbc:ID)

Serie de 4 caracteres comenzando con "B" + guion + correlativo hasta 8 digitos.
Formato: `B###-NNNNNNNN`

```xml
<cbc:ID>BA12-16</cbc:ID>
```

### 3.5 Fecha de Emision (cbc:IssueDate)
Formato: `YYYY-MM-DD`

```xml
<cbc:IssueDate>2017-05-17</cbc:IssueDate>
```

### 3.6 Hora de Emision (cbc:IssueTime)
Formato: `HH:MM:SS`

```xml
<cbc:IssueTime>07:20:45</cbc:IssueTime>
```

### 3.7 Tipo de Documento (cbc:InvoiceTypeCode)
Valor: `03` (BOLETA)

Atributos:
- `listAgencyName`: "PE:SUNAT"
- `listName`: "SUNAT:Identificador de Tipo de Documento"
- `listURI`: "urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01"

```xml
<cbc:InvoiceTypeCode listAgencyName="PE:SUNAT"
  listName="SUNAT:Identificador de Tipo de Documento"
  listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01">03</cbc:InvoiceTypeCode>
```

### 3.8 Leyendas (cbc:Note)

Codigos del Catalogo No. 52:
| Codigo | Descripcion |
|--------|------------|
| 1000 | Monto expresado en letras |
| 1002 | "Transferencia gratuita" o "Servicio prestado gratuitamente" |
| 2000 | "Comprobante de Percepcion" |
| 2001 | "Bienes transferidos en la Amazonia" |
| 2002 | "Servicios prestados en la Amazonia" |
| 2003 | "Contratos de construccion ejecutados en la Amazonia" |
| 2004 | "Agencia de Viaje - Paquete turistico" |
| 2005 | "Venta realizada por emisor itinerante" |
| 2006 | "Operacion sujeta a detraccion" / Codigo interno de software |
| 3000 | Codigo interno generado por el software de Facturacion |

```xml
<cbc:Note languageLocaleID="1000">MIL OCHOCIENTOS CINCUENTA Y OCHO CON 59/100 Soles</cbc:Note>
<cbc:Note languageLocaleID="3000">05010020170428000005</cbc:Note>
```

### 3.9 Tipo de Moneda (cbc:DocumentCurrencyCode)
ISO 4217. Valor comun: `PEN` (Sol)

```xml
<cbc:DocumentCurrencyCode listID="ISO 4217 Alpha" listName="Currency"
  listAgencyName="United Nations Economic Commission for Europe">PEN</cbc:DocumentCurrencyCode>
```

---

## 4. Guia de Remision Relacionada

Referencia a guias de remision remitente o transportista.

```xml
<cac:DespatchDocumentReference>
  <cbc:ID>0001-002020</cbc:ID>
  <cbc:DocumentTypeCode listAgencyName="PE:SUNAT"
    listName="SUNAT:Identificador de guia relacionada"
    listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01">09</cbc:DocumentTypeCode>
</cac:DespatchDocumentReference>
```

---

## 5. Otros Documentos Relacionados

Catalogo No. 12:
| Codigo | Descripcion |
|--------|------------|
| 03 | Boleta de Venta - emitida por anticipos |
| 04 | Ticket de Salida - ENAPU |
| 05 | Codigo SCOP |
| 99 | Otros |

```xml
<cac:AdditionalDocumentReference>
  <cbc:ID>024099</cbc:ID>
  <cbc:DocumentTypeCode listAgencyName="PE:SUNAT"
    listName="SUNAT:Identificador de documento relacionado"
    listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo12">99</cbc:DocumentTypeCode>
</cac:AdditionalDocumentReference>
```

---

## 6. Firma Digital

Same structure as Factura.

```xml
<cac:Signature>
  <cbc:ID>IDSignST</cbc:ID>
  <cac:SignatoryParty>
    <cac:PartyIdentification>
      <cbc:ID>20100066603</cbc:ID>
    </cac:PartyIdentification>
    <cac:PartyName>
      <cbc:Name>SUNAT</cbc:Name>
    </cac:PartyName>
  </cac:SignatoryParty>
  <cac:DigitalSignatureAttachment>
    <cac:ExternalReference>
      <cbc:URI>#SignatureSP</cbc:URI>
    </cac:ExternalReference>
  </cac:DigitalSignatureAttachment>
</cac:Signature>
```

---

## 7. Datos del Emisor (AccountingSupplierParty)

Uses `cac:PartyTaxScheme` structure (different from Factura's `cac:PartyLegalEntity`).

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cbc:Name` (PartyName) | Nombre comercial | Condicional |
| `cbc:RegistrationName` (PartyTaxScheme) | Razon social | Si |
| `cbc:CompanyID` (PartyTaxScheme) | RUC del emisor (11 digitos) | Si |
| `cbc:AddressTypeCode` | Codigo de establecimiento | Condicional |

CompanyID atributos:
- `schemeID`: "6"
- `schemeName`: "SUNAT:Identificador de Documento de Identidad"
- `schemeAgencyName`: "PE:SUNAT"
- `schemeURI`: "urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06"

```xml
<cac:AccountingSupplierParty>
  <cac:Party>
    <cac:PartyName>
      <cbc:Name><![CDATA[K&G Laboratorios]]></cbc:Name>
    </cac:PartyName>
    <cac:PartyTaxScheme>
      <cbc:RegistrationName><![CDATA[K&G Asociados S. A.]]></cbc:RegistrationName>
      <cbc:CompanyID schemeID="6" schemeName="SUNAT:Identificador de Documento de Identidad"
        schemeAgencyName="PE:SUNAT"
        schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06">46237547</cbc:CompanyID>
      <cac:RegistrationAddress>
        <cbc:AddressTypeCode>0014</cbc:AddressTypeCode>
      </cac:RegistrationAddress>
      <cac:TaxScheme>
        <cbc:ID>-</cbc:ID>
      </cac:TaxScheme>
    </cac:PartyTaxScheme>
  </cac:Party>
</cac:AccountingSupplierParty>
```

---

## 8. Datos del Receptor (AccountingCustomerParty)

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cbc:RegistrationName` | Apellidos y nombres o razon social | Si |
| `cbc:CompanyID` | Numero de documento de identidad | Si |
| `cbc:CompanyID/@schemeID` | Tipo de documento (Catalogo No. 06) | Si |

Tipos de documento de identidad (Catalogo No. 06):
| Codigo | Concepto |
|--------|----------|
| 0 | DOC.TRIB.NO.DOM.SIN.RUC |
| 1 | DOC. NACIONAL DE IDENTIDAD (DNI) |
| 4 | CARNET DE EXTRANJERIA |
| 6 | REG. UNICO DE CONTRIBUYENTES (RUC) |
| 7 | PASAPORTE |
| A | CED. DIPLOMATICA DE IDENTIDAD |
| B | DOC.IDENT.PAIS.RESIDENCIA-NO.D |
| C | Tax Identification Number - TIN |
| D | Identification Number - IN |

```xml
<cac:AccountingCustomerParty>
  <cac:Party>
    <cac:PartyTaxScheme>
      <cbc:RegistrationName><![CDATA[PAZOS ATOCHE LUANA]]></cbc:RegistrationName>
      <cbc:CompanyID schemeID="6" schemeName="SUNAT:Identificador de Documento de Identidad"
        schemeAgencyName="PE:SUNAT" schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06">46237547</cbc:CompanyID>
      <cac:TaxScheme>
        <cbc:ID>-</cbc:ID>
      </cac:TaxScheme>
    </cac:PartyTaxScheme>
  </cac:Party>
</cac:AccountingCustomerParty>
```

---

## 9. Descuento Global

| Campo | Descripcion |
|-------|------------|
| `cbc:ChargeIndicator` | `false` para descuento |
| `cbc:AllowanceChargeReasonCode` | Codigo 00 (Catalogo No. 53) |
| `cbc:MultiplierFactorNumeric` | Porcentaje del descuento (ej: 0.10 = 10%) |
| `cbc:Amount` | Monto del descuento |
| `cbc:BaseAmount` | Monto base sobre el cual se aplica |

```xml
<cac:AllowanceCharge>
  <cbc:ChargeIndicator>False</cbc:ChargeIndicator>
  <cbc:AllowanceChargeReasonCode>00</cbc:AllowanceChargeReasonCode>
  <cbc:MultiplierFactorNumeric>0.10</cbc:MultiplierFactorNumeric>
  <cbc:Amount currencyID="PEN">60.00</cbc:Amount>
  <cbc:BaseAmount currencyID="PEN">1439.48</cbc:BaseAmount>
</cac:AllowanceCharge>
```

---

## 10. Totales de Impuestos (TaxTotal)

### 10.1 Monto Total de Impuestos

```xml
<cac:TaxTotal>
  <cbc:TaxAmount currencyID="PEN">59210.65</cbc:TaxAmount>
  ...
</cac:TaxTotal>
```

### 10.2 Sumatoria ISC

TaxScheme: ID=2000, Name=ISC, TaxTypeCode=EXC
TaxCategory ID: S

```xml
<cac:TaxSubtotal>
  <cbc:TaxableAmount currencyID="PEN">6450.00</cbc:TaxableAmount>
  <cbc:TaxAmount currencyID="PEN">1096.50</cbc:TaxAmount>
  <cac:TaxCategory>
    <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
      schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
    <cac:TaxScheme>
      <cbc:ID schemeID="UN/ECE 5153" schemeAgencyID="6">2000</cbc:ID>
      <cbc:Name>ISC</cbc:Name>
      <cbc:TaxTypeCode>EXC</cbc:TaxTypeCode>
    </cac:TaxScheme>
  </cac:TaxCategory>
</cac:TaxSubtotal>
```

### 10.3 Sumatoria IGV

IGV = 18% de: [Total valor de venta operaciones gravadas] + [Sumatoria ISC]
TaxScheme: ID=1000, Name=IGV, TaxTypeCode=VAT
TaxCategory ID: S

```xml
<cac:TaxSubtotal>
  <cbc:TaxableAmount currencyID="PEN">8560.00</cbc:TaxableAmount>
  <cbc:TaxAmount currencyID="PEN">1540.80</cbc:TaxAmount>
  <cac:TaxCategory>
    <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
      schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
    <cac:TaxScheme>
      <cbc:ID schemeID="UN/ECE 5153" schemeAgencyID="6">1000</cbc:ID>
      <cbc:Name>IGV</cbc:Name>
      <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>
    </cac:TaxScheme>
  </cac:TaxCategory>
</cac:TaxSubtotal>
```

### 10.4 Total Valor de Venta - Operaciones Inafectas

TaxScheme: ID=9998, Name=INAFECTO, TaxTypeCode=FRE
TaxCategory ID: O
TaxAmount = 0.00

### 10.5 Total Valor de Venta - Operaciones Exoneradas

TaxScheme: ID=9997, Name=EXONERADO, TaxTypeCode=VAT
TaxCategory ID: E
TaxAmount = 0.00

### 10.6 Total Valor de Venta - Operaciones Gratuitas

TaxScheme: ID=9996, Name=GRATUITO, TaxTypeCode=FRE
TaxCategory ID: Z

### 10.7 Sumatoria Otros Tributos

TaxScheme: ID=9999, Name=OTROS CONCEPTOS DE PAGO, TaxTypeCode=OTH
TaxCategory ID: S

---

## 11. Totales Monetarios (LegalMonetaryTotal)

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cbc:LineExtensionAmount` | Total Valor de Venta (sin impuestos, sin descuentos globales) | Si |
| `cbc:TaxInclusiveAmount` | Total Precio de Venta (con impuestos) | Si |
| `cbc:AllowanceTotalAmount` | Total de Descuentos | Condicional |
| `cbc:ChargeTotalAmount` | Sumatoria otros Cargos (propinas, etc.) | Condicional |
| `cbc:PayableAmount` | Importe total de la venta | Si |

```xml
<cac:LegalMonetaryTotal>
  <cbc:LineExtensionAmount currencyID="PEN">1439.48</cbc:LineExtensionAmount>
  <cbc:TaxInclusiveAmount currencyID="PEN">9420.50</cbc:TaxInclusiveAmount>
  <cbc:AllowanceTotalAmount currencyID="PEN">9420.50</cbc:AllowanceTotalAmount>
  <cbc:ChargeTotalAmount currencyID="PEN">9420.50</cbc:ChargeTotalAmount>
  <cbc:PayableAmount currencyID="PEN">45.34</cbc:PayableAmount>
</cac:LegalMonetaryTotal>
```

---

## 12. Detalle de Items (InvoiceLine)

### 12.1 Numero de Orden del Item (cbc:ID)

```xml
<cac:InvoiceLine>
  <cbc:ID>1</cbc:ID>
  ...
</cac:InvoiceLine>
```

### 12.2 Cantidad de Unidades (cbc:InvoicedQuantity)

Catalogo No. 03:
| Codigo | Descripcion |
|--------|------------|
| NIU | UNIDAD (BIENES) |
| ZZ | UNIDAD (SERVICIOS) |

```xml
<cbc:InvoicedQuantity unitCode="CS" unitCodeListID="UN/ECE rec 20"
  unitCodeListAgencyName="United Nations Economic Commission for Europe">50</cbc:InvoicedQuantity>
```

### 12.3 Valor de Venta por Item (cbc:LineExtensionAmount)

Producto de la cantidad por el valor unitario menos descuentos. No incluye IGV, ISC ni otros tributos.

```xml
<cbc:LineExtensionAmount currencyID="PEN">172890.0</cbc:LineExtensionAmount>
```

### 12.4 Precio de Venta Unitario (PricingReference)

Catalogo No. 16:
| Codigo | Descripcion |
|--------|------------|
| 01 | Precio unitario (incluye el IGV) |
| 02 | Valor referencial unitario en operaciones no onerosas |

```xml
<cac:PricingReference>
  <cac:AlternativeConditionPrice>
    <cbc:PriceAmount currencyID="PEN">18.75</cbc:PriceAmount>
    <cbc:PriceTypeCode listName="SUNAT:Indicador de Tipo de Precio"
      listAgencyName="PE:SUNAT"
      listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo16">01</cbc:PriceTypeCode>
  </cac:AlternativeConditionPrice>
</cac:PricingReference>
```

### 12.5 Descuentos por Item

```xml
<cac:AllowanceCharge>
  <cbc:ChargeIndicator>false</cbc:ChargeIndicator>
  <cbc:Amount currencyID="PEN">71.97</cbc:Amount>
</cac:AllowanceCharge>
```

### 12.6 Cargos por Item

```xml
<cac:AllowanceCharge>
  <cbc:ChargeIndicator>true</cbc:ChargeIndicator>
  <cbc:Amount currencyID="PEN">44.82</cbc:Amount>
</cac:AllowanceCharge>
```

### 12.7 Afectacion al IGV por Item (TaxTotal)

Uses Catalogo No. 07 for TaxExemptionReasonCode.

Afectacion types for Boleta:

| Afectacion | Descripcion |
|------------|-------------|
| Gravado - Operacion Onerosa (10) | Operacion dentro del ambito del IGV |
| Gravado - Premio (11) | Transferencia de bienes a terceros por sorteos/concursos |
| Gravado - Donacion (12) | Transferencia altruista |
| Gravado - Retiro (13) | Retiros de bienes obligados a emitir comprobante |
| Gravado - Publicidad (14) | Bienes producidos por el transferente para promocion |
| Gravado - Bonificaciones (15) | Bienes relacionados con compras |
| Gravado - Entrega a trabajadores (16) | Bienes de libre disposicion a trabajadores |
| Exonerado - Operacion Onerosa (20) | Dentro del ambito del impuesto pero excluido |
| Inafecto - Operacion Onerosa (30) | Fuera del ambito del IGV |
| Inafecto - Premio (35) | Bienes no producidos por el transferente |
| Inafecto - Publicidad (36) | Bienes por entregas de muestras |
| Inafecto - Bonificacion (31) | Bienes relacionados con compras |
| Inafecto - Retiro (32) | Retiros no sujetos a IGV |
| Inafecto - Muestras Medicas (33) | Muestras medicas gratuitas |
| Inafecto - Retiro por Convenio Colectivo (34) | Bienes de condicion de trabajo |
| Exportacion (40) | Operaciones fuera del territorio nacional |

```xml
<cac:TaxTotal>
  <cbc:TaxAmount currencyID="PEN">259.11</cbc:TaxAmount>
  <cac:TaxSubtotal>
    <cbc:TaxableAmount currencyID="PEN">1439.48</cbc:TaxableAmount>
    <cbc:TaxAmount currencyID="PEN">259.11</cbc:TaxAmount>
    <cac:TaxCategory>
      <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
        schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
      <cbc:Percent>18.00</cbc:Percent>
      <cbc:TaxExemptionReasonCode listAgencyName="PE:SUNAT"
        listName="SUNAT:Codigo de Tipo de Afectacion del IGV"
        listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo07">10</cbc:TaxExemptionReasonCode>
      <cac:TaxScheme>
        <cbc:ID>1000</cbc:ID>
        <cbc:Name>IGV</cbc:Name>
        <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>
      </cac:TaxScheme>
    </cac:TaxCategory>
  </cac:TaxSubtotal>
</cac:TaxTotal>
```

### 12.8 Sistema de ISC por Item (cbc:TierRange)

Catalogo No. 08:
| Codigo | Descripcion |
|--------|------------|
| 01 | Sistema al valor (Apendice IV, lit. A) |
| 02 | Aplicacion del Monto Fijo (Apendice IV, lit. B) |
| 03 | Sistema de Precios de Venta al Publico (Apendice IV, lit. C) |

```xml
<cac:TaxTotal>
  <cbc:TaxAmount currencyID="PEN">1750.52</cbc:TaxAmount>
  <cac:TaxSubtotal>
    <cbc:TaxableAmount currencyID="PEN">8752.60</cbc:TaxableAmount>
    <cbc:TaxAmount currencyID="PEN">1750.52</cbc:TaxAmount>
    <cac:TaxCategory>
      <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
        schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
      <cbc:Percent>20.00</cbc:Percent>
      <cbc:TaxExemptionReasonCode listAgencyName="PE:SUNAT"
        listName="SUNAT:Codigo de Tipo de Afectacion del IGV"
        listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo07">10</cbc:TaxExemptionReasonCode>
      <cbc:TierRange>01</cbc:TierRange>
      <cac:TaxScheme>
        <cbc:ID>2000</cbc:ID>
        <cbc:Name>ISC</cbc:Name>
        <cbc:TaxTypeCode>EXC</cbc:TaxTypeCode>
      </cac:TaxScheme>
    </cac:TaxCategory>
  </cac:TaxSubtotal>
</cac:TaxTotal>
```

**IMPORTANTE:** La boleta electronica debera tener informacion de por lo menos uno de los siguientes campos: Total valor de venta - operaciones gravadas, Total valor de venta - operaciones inafectas, o Total valor de venta - operaciones exoneradas.

### 12.9 Descripcion Detallada (cac:Item/cbc:Description)

```xml
<cac:Item>
  <cbc:Description><![CDATA[CAPTOPRIL 25mg X 30]]></cbc:Description>
</cac:Item>
```

### 12.10 Codigo Producto del Item (SellersItemIdentification)

```xml
<cac:Item>
  <cac:SellersItemIdentification>
    <cbc:ID>Cap-258963</cbc:ID>
  </cac:SellersItemIdentification>
</cac:Item>
```

### 12.11 Codigo Producto SUNAT (CommodityClassification)

UNSPSC v14_0801 (Catalogo No. 25)

```xml
<cac:CommodityClassification>
  <cbc:ItemClassificationCode listID="UNSPSC" listAgencyName="GS1 US"
    listName="Item Classification">51121703</cbc:ItemClassificationCode>
</cac:CommodityClassification>
```

### 12.12 Valor Unitario del Item (cac:Price/cbc:PriceAmount)

Valor unitario sin impuestos.

```xml
<cac:Price>
  <cbc:PriceAmount currencyID="PEN">678.0</cbc:PriceAmount>
</cac:Price>
```

---

## 13. Ejemplo de Caso - Boleta Gravada con Descuento y Bonificacion

Empresa "Electrodomesticos Cruz de Motupe" de Carlos Enrique Vega Poblete
RUC: 10200545523
Boleta N BC01-3652

Fecha de Emision: 24 de junio del 2017
Adquiriente: Luana Karina Pazos Atoche
DNI: 46237547

Bienes vendidos:
| Codigo | Codigo SUNAT | Unidad | Cantidad | Descripcion | Afectacion | Precio Unitario |
|--------|-------------|--------|----------|-------------|------------|----------------|
| REF564 | 52141501 | Unidad | 1 | Refrigeradora marca "AXM" no frost de 200 ltrs | Gravado | 998.00 |
| COC124 | 95141606 | Unidad | 1 | Cocina a gas GLP, marca "AXM" de 5 hornillas | Gravado | 750.00 |

Informacion adicional:
- Precios en moneda nacional (PEN)
- Descuento de 5% por ser cliente frecuente
- Regalo de 10 sixpack de gaseosa "Guerene" de 400 ml (codigo BON012) con valor de venta total de S/.48.00 (operacion gratuita)

---

## Anexo - Resumen de Catalogos Referenciados

| Catalogo | Descripcion |
|----------|------------|
| 01 | Tipo de Documento |
| 02 | Tipo de Moneda (ISO 4217) |
| 03 | Tipo de Unidad de Medida (UN/ECE rec 20) |
| 05 | Codigos de Tipos de Tributos |
| 06 | Tipo de Documento de Identidad |
| 07 | Tipo de Afectacion del IGV |
| 08 | Tipo de Sistema de Calculo del ISC |
| 12 | Tipo de Documento Relacionado |
| 16 | Tipo de Precio de Venta Unitario |
| 17 | Tipo de Operacion (para Boleta) |
| 25 | Codigo de Producto SUNAT (UNSPSC) |
| 52 | Codigos de Leyendas |
| 53 | Codigos de Cargos o Descuentos |

---

## Anexo B - Firma Digital (XMLDSig)

La firma digital usa el estandar XMLDSig y se aloja en `ext:UBLExtension/ext:ExtensionContent`.

Estructura:
- **ds:Signature** - Contiene SignedInfo, SignatureValue, KeyInfo
  - **ds:SignedInfo** - CanonicalizationMethod, SignatureMethod, Reference
    - **ds:CanonicalizationMethod** - Algorithm para transformar a forma canonica
    - **ds:SignatureMethod** - Algoritmo de firma (RSA-SHA1)
    - **ds:Reference** - Hash del documento (URI=""), Transforms, DigestMethod, DigestValue
  - **ds:SignatureValue** - Firma codificada en Base64
  - **ds:KeyInfo** - Informacion del certificado firmante
    - **ds:X509Data** - Certificado X509
    - **ds:KeyValue** - Clave publica

Se utiliza la clave privada de un certificado digital X509 valido no vencido. Se firma todo el documento (nodo raiz).
