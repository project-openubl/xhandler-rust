# Guia de elaboracion de documentos electronicos XML - UBL 2.1
## Nota de Credito Electronica

**Basado en:** UBL 2.1
**Emisor:** SUNAT

---

## 1. Informacion General

### 1.1 Descripcion Funcional

La Nota de Credito Electronica es un documento que se emite para disminuir (anular/modificar) el valor de una Factura o Boleta emitida con anterioridad. Se utiliza para:
- Anulacion parcial o total de comprobantes
- Correcciones de precios
- Descuentos posteriores a la emision
- Devolucion de bienes

Se basa en el estandar UBL 2.1 usando el elemento raiz `<CreditNote>`.

### 1.2 Condiciones

- Solo se puede emitir una Nota de Credito para modificar un comprobante de pago que haya sido previamente enviado a SUNAT.
- La Nota de Credito debe referenciar el documento original.

---

## 2. Estructura General del XML

### 2.1 Namespaces

```xml
<?xml version="1.0" encoding="utf-8"?>
<CreditNote xmlns="urn:oasis:names:specification:ubl:schema:xsd:CreditNote-2"
  xmlns:cac="urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"
  xmlns:cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2"
  xmlns:ccts="urn:un:unece:uncefact:documentation:2"
  xmlns:ds="http://www.w3.org/2000/09/xmldsig#"
  xmlns:ext="urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2"
  xmlns:qdt="urn:oasis:names:specification:ubl:schema:xsd:QualifiedDatatypes-2"
  xmlns:udt="urn:un:unece:uncefact:data:specification:UnqualifiedDataTypesSchemaModule:2">
```

### 2.2 Extension de Firma Digital

```xml
<ext:UBLExtensions>
  <ext:UBLExtension>
    <ext:ExtensionContent>
      <!-- Firma digital ds:Signature -->
    </ext:ExtensionContent>
  </ext:UBLExtension>
</ext:UBLExtensions>
```

---

## 3. Datos del Encabezado

### 3.1 Version UBL (cbc:UBLVersionID)

Valor fijo: `2.1`

```xml
<cbc:UBLVersionID>2.1</cbc:UBLVersionID>
```

### 3.2 Version de la Estructura (cbc:CustomizationID)

Valor fijo: `2.0`

```xml
<cbc:CustomizationID schemeAgencyName="PE:SUNAT">2.0</cbc:CustomizationID>
```

### 3.3 Numeracion (cbc:ID)

Serie de 4 caracteres + guion + correlativo hasta 8 digitos.
- Para Notas de Credito asociadas a Facturas: Serie inicia con "F" (ej. FC01, FF01)
- Para Notas de Credito asociadas a Boletas: Serie inicia con "B" (ej. BC01, BB01)

```xml
<cbc:ID>FF01-1</cbc:ID>
```

### 3.4 Fecha de Emision (cbc:IssueDate)

Formato: `YYYY-MM-DD`

```xml
<cbc:IssueDate>2017-07-23</cbc:IssueDate>
```

### 3.5 Hora de Emision (cbc:IssueTime)

Formato: `HH:MM:SS`

```xml
<cbc:IssueTime>12:35:00</cbc:IssueTime>
```

### 3.6 Leyendas (cbc:Note)

Catalogo No. 52:
| Codigo | Descripcion |
|--------|------------|
| 1000 | Monto en letras |
| 1002 | TRANSFERENCIA GRATUITA |
| 2001-2010 | Leyendas especificas segun tipo de operacion |

```xml
<cbc:Note languageLocaleID="1000"><![CDATA[SON MIL QUINIENTOS Y 00/100 SOLES]]></cbc:Note>
```

### 3.7 Tipo de Moneda (cbc:DocumentCurrencyCode)

ISO 4217.

```xml
<cbc:DocumentCurrencyCode listID="ISO 4217 Alpha" listName="Currency"
  listAgencyName="United Nations Economic Commission for Europe">PEN</cbc:DocumentCurrencyCode>
```

---

## 4. Documento que Modifica (DiscrepancyResponse y BillingReference)

### 4.1 Motivo de la Nota (cac:DiscrepancyResponse)

| Campo | Descripcion | Tipo | Obligatorio |
|-------|------------|------|-------------|
| `cbc:ReferenceID` | Numero de documento que modifica | an | Si |
| `cbc:ResponseCode` | Tipo de nota de credito (Catalogo No. 09) | an | Si |
| `cbc:Description` | Motivo o sustento | an | Si |

Catalogo No. 09 - Codigos de Tipo de Nota de Credito:
| Codigo | Descripcion |
|--------|------------|
| 01 | Anulacion de la operacion |
| 02 | Anulacion por error en el RUC |
| 03 | Correccion por error en la descripcion |
| 04 | Descuento global |
| 05 | Descuento por item |
| 06 | Devolucion total |
| 07 | Devolucion por item |
| 08 | Bonificacion |
| 09 | Disminucion en el valor |
| 10 | Otros conceptos |
| 11 | Ajustes de operaciones de exportacion |
| 12 | Ajustes afectos al IVAP |
| 13 | Ajustes - Loss or Reduction of Deducible Expenses / deductions for persons with disabilities |

```xml
<cac:DiscrepancyResponse>
  <cbc:ReferenceID>F001-111</cbc:ReferenceID>
  <cbc:ResponseCode listAgencyName="PE:SUNAT"
    listName="Tipo de nota de credito"
    listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo09">01</cbc:ResponseCode>
  <cbc:Description>Anulacion del comprobante</cbc:Description>
</cac:DiscrepancyResponse>
```

### 4.2 Referencia del Documento (cac:BillingReference)

| Campo | Descripcion | Tipo | Obligatorio |
|-------|------------|------|-------------|
| `cac:InvoiceDocumentReference/cbc:ID` | Numero del documento que se modifica | an | Si |
| `cac:InvoiceDocumentReference/cbc:DocumentTypeCode` | Tipo de documento (01=Factura, 03=Boleta) | an | Si |

```xml
<cac:BillingReference>
  <cac:InvoiceDocumentReference>
    <cbc:ID>F001-111</cbc:ID>
    <cbc:DocumentTypeCode listAgencyName="PE:SUNAT"
      listName="Tipo de Documento"
      listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01">01</cbc:DocumentTypeCode>
  </cac:InvoiceDocumentReference>
</cac:BillingReference>
```

---

## 5. Guia de Remision Relacionada

```xml
<cac:DespatchDocumentReference>
  <cbc:ID>T001-1</cbc:ID>
  <cbc:DocumentTypeCode listAgencyName="PE:SUNAT"
    listName="Tipo de Documento"
    listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01">09</cbc:DocumentTypeCode>
</cac:DespatchDocumentReference>
```

---

## 6. Otros Documentos Relacionados

Catalogo No. 12:
| Codigo | Descripcion |
|--------|------------|
| 01 | Numeracion DAM |
| 04 | Numero de Resolucion de Intendencia |
| 05 | BVME Domiciliados |
| 06 | Numero de autorizacion emitida por ente regulador |
| 99 | Otros |

```xml
<cac:AdditionalDocumentReference>
  <cbc:ID>118-2017-000025</cbc:ID>
  <cbc:DocumentTypeCode listAgencyName="PE:SUNAT"
    listName="Documento Relacionado"
    listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo12">01</cbc:DocumentTypeCode>
</cac:AdditionalDocumentReference>
```

---

## 7. Firma Digital

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

## 8. Datos del Emisor (AccountingSupplierParty)

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cac:PartyIdentification/cbc:ID` | Numero de RUC | Si |
| `cbc:ID/@schemeID` | Tipo doc identidad: "6" (RUC) - Catalogo No. 06 | Si |
| `cac:PartyName/cbc:Name` | Nombre comercial | Condicional |
| `cac:PartyLegalEntity/cbc:RegistrationName` | Razon social | Si |
| `cac:RegistrationAddress/cbc:ID` | Ubigeo (Catalogo No. 13) | Condicional |
| `cac:RegistrationAddress/cbc:AddressTypeCode` | Codigo de establecimiento | Condicional |
| `cac:RegistrationAddress/cbc:CitySubdivisionName` | Urbanizacion | Condicional |
| `cac:RegistrationAddress/cbc:CityName` | Provincia | Condicional |
| `cac:RegistrationAddress/cbc:CountrySubentity` | Departamento | Condicional |
| `cac:RegistrationAddress/cbc:District` | Distrito | Condicional |
| `cac:RegistrationAddress/cac:AddressLine/cbc:Line` | Direccion completa | Condicional |
| `cac:Country/cbc:IdentificationCode` | Codigo de pais (ISO 3166-1) | Condicional |

```xml
<cac:AccountingSupplierParty>
  <cac:Party>
    <cac:PartyIdentification>
      <cbc:ID schemeID="6" schemeName="Documento de Identidad"
        schemeAgencyName="PE:SUNAT"
        schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06">20100066603</cbc:ID>
    </cac:PartyIdentification>
    <cac:PartyName>
      <cbc:Name><![CDATA[SUNAT]]></cbc:Name>
    </cac:PartyName>
    <cac:PartyLegalEntity>
      <cbc:RegistrationName><![CDATA[SUNAT]]></cbc:RegistrationName>
      <cac:RegistrationAddress>
        <cbc:ID schemeName="Ubigeos" schemeAgencyName="PE:INEI">150101</cbc:ID>
        <cbc:AddressTypeCode listAgencyName="PE:SUNAT"
          listName="Establecimientos anexos">0001</cbc:AddressTypeCode>
        <cbc:CityName>LIMA</cbc:CityName>
        <cbc:CountrySubentity>LIMA</cbc:CountrySubentity>
        <cbc:District>LIMA</cbc:District>
        <cac:AddressLine>
          <cbc:Line><![CDATA[AV. GARCILASO DE LA VEGA 1472]]></cbc:Line>
        </cac:AddressLine>
        <cac:Country>
          <cbc:IdentificationCode listID="ISO 3166-1"
            listAgencyName="United Nations Economic Commission for Europe"
            listName="Country">PE</cbc:IdentificationCode>
        </cac:Country>
      </cac:RegistrationAddress>
    </cac:PartyLegalEntity>
  </cac:Party>
</cac:AccountingSupplierParty>
```

---

## 9. Datos del Receptor (AccountingCustomerParty)

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cac:PartyIdentification/cbc:ID` | Numero de documento | Si |
| `cbc:ID/@schemeID` | Tipo de documento (Catalogo No. 06) | Si |
| `cac:PartyLegalEntity/cbc:RegistrationName` | Razon social o Nombres | Si |

Tipos de documento de identidad (Catalogo No. 06):
| Codigo | Descripcion |
|--------|------------|
| 0 | No domiciliado, sin RUC |
| 1 | DNI |
| 4 | Carnet de Extranjeria |
| 6 | RUC |
| 7 | Pasaporte |
| A | Cedula Diplomatica de identidad |
| B | Documento identidad pais residencia - No Domiciliado |

```xml
<cac:AccountingCustomerParty>
  <cac:Party>
    <cac:PartyIdentification>
      <cbc:ID schemeID="6" schemeName="Documento de Identidad"
        schemeAgencyName="PE:SUNAT"
        schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06">20600695771</cbc:ID>
    </cac:PartyIdentification>
    <cac:PartyLegalEntity>
      <cbc:RegistrationName><![CDATA[EMPRESA XYZ S.A.C.]]></cbc:RegistrationName>
    </cac:PartyLegalEntity>
  </cac:Party>
</cac:AccountingCustomerParty>
```

---

## 10. Totales de Impuestos (TaxTotal)

### 10.1 Estructura General

Same structure as Invoice. Global tax totals with TaxSubtotal for each tax type.

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cbc:TaxAmount` | Total de impuestos | Si |
| `cac:TaxSubtotal/cbc:TaxableAmount` | Base imponible | Condicional |
| `cac:TaxSubtotal/cbc:TaxAmount` | Monto del tributo | Si |
| `cac:TaxCategory/cbc:ID` | Categoria (UN/ECE 5305) | Si |
| `cac:TaxScheme/cbc:ID` | Codigo del tributo (Catalogo No. 05) | Si |
| `cac:TaxScheme/cbc:Name` | Nombre del tributo | Si |
| `cac:TaxScheme/cbc:TaxTypeCode` | Codigo internacional | Si |

### 10.2 Catalogo No. 05 - Codigos de Tributos

| ID | Nombre | TaxTypeCode | Descripcion |
|----|--------|-------------|-------------|
| 1000 | IGV | VAT | Impuesto General a las Ventas |
| 1016 | IVAP | VAT | Impuesto Venta Arroz Pilado |
| 2000 | ISC | EXC | Impuesto Selectivo al Consumo |
| 7152 | ICBPER | OTH | Impuesto Bolsas de Plastico |
| 9995 | EXP | FRE | Exportacion |
| 9996 | GRA | FRE | Gratuito |
| 9997 | EXO | VAT | Exonerado |
| 9998 | INA | FRE | Inafecto |
| 9999 | OTROS | OTH | Otros tributos |

### 10.3 IGV

```xml
<cac:TaxTotal>
  <cbc:TaxAmount currencyID="PEN">180.00</cbc:TaxAmount>
  <cac:TaxSubtotal>
    <cbc:TaxableAmount currencyID="PEN">1000.00</cbc:TaxableAmount>
    <cbc:TaxAmount currencyID="PEN">180.00</cbc:TaxAmount>
    <cac:TaxCategory>
      <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
        schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
      <cac:TaxScheme>
        <cbc:ID schemeID="UN/ECE 5153" schemeName="Tax Scheme Identifier"
          schemeAgencyName="United Nations Economic Commission for Europe">1000</cbc:ID>
        <cbc:Name>IGV</cbc:Name>
        <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>
      </cac:TaxScheme>
    </cac:TaxCategory>
  </cac:TaxSubtotal>
</cac:TaxTotal>
```

---

## 11. Totales Monetarios (LegalMonetaryTotal)

| Campo | Descripcion | Tipo | Obligatorio |
|-------|------------|------|-------------|
| `cbc:LineExtensionAmount` | Total valor de venta | n(12,2) | Si |
| `cbc:TaxInclusiveAmount` | Total precio de venta | n(12,2) | Si |
| `cbc:AllowanceTotalAmount` | Total descuentos | n(12,2) | Condicional |
| `cbc:ChargeTotalAmount` | Total cargos | n(12,2) | Condicional |
| `cbc:PrepaidAmount` | Total anticipos | n(12,2) | Condicional |
| `cbc:PayableAmount` | Importe total | n(12,2) | Si |

```xml
<cac:LegalMonetaryTotal>
  <cbc:LineExtensionAmount currencyID="PEN">1000.00</cbc:LineExtensionAmount>
  <cbc:TaxInclusiveAmount currencyID="PEN">1180.00</cbc:TaxInclusiveAmount>
  <cbc:PayableAmount currencyID="PEN">1180.00</cbc:PayableAmount>
</cac:LegalMonetaryTotal>
```

---

## 12. Detalle de Items (CreditNoteLine)

**IMPORTANTE:** El elemento de linea se llama `CreditNoteLine` (no `InvoiceLine`).

### 12.1 Estructura General

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cbc:ID` | Numero de orden del item | Si |
| `cbc:CreditedQuantity` | Cantidad (nota: usa `CreditedQuantity`, no `InvoicedQuantity`) | Si |
| `cbc:CreditedQuantity/@unitCode` | Unidad de medida (Catalogo No. 03) | Si |
| `cbc:LineExtensionAmount` | Valor de venta del item | Si |

```xml
<cac:CreditNoteLine>
  <cbc:ID>1</cbc:ID>
  <cbc:CreditedQuantity unitCode="NIU" unitCodeListID="UN/ECE rec 20"
    unitCodeListAgencyName="United Nations Economic Commission for Europe">10</cbc:CreditedQuantity>
  <cbc:LineExtensionAmount currencyID="PEN">1000.00</cbc:LineExtensionAmount>
```

### 12.2 Precio de Venta Unitario (PricingReference)

Catalogo No. 16:
| Codigo | Descripcion |
|--------|------------|
| 01 | Precio unitario (incluye IGV) |
| 02 | Valor referencial unitario en operaciones no onerosas |

```xml
<cac:PricingReference>
  <cac:AlternativeConditionPrice>
    <cbc:PriceAmount currencyID="PEN">118.00</cbc:PriceAmount>
    <cbc:PriceTypeCode listName="SUNAT:Indicador de Tipo de Precio"
      listAgencyName="PE:SUNAT"
      listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo16">01</cbc:PriceTypeCode>
  </cac:AlternativeConditionPrice>
</cac:PricingReference>
```

### 12.3 Impuestos por Linea (TaxTotal)

Same structure as global TaxTotal but at line level.

Catalogo No. 07 - Tipos de Afectacion del IGV:
| Codigo | Descripcion | TaxScheme |
|--------|------------|-----------|
| 10 | Gravado - Operacion Onerosa | IGV (1000) |
| 11-16 | Gravado - Retiros (premio, donacion, publicidad, bonificaciones, trabajadores) | IGV (1000) |
| 17 | Gravado - IVAP | IVAP (1016) |
| 20 | Exonerado - Operacion Onerosa | EXO (9997) |
| 21 | Exonerado - Transferencia gratuita | GRA (9996) |
| 30 | Inafecto - Operacion Onerosa | INA (9998) |
| 31-37 | Inafecto - Gratuitos | GRA (9996) |
| 40 | Exportacion | EXP (9995) |

```xml
<cac:TaxTotal>
  <cbc:TaxAmount currencyID="PEN">18.00</cbc:TaxAmount>
  <cac:TaxSubtotal>
    <cbc:TaxableAmount currencyID="PEN">100.00</cbc:TaxableAmount>
    <cbc:TaxAmount currencyID="PEN">18.00</cbc:TaxAmount>
    <cac:TaxCategory>
      <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
        schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
      <cbc:Percent>18.00</cbc:Percent>
      <cbc:TaxExemptionReasonCode listAgencyName="PE:SUNAT"
        listName="SUNAT:Codigo de Tipo de Afectacion del IGV"
        listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo07">10</cbc:TaxExemptionReasonCode>
      <cac:TaxScheme>
        <cbc:ID schemeID="UN/ECE 5153" schemeName="Tax Scheme Identifier"
          schemeAgencyName="United Nations Economic Commission for Europe">1000</cbc:ID>
        <cbc:Name>IGV</cbc:Name>
        <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>
      </cac:TaxScheme>
    </cac:TaxCategory>
  </cac:TaxSubtotal>
</cac:TaxTotal>
```

### 12.4 Descripcion del Item

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cac:Item/cbc:Description` | Descripcion detallada | Si |
| `cac:Item/cac:SellersItemIdentification/cbc:ID` | Codigo del producto | Condicional |
| `cac:Item/cac:CommodityClassification/cbc:ItemClassificationCode` | Codigo UNSPSC | Condicional |

```xml
<cac:Item>
  <cbc:Description><![CDATA[Producto XYZ]]></cbc:Description>
  <cac:SellersItemIdentification>
    <cbc:ID>PROD-001</cbc:ID>
  </cac:SellersItemIdentification>
</cac:Item>
```

### 12.5 Valor Unitario

```xml
<cac:Price>
  <cbc:PriceAmount currencyID="PEN">100.00</cbc:PriceAmount>
</cac:Price>
```

---

## 13. Escenarios / Casos de Uso

### 13.1 Anulacion de Operacion (Tipo 01)

Se anula completamente la operacion original. El monto de la nota de credito debe coincidir con el monto del documento original.

### 13.2 Anulacion por Error en RUC (Tipo 02)

Se emite cuando el RUC del adquiriente fue consignado incorrectamente.

### 13.3 Correccion por Error en Descripcion (Tipo 03)

Para corregir errores en la descripcion de bienes o servicios.

### 13.4 Descuento Global (Tipo 04)

Descuento aplicado al monto total del documento original.

### 13.5 Descuento por Item (Tipo 05)

Descuento aplicado a items especificos del documento original.

### 13.6 Devolucion Total (Tipo 06)

Devolucion de la totalidad de bienes del documento original.

### 13.7 Devolucion por Item (Tipo 07)

Devolucion parcial de bienes del documento original.

### 13.8 Bonificacion (Tipo 08)

Por bienes entregados gratuitamente como bonificacion.

### 13.9 Disminucion en el Valor (Tipo 09)

Reduccion en el valor de la operacion.

### 13.10 Ajustes de Exportacion (Tipo 11)

Para ajustes en operaciones de exportacion.

### 13.11 Ajustes IVAP (Tipo 12)

Para ajustes en operaciones sujetas al IVAP.

---

## 14. Diferencias Clave con Invoice

| Aspecto | Invoice (Factura/Boleta) | CreditNote (Nota de Credito) |
|---------|------------------------|------------------------------|
| Elemento raiz | `<Invoice>` | `<CreditNote>` |
| Tipo de documento | `01` (Factura) / `03` (Boleta) | `07` (Nota de Credito) |
| Elemento de linea | `<cac:InvoiceLine>` | `<cac:CreditNoteLine>` |
| Cantidad | `cbc:InvoicedQuantity` | `cbc:CreditedQuantity` |
| Referencia a doc original | No aplica | `cac:DiscrepancyResponse` + `cac:BillingReference` |
| Motivo | No aplica | Catalogo No. 09 |

---

## 15. Validaciones

### 15.1 Validacion de Referencia

- El documento referenciado en `BillingReference` debe existir y haber sido informado a SUNAT.
- El tipo de documento referenciado debe ser compatible (Factura o Boleta).

### 15.2 Validacion de Montos

- Los montos de la Nota de Credito no pueden exceder los montos del documento original (acumulados con otras notas de credito previamente emitidas sobre el mismo documento).
- La moneda debe ser la misma que la del documento original.

### 15.3 Validacion de Numeracion

- Series que inician con "F" para notas asociadas a Facturas.
- Series que inician con "B" para notas asociadas a Boletas.

### 15.4 Validacion de Fechas

- La fecha de emision de la nota no puede ser anterior a la fecha del documento original.

---

## 16. Ejemplo Completo - Nota de Credito por Anulacion

```xml
<?xml version="1.0" encoding="utf-8"?>
<CreditNote xmlns="urn:oasis:names:specification:ubl:schema:xsd:CreditNote-2"
  xmlns:cac="urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"
  xmlns:cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2"
  xmlns:ext="urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2">

  <ext:UBLExtensions>
    <ext:UBLExtension>
      <ext:ExtensionContent/>
    </ext:UBLExtension>
  </ext:UBLExtensions>

  <cbc:UBLVersionID>2.1</cbc:UBLVersionID>
  <cbc:CustomizationID schemeAgencyName="PE:SUNAT">2.0</cbc:CustomizationID>
  <cbc:ID>FF01-1</cbc:ID>
  <cbc:IssueDate>2017-07-23</cbc:IssueDate>
  <cbc:IssueTime>12:35:00</cbc:IssueTime>
  <cbc:Note languageLocaleID="1000"><![CDATA[SON MIL CIENTO OCHENTA Y 00/100 SOLES]]></cbc:Note>
  <cbc:DocumentCurrencyCode listID="ISO 4217 Alpha" listName="Currency"
    listAgencyName="United Nations Economic Commission for Europe">PEN</cbc:DocumentCurrencyCode>

  <cac:DiscrepancyResponse>
    <cbc:ReferenceID>F001-1</cbc:ReferenceID>
    <cbc:ResponseCode listAgencyName="PE:SUNAT"
      listName="Tipo de nota de credito"
      listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo09">01</cbc:ResponseCode>
    <cbc:Description>Anulacion de la operacion</cbc:Description>
  </cac:DiscrepancyResponse>

  <cac:BillingReference>
    <cac:InvoiceDocumentReference>
      <cbc:ID>F001-1</cbc:ID>
      <cbc:DocumentTypeCode listAgencyName="PE:SUNAT"
        listName="Tipo de Documento"
        listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01">01</cbc:DocumentTypeCode>
    </cac:InvoiceDocumentReference>
  </cac:BillingReference>

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

  <cac:AccountingSupplierParty>
    <cac:Party>
      <cac:PartyIdentification>
        <cbc:ID schemeID="6" schemeName="Documento de Identidad"
          schemeAgencyName="PE:SUNAT"
          schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06">20100066603</cbc:ID>
      </cac:PartyIdentification>
      <cac:PartyLegalEntity>
        <cbc:RegistrationName><![CDATA[SUNAT]]></cbc:RegistrationName>
      </cac:PartyLegalEntity>
    </cac:Party>
  </cac:AccountingSupplierParty>

  <cac:AccountingCustomerParty>
    <cac:Party>
      <cac:PartyIdentification>
        <cbc:ID schemeID="6" schemeName="Documento de Identidad"
          schemeAgencyName="PE:SUNAT"
          schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06">20600695771</cbc:ID>
      </cac:PartyIdentification>
      <cac:PartyLegalEntity>
        <cbc:RegistrationName><![CDATA[EMPRESA XYZ S.A.C.]]></cbc:RegistrationName>
      </cac:PartyLegalEntity>
    </cac:Party>
  </cac:AccountingCustomerParty>

  <cac:TaxTotal>
    <cbc:TaxAmount currencyID="PEN">180.00</cbc:TaxAmount>
    <cac:TaxSubtotal>
      <cbc:TaxableAmount currencyID="PEN">1000.00</cbc:TaxableAmount>
      <cbc:TaxAmount currencyID="PEN">180.00</cbc:TaxAmount>
      <cac:TaxCategory>
        <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
          schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
        <cac:TaxScheme>
          <cbc:ID schemeID="UN/ECE 5153" schemeName="Tax Scheme Identifier"
            schemeAgencyName="United Nations Economic Commission for Europe">1000</cbc:ID>
          <cbc:Name>IGV</cbc:Name>
          <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>
        </cac:TaxScheme>
      </cac:TaxCategory>
    </cac:TaxSubtotal>
  </cac:TaxTotal>

  <cac:LegalMonetaryTotal>
    <cbc:LineExtensionAmount currencyID="PEN">1000.00</cbc:LineExtensionAmount>
    <cbc:TaxInclusiveAmount currencyID="PEN">1180.00</cbc:TaxInclusiveAmount>
    <cbc:PayableAmount currencyID="PEN">1180.00</cbc:PayableAmount>
  </cac:LegalMonetaryTotal>

  <cac:CreditNoteLine>
    <cbc:ID>1</cbc:ID>
    <cbc:CreditedQuantity unitCode="NIU" unitCodeListID="UN/ECE rec 20"
      unitCodeListAgencyName="United Nations Economic Commission for Europe">1</cbc:CreditedQuantity>
    <cbc:LineExtensionAmount currencyID="PEN">1000.00</cbc:LineExtensionAmount>
    <cac:PricingReference>
      <cac:AlternativeConditionPrice>
        <cbc:PriceAmount currencyID="PEN">1180.00</cbc:PriceAmount>
        <cbc:PriceTypeCode listName="SUNAT:Indicador de Tipo de Precio"
          listAgencyName="PE:SUNAT"
          listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo16">01</cbc:PriceTypeCode>
      </cac:AlternativeConditionPrice>
    </cac:PricingReference>
    <cac:TaxTotal>
      <cbc:TaxAmount currencyID="PEN">180.00</cbc:TaxAmount>
      <cac:TaxSubtotal>
        <cbc:TaxableAmount currencyID="PEN">1000.00</cbc:TaxableAmount>
        <cbc:TaxAmount currencyID="PEN">180.00</cbc:TaxAmount>
        <cac:TaxCategory>
          <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
            schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
          <cbc:Percent>18.00</cbc:Percent>
          <cbc:TaxExemptionReasonCode listAgencyName="PE:SUNAT"
            listName="SUNAT:Codigo de Tipo de Afectacion del IGV"
            listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo07">10</cbc:TaxExemptionReasonCode>
          <cac:TaxScheme>
            <cbc:ID schemeID="UN/ECE 5153" schemeName="Tax Scheme Identifier"
              schemeAgencyName="United Nations Economic Commission for Europe">1000</cbc:ID>
            <cbc:Name>IGV</cbc:Name>
            <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>
          </cac:TaxScheme>
        </cac:TaxCategory>
      </cac:TaxSubtotal>
    </cac:TaxTotal>
    <cac:Item>
      <cbc:Description><![CDATA[Producto de prueba]]></cbc:Description>
    </cac:Item>
    <cac:Price>
      <cbc:PriceAmount currencyID="PEN">1000.00</cbc:PriceAmount>
    </cac:Price>
  </cac:CreditNoteLine>
</CreditNote>
```

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
| 09 | Codigos de Tipo de Nota de Credito |
| 12 | Tipo de Documento Relacionado |
| 13 | Ubigeo (INEI) |
| 16 | Tipo de Precio de Venta Unitario |
| 51 | Tipo de Operacion |
| 52 | Codigos de Leyendas |
| 53 | Cargos o Descuentos |
