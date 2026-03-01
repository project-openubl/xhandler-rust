# Guia de elaboracion de documentos electronicos XML
## Resumen Diario de Boletas de Venta y Notas Electronicas

**Emisor:** SUNAT

---

## 1. Informacion General

### 1.1 Descripcion Funcional

El Resumen Diario es un documento XML que consolida las boletas de venta y notas de credito/debito asociadas a boletas emitidas en un dia determinado, para ser informadas a SUNAT.

A diferencia de las facturas (que se envian individualmente), las boletas de venta se envian agrupadas en resumenes diarios. Cada resumen contiene la informacion consolidada de uno o mas comprobantes (boletas, notas de credito asociadas a boletas, notas de debito asociadas a boletas).

### 1.2 Tipos de Documentos que Incluye

- Boleta de Venta (tipo 03)
- Nota de Credito asociada a Boleta (tipo 07)
- Nota de Debito asociada a Boleta (tipo 08)

### 1.3 Indicadores de Estado

Cada item del resumen tiene un indicador de estado:
| Codigo | Descripcion |
|--------|------------|
| 1 | Adicionar - Nuevo comprobante informado |
| 2 | Modificar - Correccion de datos del comprobante |
| 3 | Anulado - Anulacion del comprobante |

---

## 2. Estructura General del XML

### 2.1 Namespaces

El documento usa el namespace `SummaryDocuments` de SUNAT (no es un estandar UBL puro).

```xml
<?xml version="1.0" encoding="utf-8"?>
<SummaryDocuments xmlns="urn:sunat:names:specification:ubl:peru:schema:xsd:SummaryDocuments-1"
  xmlns:cac="urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"
  xmlns:cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2"
  xmlns:ds="http://www.w3.org/2000/09/xmldsig#"
  xmlns:ext="urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2"
  xmlns:sac="urn:sunat:names:specification:ubl:peru:schema:xsd:SunatAggregateComponents-1">
```

**Nota importante:** Este documento usa el namespace adicional `sac` (SunatAggregateComponents) que no esta presente en facturas, boletas, ni notas.

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

Valor fijo: `2.0`

```xml
<cbc:UBLVersionID>2.0</cbc:UBLVersionID>
```

### 3.2 Version de la Estructura (cbc:CustomizationID)

Valor fijo: `1.1`

```xml
<cbc:CustomizationID>1.1</cbc:CustomizationID>
```

### 3.3 Identificador del Resumen (cbc:ID)

Formato: `RC-YYYYMMDD-#####`
- `RC`: Prefijo fijo para Resumen de Comprobantes
- `YYYYMMDD`: Fecha de generacion del resumen
- `#####`: Correlativo del resumen del dia (5 digitos)

```xml
<cbc:ID>RC-20170625-00001</cbc:ID>
```

### 3.4 Fecha de Emision de los Documentos (cbc:ReferenceDate)

Fecha de emision de los documentos incluidos en el resumen.

Formato: `YYYY-MM-DD`

```xml
<cbc:ReferenceDate>2017-06-24</cbc:ReferenceDate>
```

### 3.5 Fecha de Generacion del Resumen (cbc:IssueDate)

Fecha en que se genera el resumen.

```xml
<cbc:IssueDate>2017-06-25</cbc:IssueDate>
```

**Regla:** La fecha de generacion (`IssueDate`) debe ser igual o posterior a la fecha de emision de los documentos (`ReferenceDate`).

---

## 4. Firma Digital

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

## 5. Datos del Emisor (AccountingSupplierParty)

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cbc:CustomerAssignedAccountID` | Numero de RUC | Si |
| `cbc:AdditionalAccountID` | Tipo de documento (6 = RUC) | Si |
| `cac:Party/cac:PartyLegalEntity/cbc:RegistrationName` | Razon social | Si |

```xml
<cac:AccountingSupplierParty>
  <cbc:CustomerAssignedAccountID>20100066603</cbc:CustomerAssignedAccountID>
  <cbc:AdditionalAccountID>6</cbc:AdditionalAccountID>
  <cac:Party>
    <cac:PartyLegalEntity>
      <cbc:RegistrationName><![CDATA[SUNAT]]></cbc:RegistrationName>
    </cac:PartyLegalEntity>
  </cac:Party>
</cac:AccountingSupplierParty>
```

---

## 6. Lineas del Resumen (sac:SummaryDocumentsLine)

Cada linea representa un documento (boleta o nota) que se esta informando.

### 6.1 Numero de Linea

```xml
<sac:SummaryDocumentsLine>
  <cbc:LineID>1</cbc:LineID>
```

### 6.2 Tipo de Documento

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cbc:DocumentTypeCode` | Tipo de documento (03=Boleta, 07=Nota Credito, 08=Nota Debito) | Si |

```xml
<cbc:DocumentTypeCode>03</cbc:DocumentTypeCode>
```

### 6.3 Numero de Serie y Correlativo del Documento

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cbc:ID` | Serie-Correlativo del documento | Si |

```xml
<cbc:ID>B001-1</cbc:ID>
```

### 6.4 Datos del Receptor

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cac:AccountingCustomerParty/cbc:CustomerAssignedAccountID` | Numero de documento del adquiriente | Condicional |
| `cac:AccountingCustomerParty/cbc:AdditionalAccountID` | Tipo de documento de identidad (Catalogo No. 06) | Condicional |

```xml
<cac:AccountingCustomerParty>
  <cbc:CustomerAssignedAccountID>46237547</cbc:CustomerAssignedAccountID>
  <cbc:AdditionalAccountID>1</cbc:AdditionalAccountID>
</cac:AccountingCustomerParty>
```

### 6.5 Referencia al Documento que Modifica (solo para Notas)

Cuando el item del resumen es una nota de credito o debito:

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cac:BillingReference/cac:InvoiceDocumentReference/cbc:ID` | Numero del documento que modifica | Condicional |
| `cac:BillingReference/cac:InvoiceDocumentReference/cbc:DocumentTypeCode` | Tipo de documento que modifica | Condicional |

```xml
<cac:BillingReference>
  <cac:InvoiceDocumentReference>
    <cbc:ID>B001-5</cbc:ID>
    <cbc:DocumentTypeCode>03</cbc:DocumentTypeCode>
  </cac:InvoiceDocumentReference>
</cac:BillingReference>
```

### 6.6 Estado del Item

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cac:Status/cbc:ConditionCode` | Estado del documento (1=Adicionar, 2=Modificar, 3=Anulado) | Si |

```xml
<cac:Status>
  <cbc:ConditionCode>1</cbc:ConditionCode>
</cac:Status>
```

### 6.7 Importe Total de la Venta

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `sac:TotalAmount` | Importe total del documento (incluye impuestos) | Si |

```xml
<sac:TotalAmount currencyID="PEN">1180.00</sac:TotalAmount>
```

### 6.8 Total Valor de Venta por Tipo de Operacion (sac:BillingPayment)

Se incluye un `sac:BillingPayment` por cada tipo de operacion presente en el documento.

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cbc:PaidAmount` | Total valor de venta por tipo | Si |
| `cbc:InstructionID` | Tipo de operacion | Si |

Tipos de operacion:
| InstructionID | Descripcion |
|---------------|------------|
| 01 | Gravado (operaciones gravadas) |
| 02 | Exonerado (operaciones exoneradas) |
| 03 | Inafecto (operaciones inafectas) |
| 04 | Exportacion |
| 05 | Gratuitas |

```xml
<sac:BillingPayment>
  <cbc:PaidAmount currencyID="PEN">1000.00</cbc:PaidAmount>
  <cbc:InstructionID>01</cbc:InstructionID>
</sac:BillingPayment>
```

Se pueden incluir multiples `sac:BillingPayment` si el documento tiene diferentes tipos de operacion:

```xml
<sac:BillingPayment>
  <cbc:PaidAmount currencyID="PEN">800.00</cbc:PaidAmount>
  <cbc:InstructionID>01</cbc:InstructionID>
</sac:BillingPayment>
<sac:BillingPayment>
  <cbc:PaidAmount currencyID="PEN">200.00</cbc:PaidAmount>
  <cbc:InstructionID>02</cbc:InstructionID>
</sac:BillingPayment>
```

### 6.9 Otros Cargos (sac:AllowanceCharge)

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cbc:ChargeIndicator` | `true` para cargos, `false` para descuentos | Condicional |
| `cbc:Amount` | Monto del cargo/descuento | Condicional |

```xml
<sac:AllowanceCharge>
  <cbc:ChargeIndicator>true</cbc:ChargeIndicator>
  <cbc:Amount currencyID="PEN">50.00</cbc:Amount>
</sac:AllowanceCharge>
```

### 6.10 Impuestos (cac:TaxTotal)

Se incluyen los totales de impuestos del documento.

| Campo | Descripcion | Obligatorio |
|-------|------------|-------------|
| `cbc:TaxAmount` | Monto total del impuesto | Si |
| `cac:TaxSubtotal/cbc:TaxAmount` | Monto del impuesto por tipo | Si |
| `cac:TaxScheme/cbc:ID` | Codigo del tributo (Catalogo No. 05) | Si |
| `cac:TaxScheme/cbc:Name` | Nombre del tributo | Si |
| `cac:TaxScheme/cbc:TaxTypeCode` | Codigo internacional | Si |

Tributos aplicables:
| ID | Nombre | TaxTypeCode |
|----|--------|-------------|
| 1000 | IGV | VAT |
| 2000 | ISC | EXC |
| 9999 | OTROS | OTH |
| 7152 | ICBPER | OTH |

```xml
<cac:TaxTotal>
  <cbc:TaxAmount currencyID="PEN">180.00</cbc:TaxAmount>
  <cac:TaxSubtotal>
    <cbc:TaxAmount currencyID="PEN">180.00</cbc:TaxAmount>
    <cac:TaxCategory>
      <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
        schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
      <cac:TaxScheme>
        <cbc:ID>1000</cbc:ID>
        <cbc:Name>IGV</cbc:Name>
        <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>
      </cac:TaxScheme>
    </cac:TaxCategory>
  </cac:TaxSubtotal>
</cac:TaxTotal>
```

---

## 7. Ejemplo Completo - Resumen Diario

```xml
<?xml version="1.0" encoding="utf-8"?>
<SummaryDocuments xmlns="urn:sunat:names:specification:ubl:peru:schema:xsd:SummaryDocuments-1"
  xmlns:cac="urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"
  xmlns:cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2"
  xmlns:ds="http://www.w3.org/2000/09/xmldsig#"
  xmlns:ext="urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2"
  xmlns:sac="urn:sunat:names:specification:ubl:peru:schema:xsd:SunatAggregateComponents-1">

  <ext:UBLExtensions>
    <ext:UBLExtension>
      <ext:ExtensionContent/>
    </ext:UBLExtension>
  </ext:UBLExtensions>

  <cbc:UBLVersionID>2.0</cbc:UBLVersionID>
  <cbc:CustomizationID>1.1</cbc:CustomizationID>
  <cbc:ID>RC-20170625-00001</cbc:ID>
  <cbc:ReferenceDate>2017-06-24</cbc:ReferenceDate>
  <cbc:IssueDate>2017-06-25</cbc:IssueDate>

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
    <cbc:CustomerAssignedAccountID>20100066603</cbc:CustomerAssignedAccountID>
    <cbc:AdditionalAccountID>6</cbc:AdditionalAccountID>
    <cac:Party>
      <cac:PartyLegalEntity>
        <cbc:RegistrationName><![CDATA[SUNAT]]></cbc:RegistrationName>
      </cac:PartyLegalEntity>
    </cac:Party>
  </cac:AccountingSupplierParty>

  <!-- Linea 1: Boleta nueva (Adicionar) -->
  <sac:SummaryDocumentsLine>
    <cbc:LineID>1</cbc:LineID>
    <cbc:DocumentTypeCode>03</cbc:DocumentTypeCode>
    <cbc:ID>B001-1</cbc:ID>
    <cac:AccountingCustomerParty>
      <cbc:CustomerAssignedAccountID>46237547</cbc:CustomerAssignedAccountID>
      <cbc:AdditionalAccountID>1</cbc:AdditionalAccountID>
    </cac:AccountingCustomerParty>
    <cac:Status>
      <cbc:ConditionCode>1</cbc:ConditionCode>
    </cac:Status>
    <sac:TotalAmount currencyID="PEN">1180.00</sac:TotalAmount>
    <sac:BillingPayment>
      <cbc:PaidAmount currencyID="PEN">1000.00</cbc:PaidAmount>
      <cbc:InstructionID>01</cbc:InstructionID>
    </sac:BillingPayment>
    <cac:TaxTotal>
      <cbc:TaxAmount currencyID="PEN">180.00</cbc:TaxAmount>
      <cac:TaxSubtotal>
        <cbc:TaxAmount currencyID="PEN">180.00</cbc:TaxAmount>
        <cac:TaxCategory>
          <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
            schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
          <cac:TaxScheme>
            <cbc:ID>1000</cbc:ID>
            <cbc:Name>IGV</cbc:Name>
            <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>
          </cac:TaxScheme>
        </cac:TaxCategory>
      </cac:TaxSubtotal>
    </cac:TaxTotal>
  </sac:SummaryDocumentsLine>

  <!-- Linea 2: Boleta anulada -->
  <sac:SummaryDocumentsLine>
    <cbc:LineID>2</cbc:LineID>
    <cbc:DocumentTypeCode>03</cbc:DocumentTypeCode>
    <cbc:ID>B001-2</cbc:ID>
    <cac:AccountingCustomerParty>
      <cbc:CustomerAssignedAccountID>-</cbc:CustomerAssignedAccountID>
      <cbc:AdditionalAccountID>0</cbc:AdditionalAccountID>
    </cac:AccountingCustomerParty>
    <cac:Status>
      <cbc:ConditionCode>3</cbc:ConditionCode>
    </cac:Status>
    <sac:TotalAmount currencyID="PEN">590.00</sac:TotalAmount>
    <sac:BillingPayment>
      <cbc:PaidAmount currencyID="PEN">500.00</cbc:PaidAmount>
      <cbc:InstructionID>01</cbc:InstructionID>
    </sac:BillingPayment>
    <cac:TaxTotal>
      <cbc:TaxAmount currencyID="PEN">90.00</cbc:TaxAmount>
      <cac:TaxSubtotal>
        <cbc:TaxAmount currencyID="PEN">90.00</cbc:TaxAmount>
        <cac:TaxCategory>
          <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
            schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
          <cac:TaxScheme>
            <cbc:ID>1000</cbc:ID>
            <cbc:Name>IGV</cbc:Name>
            <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>
          </cac:TaxScheme>
        </cac:TaxCategory>
      </cac:TaxSubtotal>
    </cac:TaxTotal>
  </sac:SummaryDocumentsLine>

  <!-- Linea 3: Nota de credito asociada a boleta -->
  <sac:SummaryDocumentsLine>
    <cbc:LineID>3</cbc:LineID>
    <cbc:DocumentTypeCode>07</cbc:DocumentTypeCode>
    <cbc:ID>BC01-1</cbc:ID>
    <cac:AccountingCustomerParty>
      <cbc:CustomerAssignedAccountID>46237547</cbc:CustomerAssignedAccountID>
      <cbc:AdditionalAccountID>1</cbc:AdditionalAccountID>
    </cac:AccountingCustomerParty>
    <cac:BillingReference>
      <cac:InvoiceDocumentReference>
        <cbc:ID>B001-5</cbc:ID>
        <cbc:DocumentTypeCode>03</cbc:DocumentTypeCode>
      </cac:InvoiceDocumentReference>
    </cac:BillingReference>
    <cac:Status>
      <cbc:ConditionCode>1</cbc:ConditionCode>
    </cac:Status>
    <sac:TotalAmount currencyID="PEN">236.00</sac:TotalAmount>
    <sac:BillingPayment>
      <cbc:PaidAmount currencyID="PEN">200.00</cbc:PaidAmount>
      <cbc:InstructionID>01</cbc:InstructionID>
    </sac:BillingPayment>
    <cac:TaxTotal>
      <cbc:TaxAmount currencyID="PEN">36.00</cbc:TaxAmount>
      <cac:TaxSubtotal>
        <cbc:TaxAmount currencyID="PEN">36.00</cbc:TaxAmount>
        <cac:TaxCategory>
          <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
            schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
          <cac:TaxScheme>
            <cbc:ID>1000</cbc:ID>
            <cbc:Name>IGV</cbc:Name>
            <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>
          </cac:TaxScheme>
        </cac:TaxCategory>
      </cac:TaxSubtotal>
    </cac:TaxTotal>
  </sac:SummaryDocumentsLine>
</SummaryDocuments>
```

---

## 8. Validaciones

### 8.1 Formato del Identificador

- Debe seguir el formato `RC-YYYYMMDD-#####`.
- El correlativo debe ser unico por dia.

### 8.2 Fechas

- `ReferenceDate` (fecha de emision de documentos) no puede ser futura.
- `IssueDate` (fecha de generacion) debe ser >= `ReferenceDate`.
- `IssueDate` no puede ser mayor a 7 dias calendario despues de `ReferenceDate`.

### 8.3 Documentos

- Todos los documentos del resumen deben tener la misma `ReferenceDate`.
- La serie de los documentos debe iniciar con "B".
- El tipo de documento debe ser 03 (Boleta), 07 (Nota Credito) o 08 (Nota Debito).

### 8.4 Notas de Credito/Debito

- Cuando el tipo es 07 o 08, debe incluir `BillingReference` con referencia al documento original.
- El documento referenciado debe ser tipo 03 (Boleta).

### 8.5 Estado

- `ConditionCode` debe ser 1 (Adicionar), 2 (Modificar) o 3 (Anulado).
- Para adicionar (1), el documento no debe haber sido informado previamente.
- Para modificar (2) o anular (3), el documento debe haber sido previamente informado con estado 1.

### 8.6 Montos

- `TotalAmount` debe ser > 0 (excepto anulaciones).
- Debe incluir al menos un `BillingPayment`.
- La suma de impuestos debe ser coherente con los valores de venta.

---

## 9. Proceso de Envio

### 9.1 Flujo

1. El emisor genera las boletas y notas durante el dia.
2. Al final del dia (o al dia siguiente), genera el Resumen Diario con todas las boletas/notas emitidas.
3. El Resumen se firma digitalmente.
4. Se empaqueta en un ZIP y se envia a SUNAT via SOAP.
5. SUNAT devuelve un numero de ticket.
6. Con el ticket, se consulta el estado del procesamiento.
7. SUNAT devuelve el CDR (Constancia de Recepcion) con el resultado.

### 9.2 Nombre del Archivo ZIP

Formato: `{RUC}-RC-{YYYYMMDD}-{correlativo}.zip`

Ejemplo: `20100066603-RC-20170625-00001.zip`

### 9.3 Nombre del Archivo XML dentro del ZIP

Formato: `{RUC}-RC-{YYYYMMDD}-{correlativo}.xml`

Ejemplo: `20100066603-RC-20170625-00001.xml`

---

## 10. Diferencias con Comunicacion de Baja

| Aspecto | Resumen Diario | Comunicacion de Baja |
|---------|---------------|---------------------|
| Prefijo | RC | RA |
| Documentos | Boletas, Notas asociadas a Boletas | Facturas, Notas asociadas a Facturas |
| Contenido | Datos resumidos (totales, impuestos) | Solo serie-correlativo y motivo |
| Estado | Adicionar (1), Modificar (2), Anular (3) | Solo baja |
| Uso | Informar y/o anular boletas | Solo anular facturas |

---

## Anexo - Resumen de Campos

| Seccion | Campo | Descripcion | Obligatorio |
|---------|-------|------------|-------------|
| Encabezado | `cbc:UBLVersionID` | Version UBL: "2.0" | Si |
| Encabezado | `cbc:CustomizationID` | Version estructura: "1.1" | Si |
| Encabezado | `cbc:ID` | Identificador RC-YYYYMMDD-##### | Si |
| Encabezado | `cbc:ReferenceDate` | Fecha de emision documentos | Si |
| Encabezado | `cbc:IssueDate` | Fecha de generacion resumen | Si |
| Emisor | `cbc:CustomerAssignedAccountID` | RUC emisor | Si |
| Emisor | `cbc:AdditionalAccountID` | Tipo doc: "6" | Si |
| Emisor | `cbc:RegistrationName` | Razon social | Si |
| Linea | `cbc:LineID` | Numero de linea | Si |
| Linea | `cbc:DocumentTypeCode` | Tipo doc: 03, 07, 08 | Si |
| Linea | `cbc:ID` | Serie-correlativo documento | Si |
| Linea | `cbc:CustomerAssignedAccountID` | Doc identidad adquiriente | Condicional |
| Linea | `cbc:AdditionalAccountID` | Tipo doc identidad | Condicional |
| Linea | `cac:BillingReference` | Ref. doc que modifica (notas) | Condicional |
| Linea | `cbc:ConditionCode` | Estado: 1, 2, 3 | Si |
| Linea | `sac:TotalAmount` | Importe total | Si |
| Linea | `sac:BillingPayment` | Total por tipo operacion | Si |
| Linea | `cac:TaxTotal` | Impuestos | Si |

---

## Anexo B - Catalogo No. 06 - Tipos de Documento de Identidad

| Codigo | Descripcion |
|--------|------------|
| 0 | No domiciliado, sin RUC |
| 1 | DNI |
| 4 | Carnet de Extranjeria |
| 6 | RUC |
| 7 | Pasaporte |
| A | Cedula Diplomatica |
| B | Doc. identidad pais residencia |
