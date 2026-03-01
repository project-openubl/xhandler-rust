# Guia de elaboracion de documentos electronicos XML - UBL 2.1

## Factura Electronica

**Version:** 2024.02.22
**Basado en:** UBL 2.1
**Emisor:** SUNAT - Superintendencia Nacional de Aduanas y de Administracion Tributaria (Peru)

---

## 1. Informacion General

### 1.1 Descripcion Funcional

La factura electronica es un comprobante de pago emitido por el contribuyente (emisor electronico) a traves del Sistema de Emision Electronica, conforme a la normatividad vigente.

### 1.2 Documento XML Estandar

El documento XML se basa en el estandar UBL (Universal Business Language) version 2.1, publicado por OASIS.

### 1.3 Condiciones de Emision

- El emisor electronico debe encontrarse en estado activo y habido en el RUC.
- Se debe emitir en moneda nacional o extranjera.
- Debe contener la firma digital del emisor.

---

## 2. Estructura General del XML

### 2.1 Declaracion y Namespaces

```xml
<?xml version="1.0" encoding="utf-8"?>
<Invoice xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2"
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
      <!-- Firma digital -->
    </ext:ExtensionContent>
  </ext:UBLExtension>
</ext:UBLExtensions>
```

---

## 3. Datos del Encabezado del Documento

### 3.1 Version UBL

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:UBLVersionID` | Version del UBL | an | 3 | Si |

Valor fijo: `2.1`

```xml
<cbc:UBLVersionID>2.1</cbc:UBLVersionID>
```

### 3.2 Version de la Estructura del Documento

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:CustomizationID` | Version de la estructura del documento | an | 3 | Si |

Valor fijo: `2.0`

Atributo `schemeAgencyName`: "PE:SUNAT"

```xml
<cbc:CustomizationID schemeAgencyName="PE:SUNAT">2.0</cbc:CustomizationID>
```

### 3.3 Numeracion del Documento

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:ID` | Numeracion, conformada por serie y numero correlativo | an | 13 | Si |

Formato: `F###-NNNNNNNN` (Serie de 4 caracteres + guion + correlativo hasta 8 digitos)

- La serie comienza con "F" para facturas.
- El numero correlativo no debe contener ceros a la izquierda innecesarios.

```xml
<cbc:ID>F001-1</cbc:ID>
```

### 3.4 Fecha de Emision

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:IssueDate` | Fecha de emision | an | 10 | Si |

Formato: `YYYY-MM-DD`

```xml
<cbc:IssueDate>2017-07-08</cbc:IssueDate>
```

### 3.5 Hora de Emision

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:IssueTime` | Hora de emision | an | 8 | Condicional |

Formato: `HH:MM:SS`

```xml
<cbc:IssueTime>18:30:00</cbc:IssueTime>
```

### 3.6 Fecha de Vencimiento

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:DueDate` | Fecha de vencimiento | an | 10 | Condicional |

```xml
<cbc:DueDate>2017-07-28</cbc:DueDate>
```

### 3.7 Tipo de Documento

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:InvoiceTypeCode` | Tipo de comprobante | an | 2 | Si |

Valor: `01` (Factura)

Atributos:
- `listAgencyName`: "PE:SUNAT"
- `listName`: "Tipo de Documento"
- `listURI`: "urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01"
- `listID`: Tipo de operacion segun Catalogo No. 51
- `name`: Denominacion del tipo de operacion

Valores de `listID` (Catalogo No. 51):
| Codigo | Descripcion |
|--------|------------|
| 0101 | Venta interna |
| 0112 | Venta interna - sustenta gastos deducibles persona natural |
| 0113 | Venta interna - NRUS |
| 0200 | Exportacion de bienes |
| 0201 | Exportacion de servicios - prestacion de servicios realizados integramente en el pais |
| 0202 | Exportacion de servicios - prestacion de servicios de hospedaje No Domiciliado |
| 0203 | Exportacion de servicios - transporte de navieras |
| 0204 | Exportacion de servicios - servicios a turistas no domiciliados |
| 0205 | Exportacion de servicios - venta de bienes muebles |
| 0206 | Exportacion de servicios - servicios complementarios al transporte de carga |
| 0207 | Exportacion de servicios - suministro de energia electrica |
| 0208 | Exportacion de servicios - prestacion de servicios realizados parcialmente en el extranjero |
| 0301 | Operaciones con Carta de porte aereo (emitidas en el ambito nacional) |
| 0302 | Operaciones de Transporte ferroviario de pasajeros |
| 0401 | Ventas no domiciliados que no califican como exportacion |
| 1001 | Operacion sujeta a detraccion |
| 1002 | Operacion sujeta a detraccion - Recursos Hidrobiologicos |
| 1003 | Operacion sujeta a detraccion - Servicios de transporte de pasajeros |
| 1004 | Operacion sujeta a detraccion - Servicios de transporte de carga |
| 2001 | Operacion sujeta a percepcion |

```xml
<cbc:InvoiceTypeCode listAgencyName="PE:SUNAT" listName="Tipo de Documento"
  listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01"
  listID="0101" name="Venta Interna">01</cbc:InvoiceTypeCode>
```

### 3.8 Leyendas

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:Note` | Leyenda | an | 1500 | Condicional |

Atributo `languageLocaleID` referencia al Catalogo No. 52.

Codigos principales del Catalogo No. 52:
| Codigo | Descripcion |
|--------|------------|
| 1000 | Monto en letras |
| 1002 | Leyenda "TRANSFERENCIA GRATUITA DE UN BIEN Y/O SERVICIO PRESTADO GRATUITAMENTE" |
| 2000 | Leyenda "COMPROBANTE DE PERCEPCION" |
| 2001 | Leyenda "BIENES TRANSFERIDOS EN LA AMAZONIA..." |
| 2002 | Leyenda "SERVICIOS PRESTADOS EN LA AMAZONIA..." |
| 2003 | Leyenda "CONTRATOS DE CONSTRUCCION EJECUTADOS EN LA AMAZONIA..." |
| 2004 | Leyenda sobre NRUS |
| 2005 | Leyenda sobre operacion sujeta a detraccion |
| 2006 | Leyenda "Operacion sujeta a IVAP" |
| 2007 | Leyenda sobre venta a no domiciliados |
| 2008 | Leyenda sobre exportacion de servicios |
| 2009 | Leyenda sobre servicio de hospedaje |
| 2010 | Leyenda sobre servicios de transporte |

```xml
<cbc:Note languageLocaleID="1000"><![CDATA[SON MIL QUINIENTOS Y 00/100 SOLES]]></cbc:Note>
```

Se permite mas de un `cbc:Note` con diferentes `languageLocaleID`.

### 3.9 Tipo de Moneda

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:DocumentCurrencyCode` | Tipo de moneda | an | 3 | Si |

Catalogo No. 02 (ISO 4217).

```xml
<cbc:DocumentCurrencyCode listID="ISO 4217 Alpha" listName="Currency"
  listAgencyName="United Nations Economic Commission for Europe">PEN</cbc:DocumentCurrencyCode>
```

Monedas principales:
| Codigo | Descripcion |
|--------|------------|
| PEN | Sol |
| USD | Dolar americano |

### 3.10 Numero de Orden de Compra

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:OrderReference/cbc:ID` | Numero de orden de compra o servicio | an | Max 20 | Condicional |

```xml
<cac:OrderReference>
  <cbc:ID>0001-12000256</cbc:ID>
</cac:OrderReference>
```

### 3.11 Guia de Remision Relacionada

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:DespatchDocumentReference/cbc:ID` | Numero de guia | an | Max 30 | Condicional |
| `cac:DespatchDocumentReference/cbc:DocumentTypeCode` | Tipo de documento | an | 2 | Condicional |

```xml
<cac:DespatchDocumentReference>
  <cbc:ID>T001-1</cbc:ID>
  <cbc:DocumentTypeCode listAgencyName="PE:SUNAT" listName="Tipo de Documento"
    listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01">09</cbc:DocumentTypeCode>
</cac:DespatchDocumentReference>
```

### 3.12 Otros Documentos Relacionados

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:AdditionalDocumentReference/cbc:ID` | Numero de documento | an | Max 30 | Condicional |
| `cac:AdditionalDocumentReference/cbc:DocumentTypeCode` | Tipo de documento | an | 2 | Condicional |

Tipos de documento relacionado (Catalogo No. 12):
| Codigo | Descripcion |
|--------|------------|
| 01 | Numeracion DAM |
| 04 | Numero de Resolucion de Intendencia |
| 05 | BVME Domiciliados |
| 06 | Numero de autorizacion emitida por el ente regulador (OSIPTEL, OSINERG, etc.) |
| 99 | Otros |

```xml
<cac:AdditionalDocumentReference>
  <cbc:ID>118-2017-000025</cbc:ID>
  <cbc:DocumentTypeCode listAgencyName="PE:SUNAT" listName="Documento Relacionado"
    listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo12">01</cbc:DocumentTypeCode>
</cac:AdditionalDocumentReference>
```

---

## 4. Firma Digital

| Campo | Descripcion | Tipo | Obligatorio |
|-------|------------|------|-------------|
| `cac:Signature/cbc:ID` | Identificador de la firma | an | Si |
| `cac:Signature/cac:SignatoryParty/cac:PartyIdentification/cbc:ID` | RUC del firmante | an | Si |
| `cac:Signature/cac:SignatoryParty/cac:PartyName/cbc:Name` | Nombre del firmante | an | Si |
| `cac:Signature/cac:DigitalSignatureAttachment/cac:ExternalReference/cbc:URI` | Referencia a la firma | an | Si |

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

## 5. Datos del Emisor

### 5.1 AccountingSupplierParty

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:CustomerAssignedAccountID` | Numero de RUC | n | 11 | Si |
| `cbc:AdditionalAccountID` | Tipo de documento de identidad | n | 1 | Si |
| `cac:Party/cac:PartyIdentification/cbc:ID` | Numero de documento | an | Max 15 | Si |
| `cac:Party/cac:PartyIdentification/cbc:ID/@schemeID` | Tipo de documento (Catalogo No. 06) | an | 1 | Si |
| `cac:Party/cac:PartyName/cbc:Name` | Nombre comercial | an | Max 1500 | Condicional |
| `cac:Party/cac:PartyLegalEntity/cbc:RegistrationName` | Razon social | an | Max 1500 | Si |
| `cac:Party/cac:PartyLegalEntity/cac:RegistrationAddress/cbc:ID` | Ubigeo (Catalogo No. 13) | an | 6 | Condicional |
| `cac:Party/cac:PartyLegalEntity/cac:RegistrationAddress/cbc:AddressTypeCode` | Codigo de establecimiento | an | 4 | Condicional |
| `cac:Party/cac:PartyLegalEntity/cac:RegistrationAddress/cbc:CitySubdivisionName` | Urbanizacion | an | Max 25 | Condicional |
| `cac:Party/cac:PartyLegalEntity/cac:RegistrationAddress/cbc:CityName` | Provincia | an | Max 30 | Condicional |
| `cac:Party/cac:PartyLegalEntity/cac:RegistrationAddress/cbc:CountrySubentity` | Departamento | an | Max 30 | Condicional |
| `cac:Party/cac:PartyLegalEntity/cac:RegistrationAddress/cbc:District` | Distrito | an | Max 30 | Condicional |
| `cac:Party/cac:PartyLegalEntity/cac:RegistrationAddress/cac:AddressLine/cbc:Line` | Direccion completa | an | Max 200 | Condicional |
| `cac:Party/cac:PartyLegalEntity/cac:RegistrationAddress/cac:Country/cbc:IdentificationCode` | Codigo de pais | an | 2 | Condicional |

Tipos de documento de identidad (Catalogo No. 06):
| Codigo | Descripcion |
|--------|------------|
| 0 | No domiciliado, sin RUC (solo para exportacion) |
| 1 | DNI |
| 4 | Carnet de Extranjeria |
| 6 | RUC |
| 7 | Pasaporte |
| A | Cedula Diplomatica de identidad |
| B | Documento de identidad pais de residencia - No Domiciliado |

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
        <cbc:CitySubdivisionName>-</cbc:CitySubdivisionName>
        <cbc:CityName>LIMA</cbc:CityName>
        <cbc:CountrySubentity>LIMA</cbc:CountrySubentity>
        <cbc:District>LIMA</cbc:District>
        <cac:AddressLine>
          <cbc:Line><![CDATA[AV. GARCILASO DE LA VEGA 1472]]></cbc:Line>
        </cac:AddressLine>
        <cac:Country>
          <cbc:IdentificationCode listID="ISO 3166-1" listAgencyName="United Nations Economic Commission for Europe"
            listName="Country">PE</cbc:IdentificationCode>
        </cac:Country>
      </cac:RegistrationAddress>
    </cac:PartyLegalEntity>
  </cac:Party>
</cac:AccountingSupplierParty>
```

---

## 6. Datos del Receptor (Adquiriente/Cliente)

### 6.1 AccountingCustomerParty

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:Party/cac:PartyIdentification/cbc:ID` | Numero de documento | an | Max 15 | Si |
| `cac:Party/cac:PartyIdentification/cbc:ID/@schemeID` | Tipo de documento (Catalogo No. 06) | an | 1 | Si |
| `cac:Party/cac:PartyLegalEntity/cbc:RegistrationName` | Apellidos y nombres o Razon social | an | Max 1500 | Si |

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
      <cac:RegistrationAddress>
        <cac:AddressLine>
          <cbc:Line><![CDATA[AV. LOS PINOS 123]]></cbc:Line>
        </cac:AddressLine>
        <cac:Country>
          <cbc:IdentificationCode listID="ISO 3166-1"
            listAgencyName="United Nations Economic Commission for Europe"
            listName="Country">PE</cbc:IdentificationCode>
        </cac:Country>
      </cac:RegistrationAddress>
    </cac:PartyLegalEntity>
  </cac:Party>
</cac:AccountingCustomerParty>
```

---

## 7. Detraccion / Forma de Pago

### 7.1 Informacion de Detraccion

Obligatorio cuando el tipo de operacion (Catalogo No. 51) es "1001", "1002", "1003" o "1004".

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:PaymentMeans/cbc:ID` | Medio de pago | an | Max 3 | Condicional |
| `cac:PaymentMeans/cbc:PaymentMeansCode` | Codigo del medio de pago | an | Max 3 | Condicional |
| `cac:PaymentMeans/cac:PayeeFinancialAccount/cbc:ID` | Numero de cuenta | an | Max 30 | Condicional |

Valores para `cbc:ID`:
- `Detraccion`: Para operaciones sujetas a detraccion

Valores para `cbc:PaymentMeansCode` (Catalogo No. 59):
| Codigo | Descripcion |
|--------|------------|
| 001 | Deposito en cuenta |
| 002 | Giro |
| 003 | Transferencia de fondos |
| 004 | Orden de pago |
| 005 | Tarjeta de debito |
| 006 | Tarjeta de credito emitida en el pais por una empresa del sistema financiero |
| 007 | Cheques con la clausula de "NO NEGOCIABLE" |
| 008 | Efectivo, por operaciones en las que no existe obligacion de utilizar medio de pago |
| 009 | Efectivo, en los demas casos |
| 010 | Medios de pago usados en comercio exterior |
| 011 | Documentos valorados y titulos valores |
| 012 | Otros medios de pago |

```xml
<cac:PaymentMeans>
  <cbc:ID>Detraccion</cbc:ID>
  <cbc:PaymentMeansCode>001</cbc:PaymentMeansCode>
  <cac:PayeeFinancialAccount>
    <cbc:ID>00-068-123456</cbc:ID>
  </cac:PayeeFinancialAccount>
</cac:PaymentMeans>
```

### 7.2 Monto y Porcentaje de Detraccion

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:PaymentTerms/cbc:ID` | Identificador de detraccion | an | - | Condicional |
| `cac:PaymentTerms/cbc:PaymentMeansID` | Codigo del bien o servicio (Catalogo No. 54) | an | 3 | Condicional |
| `cac:PaymentTerms/cbc:Amount` | Monto de la detraccion | n | 12,2 | Condicional |
| `cac:PaymentTerms/cbc:PaymentPercent` | Porcentaje de detraccion | n | 5,2 | Condicional |

```xml
<cac:PaymentTerms>
  <cbc:ID>Detraccion</cbc:ID>
  <cbc:PaymentMeansID>025</cbc:PaymentMeansID>
  <cbc:Amount currencyID="PEN">285.00</cbc:Amount>
  <cbc:PaymentPercent>15.00</cbc:PaymentPercent>
</cac:PaymentTerms>
```

### 7.3 Forma de Pago

Permite indicar si la operacion es al contado o al credito.

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:PaymentTerms/cbc:ID` | Tipo de forma de pago | an | - | Condicional |
| `cac:PaymentTerms/cbc:Amount` | Monto neto pendiente de pago (solo Credito) | n | 12,2 | Condicional |

Valores para `cbc:ID`:
- `FormaPago` con `cbc:PaymentMeansID` = `Contado` o `Credito`

**Pago al contado:**
```xml
<cac:PaymentTerms>
  <cbc:ID>FormaPago</cbc:ID>
  <cbc:PaymentMeansID>Contado</cbc:PaymentMeansID>
</cac:PaymentTerms>
```

**Pago al credito:**
```xml
<cac:PaymentTerms>
  <cbc:ID>FormaPago</cbc:ID>
  <cbc:PaymentMeansID>Credito</cbc:PaymentMeansID>
  <cbc:Amount currencyID="PEN">1000.00</cbc:Amount>
</cac:PaymentTerms>
```

### 7.4 Cuotas de Pago (Credito)

Cuando la forma de pago es "Credito", se deben consignar las cuotas.

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:PaymentTerms/cbc:ID` | Identificador de cuota | an | - | Condicional |
| `cac:PaymentTerms/cbc:PaymentMeansID` | Numero de cuota | an | - | Condicional |
| `cac:PaymentTerms/cbc:Amount` | Monto de la cuota | n | 12,2 | Condicional |
| `cac:PaymentTerms/cbc:PaymentDueDate` | Fecha de vencimiento de la cuota | an | 10 | Condicional |

```xml
<cac:PaymentTerms>
  <cbc:ID>FormaPago</cbc:ID>
  <cbc:PaymentMeansID>Cuota001</cbc:PaymentMeansID>
  <cbc:Amount currencyID="PEN">500.00</cbc:Amount>
  <cbc:PaymentDueDate>2021-04-15</cbc:PaymentDueDate>
</cac:PaymentTerms>
<cac:PaymentTerms>
  <cbc:ID>FormaPago</cbc:ID>
  <cbc:PaymentMeansID>Cuota002</cbc:PaymentMeansID>
  <cbc:Amount currencyID="PEN">500.00</cbc:Amount>
  <cbc:PaymentDueDate>2021-05-15</cbc:PaymentDueDate>
</cac:PaymentTerms>
```

---

## 8. Prepago

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:PrepaidPayment/cbc:ID` | Identificador de pago | an | - | Condicional |
| `cac:PrepaidPayment/cbc:PaidAmount` | Monto de anticipo | n | 12,2 | Condicional |
| `cac:PrepaidPayment/cbc:InstructionID` | Identificacion del documento relacionado | an | - | Condicional |

```xml
<cac:PrepaidPayment>
  <cbc:ID schemeID="02" schemeName="Tipo de Documento"
    schemeAgencyName="PE:SUNAT">F001-5</cbc:ID>
  <cbc:PaidAmount currencyID="PEN">200.00</cbc:PaidAmount>
</cac:PrepaidPayment>
```

---

## 9. Cargos y Descuentos Globales

### 9.1 AllowanceCharge a nivel de documento

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:AllowanceCharge/cbc:ChargeIndicator` | Indicador de cargo/descuento | an | 4-5 | Si |
| `cac:AllowanceCharge/cbc:AllowanceChargeReasonCode` | Codigo de cargo/descuento (Catalogo No. 53) | an | 2 | Si |
| `cac:AllowanceCharge/cbc:MultiplierFactorNumeric` | Factor de cargo/descuento | n | 5,5 | Si |
| `cac:AllowanceCharge/cbc:Amount` | Monto de cargo/descuento | n | 12,2 | Si |
| `cac:AllowanceCharge/cbc:BaseAmount` | Monto base | n | 12,2 | Si |

Valores de `cbc:ChargeIndicator`:
- `false`: Descuento
- `true`: Cargo

Codigos del Catalogo No. 53 (Descuentos):
| Codigo | Descripcion | ChargeIndicator |
|--------|------------|-----------------|
| 00 | Descuentos que afectan la base imponible del IGV/IVAP | false |
| 01 | Descuentos que no afectan la base imponible del IGV/IVAP | false |
| 02 | Descuento global que afecta la base imponible del IGV/IVAP (por item) | false |
| 03 | Descuento global que no afecta la base imponible del IGV/IVAP (por item) | false |
| 04 | Descuentos globales por anticipos - Gravadas | false |
| 05 | Descuentos globales por anticipos - Exoneradas | false |
| 06 | Descuentos globales por anticipos - Inafectas | false |

Codigos del Catalogo No. 53 (Cargos):
| Codigo | Descripcion | ChargeIndicator |
|--------|------------|-----------------|
| 45 | FISE (Fondo de Inclusion Social Energetico) | true |
| 46 | Recargo al consumo y/o propinas | true |
| 47 | Cargos que afectan la base imponible del IGV/IVAP | true |
| 48 | Cargos que no afectan la base imponible del IGV/IVAP | true |
| 49 | Cargos globales que afectan la base imponible del IGV/IVAP (por item) | true |
| 50 | Cargos globales que no afectan la base imponible del IGV/IVAP (por item) | true |

**Ejemplo - Descuento global que afecta base imponible:**
```xml
<cac:AllowanceCharge>
  <cbc:ChargeIndicator>false</cbc:ChargeIndicator>
  <cbc:AllowanceChargeReasonCode listAgencyName="PE:SUNAT"
    listName="Cargo/descuento" listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo53">02</cbc:AllowanceChargeReasonCode>
  <cbc:MultiplierFactorNumeric>0.10</cbc:MultiplierFactorNumeric>
  <cbc:Amount currencyID="PEN">100.00</cbc:Amount>
  <cbc:BaseAmount currencyID="PEN">1000.00</cbc:BaseAmount>
</cac:AllowanceCharge>
```

---

## 10. Totales de Impuestos (TaxTotal)

### 10.1 Estructura General

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:TaxTotal/cbc:TaxAmount` | Total de impuestos | n | 12,2 | Si |
| `cac:TaxTotal/cac:TaxSubtotal/cbc:TaxableAmount` | Total valor de venta (base imponible) | n | 12,2 | Condicional |
| `cac:TaxTotal/cac:TaxSubtotal/cbc:TaxAmount` | Sumatoria de tributos | n | 12,2 | Si |
| `cac:TaxTotal/cac:TaxSubtotal/cac:TaxCategory/cbc:ID` | Codigo de categoria de tributo (UN/ECE 5305) | an | 1 | Si |
| `cac:TaxTotal/cac:TaxSubtotal/cac:TaxCategory/cbc:Percent` | Porcentaje/Tasa | n | 3,5 | Condicional |
| `cac:TaxTotal/cac:TaxSubtotal/cac:TaxCategory/cbc:TaxExemptionReasonCode` | Codigo de afectacion del IGV (Catalogo No. 07) | an | 2 | Condicional |
| `cac:TaxTotal/cac:TaxSubtotal/cac:TaxCategory/cac:TaxScheme/cbc:ID` | Codigo del tributo (Catalogo No. 05) | an | 4 | Si |
| `cac:TaxTotal/cac:TaxSubtotal/cac:TaxCategory/cac:TaxScheme/cbc:Name` | Nombre del tributo | an | Max 6 | Si |
| `cac:TaxTotal/cac:TaxSubtotal/cac:TaxCategory/cac:TaxScheme/cbc:TaxTypeCode` | Codigo internacional del tributo | an | 3 | Si |

### 10.2 Catalogo No. 05 - Codigos de Tributos

| ID (cbc:ID) | Nombre (cbc:Name) | Codigo UN/ECE 5153 (cbc:TaxTypeCode) | Descripcion |
|-------------|-------------------|---------------------------------------|-------------|
| 1000 | IGV | VAT | Impuesto General a las Ventas |
| 1016 | IVAP | VAT | Impuesto a la Venta de Arroz Pilado |
| 2000 | ISC | EXC | Impuesto Selectivo al Consumo |
| 7152 | ICBPER | OTH | Impuesto al Consumo de las Bolsas de Plastico |
| 9995 | EXP | FRE | Exportacion |
| 9996 | GRA | FRE | Gratuito |
| 9997 | EXO | VAT | Exonerado |
| 9998 | INA | FRE | Inafecto |
| 9999 | OTROS | OTH | Otros tributos |

### 10.3 Catalogo No. 07 - Codigos de Tipo de Afectacion del IGV

| Codigo | Descripcion | Tipo Tributo |
|--------|------------|--------------|
| 10 | Gravado - Operacion Onerosa | IGV (1000) |
| 11 | Gravado - Retiro por premio | IGV (1000) |
| 12 | Gravado - Retiro por donacion | IGV (1000) |
| 13 | Gravado - Retiro | IGV (1000) |
| 14 | Gravado - Retiro por publicidad | IGV (1000) |
| 15 | Gravado - Bonificaciones | IGV (1000) |
| 16 | Gravado - Retiro por entrega a trabajadores | IGV (1000) |
| 17 | Gravado - IVAP | IVAP (1016) |
| 20 | Exonerado - Operacion Onerosa | EXO (9997) |
| 21 | Exonerado - Transferencia gratuita | GRA (9996) |
| 30 | Inafecto - Operacion Onerosa | INA (9998) |
| 31 | Inafecto - Retiro por bonificacion | GRA (9996) |
| 32 | Inafecto - Retiro | GRA (9996) |
| 33 | Inafecto - Retiro por Muestras Medicas | GRA (9996) |
| 34 | Inafecto - Retiro por Convenio Colectivo | GRA (9996) |
| 35 | Inafecto - Retiro por premio | GRA (9996) |
| 36 | Inafecto - Retiro por publicidad | GRA (9996) |
| 37 | Inafecto - Transferencia gratuita | GRA (9996) |
| 40 | Exportacion de Bienes o Servicios | EXP (9995) |

**Ejemplo - IGV gravado:**
```xml
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
```

### 10.4 Catalogo No. 08 - Tipos de Sistema de Calculo del ISC

| Codigo | Descripcion |
|--------|------------|
| 01 | Sistema al valor (Monto fijo) |
| 02 | Aplicacion del monto fijo |
| 03 | Sistema de precios de venta al publico |

### 10.5 Mapeo de TaxCategory ID (UN/ECE 5305)

| Codigo | Descripcion | Uso |
|--------|------------|-----|
| S | Standard rate | Gravado IGV/IVAP |
| AA | Lower rate | - |
| E | Exempt from tax | Exonerado |
| G | Free export item | Exportacion |
| O | Services outside scope of tax | Inafecto/Gratuito |
| Z | Zero rated goods | - |

---

## 11. Total Valores de Venta (LegalMonetaryTotal)

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:LineExtensionAmount` | Total valor de venta | n | 12,2 | Si |
| `cbc:TaxInclusiveAmount` | Total precio de venta (con impuestos) | n | 12,2 | Si |
| `cbc:AllowanceTotalAmount` | Total descuentos | n | 12,2 | Condicional |
| `cbc:ChargeTotalAmount` | Total otros cargos | n | 12,2 | Condicional |
| `cbc:PrepaidAmount` | Total anticipos | n | 12,2 | Condicional |
| `cbc:PayableAmount` | Importe total de la venta | n | 12,2 | Si |
| `cbc:PayableRoundingAmount` | Redondeo aplicado al total | n | 12,2 | Condicional |

Reglas de calculo:
- `LineExtensionAmount` = Suma de (Valor de venta de cada linea)
- `TaxInclusiveAmount` = `LineExtensionAmount` + `TaxTotal` - Descuentos Globales + Cargos Globales
- `PayableAmount` = `TaxInclusiveAmount` - `PrepaidAmount` + `PayableRoundingAmount`

```xml
<cac:LegalMonetaryTotal>
  <cbc:LineExtensionAmount currencyID="PEN">1000.00</cbc:LineExtensionAmount>
  <cbc:TaxInclusiveAmount currencyID="PEN">1180.00</cbc:TaxInclusiveAmount>
  <cbc:AllowanceTotalAmount currencyID="PEN">0.00</cbc:AllowanceTotalAmount>
  <cbc:ChargeTotalAmount currencyID="PEN">0.00</cbc:ChargeTotalAmount>
  <cbc:PayableAmount currencyID="PEN">1180.00</cbc:PayableAmount>
</cac:LegalMonetaryTotal>
```

---

## 12. Detalle de Items / Lineas (InvoiceLine)

### 12.1 Estructura General

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:ID` | Numero de orden del item | n | - | Si |
| `cbc:InvoicedQuantity` | Cantidad | n | 12,10 | Si |
| `cbc:InvoicedQuantity/@unitCode` | Unidad de medida (Catalogo No. 03) | an | 3 | Si |
| `cbc:InvoicedQuantity/@unitCodeListID` | "UN/ECE rec 20" | an | - | Si |
| `cbc:InvoicedQuantity/@unitCodeListAgencyName` | "United Nations Economic Commission for Europe" | an | - | Si |
| `cbc:LineExtensionAmount` | Valor de venta del item | n | 12,2 | Si |

```xml
<cac:InvoiceLine>
  <cbc:ID>1</cbc:ID>
  <cbc:InvoicedQuantity unitCode="NIU" unitCodeListID="UN/ECE rec 20"
    unitCodeListAgencyName="United Nations Economic Commission for Europe">10</cbc:InvoicedQuantity>
  <cbc:LineExtensionAmount currencyID="PEN">1000.00</cbc:LineExtensionAmount>
```

### 12.2 Referencia de Precios (PricingReference)

#### 12.2.1 Precio Unitario con Impuestos (Valor referencial unitario en operaciones no onerosas)

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:PricingReference/cac:AlternativeConditionPrice/cbc:PriceAmount` | Precio unitario (incluye impuestos) | n | 12,10 | Si |
| `cac:PricingReference/cac:AlternativeConditionPrice/cbc:PriceTypeCode` | Tipo de precio (Catalogo No. 16) | an | 2 | Si |

Catalogo No. 16 - Tipo de Precio de Venta Unitario:
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

### 12.3 Descuentos y Cargos por Item

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:AllowanceCharge/cbc:ChargeIndicator` | Indicador de cargo/descuento | an | - | Condicional |
| `cac:AllowanceCharge/cbc:AllowanceChargeReasonCode` | Codigo del Catalogo No. 53 | an | 2 | Condicional |
| `cac:AllowanceCharge/cbc:MultiplierFactorNumeric` | Factor | n | 5,5 | Condicional |
| `cac:AllowanceCharge/cbc:Amount` | Monto de descuento/cargo | n | 12,2 | Condicional |
| `cac:AllowanceCharge/cbc:BaseAmount` | Monto base | n | 12,2 | Condicional |

### 12.4 Impuestos por Linea (TaxTotal)

Misma estructura que los impuestos globales (seccion 10), pero a nivel de linea.

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

### 12.5 Descripcion del Item

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:Item/cbc:Description` | Descripcion detallada del bien o servicio | an | Max 500 | Si |
| `cac:Item/cac:SellersItemIdentification/cbc:ID` | Codigo del producto del vendedor | an | Max 30 | Condicional |
| `cac:Item/cac:CommodityClassification/cbc:ItemClassificationCode` | Codigo del producto (UNSPSC, GS1) | an | Max 18 | Condicional |
| `cac:Item/cac:AdditionalItemProperty/cbc:Name` | Propiedad adicional - Nombre | an | Max 100 | Condicional |
| `cac:Item/cac:AdditionalItemProperty/cbc:NameCode` | Propiedad adicional - Codigo | an | Max 4 | Condicional |
| `cac:Item/cac:AdditionalItemProperty/cbc:Value` | Propiedad adicional - Valor | an | Max 100 | Condicional |

```xml
<cac:Item>
  <cbc:Description><![CDATA[Televisor plasma de 42", marca "RCA"]]></cbc:Description>
  <cac:SellersItemIdentification>
    <cbc:ID>PROD-001</cbc:ID>
  </cac:SellersItemIdentification>
  <cac:CommodityClassification>
    <cbc:ItemClassificationCode listID="UNSPSC" listAgencyName="GS1 US"
      listName="Item Classification">52161505</cbc:ItemClassificationCode>
  </cac:CommodityClassification>
</cac:Item>
```

### 12.6 Valor Unitario del Item

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:Price/cbc:PriceAmount` | Valor unitario (sin impuestos) | n | 12,10 | Si |

```xml
<cac:Price>
  <cbc:PriceAmount currencyID="PEN">100.00</cbc:PriceAmount>
</cac:Price>
```

---

## 13. Percepcion

Cuando el tipo de operacion (Catalogo No. 51) es "2001".

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cac:PaymentTerms/cbc:ID` | "Percepcion" | an | - | Condicional |
| `cac:PaymentTerms/cbc:Amount` | Monto de la percepcion | n | 12,2 | Condicional |
| `cac:PaymentTerms/cbc:PaymentPercent` | Tasa de percepcion (Catalogo No. 61) | n | 5,2 | Condicional |

```xml
<cac:PaymentTerms>
  <cbc:ID>Percepcion</cbc:ID>
  <cbc:Amount currencyID="PEN">23.60</cbc:Amount>
  <cbc:PaymentPercent>2.00</cbc:PaymentPercent>
</cac:PaymentTerms>
```

---

## 14. Tipo de Cambio

Cuando la moneda del documento es diferente a PEN, se debe incluir el tipo de cambio.

| Campo | Descripcion | Tipo | Longitud | Obligatorio |
|-------|------------|------|----------|-------------|
| `cbc:SourceCurrencyCode` | Moneda de origen (moneda del documento) | an | 3 | Condicional |
| `cbc:TargetCurrencyCode` | Moneda de destino (PEN) | an | 3 | Condicional |
| `cbc:CalculationRate` | Tipo de cambio | n | 4,6 | Condicional |
| `cbc:Date` | Fecha del tipo de cambio | an | 10 | Condicional |

---

## 15. Escenarios / Casos de Uso

### 15.1 Operacion Gravada con IGV (Caso 1)

Operacion estandar con IGV al 18%.

- `InvoiceTypeCode/@listID` = "0101" (Venta interna)
- `TaxExemptionReasonCode` = "10" (Gravado - Operacion Onerosa)
- `TaxScheme/cbc:ID` = "1000" (IGV)
- `PriceTypeCode` = "01" (Precio unitario incluye IGV)

### 15.2 Operacion Gravada con IGV e ISC (Caso 2)

Operacion con IGV e ISC.

- Se agregan dos `TaxSubtotal` en el `TaxTotal` de la linea.
- ISC se calcula primero, su monto se agrega a la base imponible del IGV.
- `TaxScheme/cbc:ID` = "2000" (ISC)
- Incluye `cbc:TierRange` para el tipo de sistema de calculo ISC (Catalogo No. 08).

### 15.3 Operacion Exonerada (Caso 3)

- `TaxExemptionReasonCode` = "20" (Exonerado - Operacion Onerosa)
- `TaxScheme/cbc:ID` = "9997" (EXO)
- `TaxCategory/cbc:ID` = "E"
- IGV = 0.00

### 15.4 Operacion Inafecta (Caso 4)

- `TaxExemptionReasonCode` = "30" (Inafecto - Operacion Onerosa)
- `TaxScheme/cbc:ID` = "9998" (INA)
- `TaxCategory/cbc:ID` = "O"
- IGV = 0.00

### 15.5 Operacion Gratuita - Gravada (Caso 5)

Transferencia gratuita gravada con IGV.

- `TaxExemptionReasonCode` = "11" a "16" (segun tipo de retiro)
- `TaxScheme/cbc:ID` = "1000" (IGV) para el calculo del impuesto
- `TaxScheme/cbc:ID` = "9996" (GRA) en un segundo TaxSubtotal
- `PriceTypeCode` = "02" (Valor referencial)
- `Price/PriceAmount` = 0.00 (valor de venta es cero)
- `LineExtensionAmount` = 0.00
- Leyenda obligatoria (Catalogo 52, codigo 1002): "TRANSFERENCIA GRATUITA DE UN BIEN Y/O SERVICIO PRESTADO GRATUITAMENTE"

### 15.6 Operacion Gratuita - Inafecta (Caso 6)

Transferencia gratuita inafecta.

- `TaxExemptionReasonCode` = "31" a "37" (segun tipo)
- `TaxScheme/cbc:ID` = "9996" (GRA)
- `TaxCategory/cbc:ID` = "O"
- `PriceTypeCode` = "02" (Valor referencial)
- `Price/PriceAmount` = 0.00
- `LineExtensionAmount` = 0.00

### 15.7 Operacion Gratuita - Exonerada (Caso 7)

Transferencia gratuita exonerada.

- `TaxExemptionReasonCode` = "21"
- `TaxScheme/cbc:ID` = "9996" (GRA)
- `TaxCategory/cbc:ID` = "E"

### 15.8 Operacion de Exportacion (Caso 8)

- `InvoiceTypeCode/@listID` = "0200" a "0208" (segun tipo de exportacion)
- `TaxExemptionReasonCode` = "40"
- `TaxScheme/cbc:ID` = "9995" (EXP)
- `TaxCategory/cbc:ID` = "G"

### 15.9 Operacion con IVAP (Caso 9)

Impuesto a la Venta del Arroz Pilado.

- `TaxExemptionReasonCode` = "17"
- `TaxScheme/cbc:ID` = "1016" (IVAP)
- Tasa: 4%
- `InvoiceTypeCode/@listID` debe contener valor del Catalogo 51 que indique IVAP

### 15.10 Operacion con ICBPER (Caso 10)

Impuesto al Consumo de las Bolsas de Plastico.

- `TaxScheme/cbc:ID` = "7152" (ICBPER)
- `TaxCategory/cbc:ID` = "O"
- Se calcula como: cantidad x monto fijo por bolsa
- El monto se incluye como un TaxSubtotal adicional en la linea

```xml
<cac:TaxSubtotal>
  <cbc:TaxAmount currencyID="PEN">0.30</cbc:TaxAmount>
  <cac:TaxCategory>
    <cbc:Percent>0.30</cbc:Percent>
    <cbc:TaxExemptionReasonCode listAgencyName="PE:SUNAT"
      listName="SUNAT:Codigo de Tipo de Afectacion del IGV"
      listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo07">10</cbc:TaxExemptionReasonCode>
    <cac:TaxScheme>
      <cbc:ID schemeID="UN/ECE 5153" schemeName="Tax Scheme Identifier"
        schemeAgencyName="United Nations Economic Commission for Europe">7152</cbc:ID>
      <cbc:Name>ICBPER</cbc:Name>
      <cbc:TaxTypeCode>OTH</cbc:TaxTypeCode>
    </cac:TaxScheme>
  </cac:TaxCategory>
</cac:TaxSubtotal>
```

### 15.11 Operacion con Detraccion (Caso 11)

- `InvoiceTypeCode/@listID` = "1001", "1002", "1003" o "1004"
- Incluye `PaymentMeans` y `PaymentTerms` con datos de detraccion
- Leyenda obligatoria (Catalogo 52, codigo 2006 o similar)

### 15.12 Operacion con Percepcion (Caso 12)

- `InvoiceTypeCode/@listID` = "2001"
- Incluye `PaymentTerms` con datos de percepcion

### 15.13 Operacion con Descuentos (Caso 13)

- Descuentos que afectan la base imponible (codigo 00 por linea, 02 global)
- Descuentos que NO afectan la base imponible (codigo 01 por linea, 03 global)

### 15.14 Operacion con Anticipos (Caso 14)

- Se incluye `PrepaidPayment` con referencia al documento de anticipo
- Se incluyen descuentos globales por anticipos (codigos 04, 05, 06 del Catalogo 53)
- El `PrepaidAmount` se descuenta del `PayableAmount`

### 15.15 Factura con Operaciones Mixtas

Cuando una factura tiene items con diferentes tipos de afectacion (gravado, exonerado, inafecto, gratuito), se debe generar un `TaxSubtotal` por cada tipo de tributo en el `TaxTotal` global.

---

## 16. Unidades de Medida (Catalogo No. 03)

Codigo basado en el estandar UN/ECE Recommendation 20.

Unidades comunes:
| Codigo | Descripcion |
|--------|------------|
| NIU | Unidad (numero de unidades) |
| ZZ | Servicio |
| KGM | Kilogramo |
| LTR | Litro |
| MTR | Metro |
| MTK | Metro cuadrado |
| MTQ | Metro cubico |
| GRM | Gramo |
| TNE | Tonelada |
| BX | Caja |
| PK | Paquete |
| DZN | Docena |
| SET | Juego |

---

## 17. Validaciones Importantes

### 17.1 Validacion de Numeracion

- La serie debe iniciar con "F" para facturas.
- El correlativo es un numero entero sin ceros a la izquierda innecesarios.
- Formato: `F###-########` (4 caracteres serie + guion + hasta 8 digitos correlativo).

### 17.2 Validacion de Montos

- Los montos deben tener como maximo 2 decimales (excepto cantidades que permiten hasta 10 decimales y valores unitarios que permiten hasta 10 decimales).
- Los montos del atributo `currencyID` deben coincidir con `DocumentCurrencyCode`.

### 17.3 Validacion de Impuestos

- La sumatoria de impuestos por linea debe coincidir con los totales globales.
- `TaxTotal/TaxAmount` = Suma de todos los `TaxSubtotal/TaxAmount`.
- La base imponible (`TaxableAmount`) debe coincidir con la suma de los valores de venta de las lineas correspondientes.

### 17.4 Validacion de Totales

- `LineExtensionAmount` = Suma de `LineExtensionAmount` de todas las lineas.
- `TaxInclusiveAmount` = `LineExtensionAmount` + Impuestos - Descuentos globales + Cargos globales.
- `PayableAmount` = `TaxInclusiveAmount` - `PrepaidAmount` + `PayableRoundingAmount`.

### 17.5 Operaciones Gratuitas

- `LineExtensionAmount` de la linea = 0.00.
- `Price/PriceAmount` = 0.00.
- `PriceTypeCode` = "02".
- Se debe incluir el valor referencial en `AlternativeConditionPrice`.
- No se suma a los totales de venta gravada/exonerada/inafecta onerosa.

---

## 18. Reglas de Negocio / Validaciones SUNAT

### 18.1 Reglas sobre el Emisor

- El RUC del emisor debe estar activo y habido.
- El tipo de documento del emisor debe ser "6" (RUC).
- El emisor debe estar autorizado como emisor electronico.

### 18.2 Reglas sobre el Adquiriente

- Para facturas, el tipo de documento del adquiriente es generalmente "6" (RUC).
- Para ventas a personas naturales (B2C) con montos menores a 700 PEN, se puede usar "-" como numero de documento.
- Para exportaciones, el tipo de documento puede ser "0" (No domiciliado).

### 18.3 Reglas sobre Fechas

- La fecha de emision no puede ser anterior a mas de 7 dias calendario de la fecha de envio a SUNAT.
- La fecha de emision no puede ser posterior a la fecha de envio.

### 18.4 Reglas sobre el Monto

- El monto total `PayableAmount` debe ser mayor a cero (excepto operaciones 100% gratuitas).
- El redondeo (`PayableRoundingAmount`) no puede exceder +/- 1.00.

---

## 19. Ejemplo Completo - Factura Gravada con IGV

```xml
<?xml version="1.0" encoding="utf-8"?>
<Invoice xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2"
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
  <cbc:ID>F001-1</cbc:ID>
  <cbc:IssueDate>2017-07-08</cbc:IssueDate>
  <cbc:IssueTime>18:30:00</cbc:IssueTime>
  <cbc:InvoiceTypeCode listAgencyName="PE:SUNAT" listName="Tipo de Documento"
    listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01"
    listID="0101" name="Venta Interna">01</cbc:InvoiceTypeCode>
  <cbc:Note languageLocaleID="1000"><![CDATA[SON CIENTO DIECIOCHO Y 00/100 SOLES]]></cbc:Note>
  <cbc:DocumentCurrencyCode listID="ISO 4217 Alpha" listName="Currency"
    listAgencyName="United Nations Economic Commission for Europe">PEN</cbc:DocumentCurrencyCode>

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

  <cac:LegalMonetaryTotal>
    <cbc:LineExtensionAmount currencyID="PEN">100.00</cbc:LineExtensionAmount>
    <cbc:TaxInclusiveAmount currencyID="PEN">118.00</cbc:TaxInclusiveAmount>
    <cbc:PayableAmount currencyID="PEN">118.00</cbc:PayableAmount>
  </cac:LegalMonetaryTotal>

  <cac:InvoiceLine>
    <cbc:ID>1</cbc:ID>
    <cbc:InvoicedQuantity unitCode="NIU" unitCodeListID="UN/ECE rec 20"
      unitCodeListAgencyName="United Nations Economic Commission for Europe">1</cbc:InvoicedQuantity>
    <cbc:LineExtensionAmount currencyID="PEN">100.00</cbc:LineExtensionAmount>
    <cac:PricingReference>
      <cac:AlternativeConditionPrice>
        <cbc:PriceAmount currencyID="PEN">118.00</cbc:PriceAmount>
        <cbc:PriceTypeCode listName="SUNAT:Indicador de Tipo de Precio"
          listAgencyName="PE:SUNAT"
          listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo16">01</cbc:PriceTypeCode>
      </cac:AlternativeConditionPrice>
    </cac:PricingReference>
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
    <cac:Item>
      <cbc:Description><![CDATA[Producto de prueba]]></cbc:Description>
      <cac:CommodityClassification>
        <cbc:ItemClassificationCode listID="UNSPSC" listAgencyName="GS1 US"
          listName="Item Classification">52161505</cbc:ItemClassificationCode>
      </cac:CommodityClassification>
    </cac:Item>
    <cac:Price>
      <cbc:PriceAmount currencyID="PEN">100.00</cbc:PriceAmount>
    </cac:Price>
  </cac:InvoiceLine>
</Invoice>
```

---

## 20. Ejemplo - Factura Gratuita (Inafecta)

```xml
<!-- Linea con transferencia gratuita inafecta -->
<cac:InvoiceLine>
  <cbc:ID>1</cbc:ID>
  <cbc:InvoicedQuantity unitCode="NIU" unitCodeListID="UN/ECE rec 20"
    unitCodeListAgencyName="United Nations Economic Commission for Europe">1</cbc:InvoicedQuantity>
  <cbc:LineExtensionAmount currencyID="PEN">0.00</cbc:LineExtensionAmount>
  <cac:PricingReference>
    <cac:AlternativeConditionPrice>
      <cbc:PriceAmount currencyID="PEN">100.00</cbc:PriceAmount>
      <cbc:PriceTypeCode listName="SUNAT:Indicador de Tipo de Precio"
        listAgencyName="PE:SUNAT"
        listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo16">02</cbc:PriceTypeCode>
    </cac:AlternativeConditionPrice>
  </cac:PricingReference>
  <cac:TaxTotal>
    <cbc:TaxAmount currencyID="PEN">0.00</cbc:TaxAmount>
    <cac:TaxSubtotal>
      <cbc:TaxableAmount currencyID="PEN">100.00</cbc:TaxableAmount>
      <cbc:TaxAmount currencyID="PEN">0.00</cbc:TaxAmount>
      <cac:TaxCategory>
        <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
          schemeAgencyName="United Nations Economic Commission for Europe">O</cbc:ID>
        <cbc:Percent>18.00</cbc:Percent>
        <cbc:TaxExemptionReasonCode listAgencyName="PE:SUNAT"
          listName="SUNAT:Codigo de Tipo de Afectacion del IGV"
          listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo07">37</cbc:TaxExemptionReasonCode>
        <cac:TaxScheme>
          <cbc:ID schemeID="UN/ECE 5153" schemeName="Tax Scheme Identifier"
            schemeAgencyName="United Nations Economic Commission for Europe">9996</cbc:ID>
          <cbc:Name>GRA</cbc:Name>
          <cbc:TaxTypeCode>FRE</cbc:TaxTypeCode>
        </cac:TaxScheme>
      </cac:TaxCategory>
    </cac:TaxSubtotal>
  </cac:TaxTotal>
  <cac:Item>
    <cbc:Description><![CDATA[Producto gratuito]]></cbc:Description>
  </cac:Item>
  <cac:Price>
    <cbc:PriceAmount currencyID="PEN">0.00</cbc:PriceAmount>
  </cac:Price>
</cac:InvoiceLine>
```

---

## 21. Ejemplo - Factura con ISC

```xml
<!-- TaxTotal de linea con ISC + IGV -->
<cac:TaxTotal>
  <cbc:TaxAmount currencyID="PEN">230.40</cbc:TaxAmount>
  <!-- ISC -->
  <cac:TaxSubtotal>
    <cbc:TaxableAmount currencyID="PEN">1000.00</cbc:TaxableAmount>
    <cbc:TaxAmount currencyID="PEN">100.00</cbc:TaxAmount>
    <cac:TaxCategory>
      <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
        schemeAgencyName="United Nations Economic Commission for Europe">S</cbc:ID>
      <cbc:Percent>10.00</cbc:Percent>
      <cbc:TierRange>01</cbc:TierRange>
      <cac:TaxScheme>
        <cbc:ID schemeID="UN/ECE 5153" schemeName="Tax Scheme Identifier"
          schemeAgencyName="United Nations Economic Commission for Europe">2000</cbc:ID>
        <cbc:Name>ISC</cbc:Name>
        <cbc:TaxTypeCode>EXC</cbc:TaxTypeCode>
      </cac:TaxScheme>
    </cac:TaxCategory>
  </cac:TaxSubtotal>
  <!-- IGV (base = valor venta + ISC) -->
  <cac:TaxSubtotal>
    <cbc:TaxableAmount currencyID="PEN">1100.00</cbc:TaxableAmount>
    <cbc:TaxAmount currencyID="PEN">198.00</cbc:TaxAmount>
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

---

## 22. Ejemplo - Factura de Exportacion

```xml
<cbc:InvoiceTypeCode listAgencyName="PE:SUNAT" listName="Tipo de Documento"
  listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01"
  listID="0200" name="Exportacion de Bienes">01</cbc:InvoiceTypeCode>

<!-- TaxTotal global -->
<cac:TaxTotal>
  <cbc:TaxAmount currencyID="USD">0.00</cbc:TaxAmount>
  <cac:TaxSubtotal>
    <cbc:TaxableAmount currencyID="USD">1000.00</cbc:TaxableAmount>
    <cbc:TaxAmount currencyID="USD">0.00</cbc:TaxAmount>
    <cac:TaxCategory>
      <cbc:ID schemeID="UN/ECE 5305" schemeName="Tax Category Identifier"
        schemeAgencyName="United Nations Economic Commission for Europe">G</cbc:ID>
      <cbc:Percent>0.00</cbc:Percent>
      <cbc:TaxExemptionReasonCode listAgencyName="PE:SUNAT"
        listName="SUNAT:Codigo de Tipo de Afectacion del IGV"
        listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo07">40</cbc:TaxExemptionReasonCode>
      <cac:TaxScheme>
        <cbc:ID schemeID="UN/ECE 5153" schemeName="Tax Scheme Identifier"
          schemeAgencyName="United Nations Economic Commission for Europe">9995</cbc:ID>
        <cbc:Name>EXP</cbc:Name>
        <cbc:TaxTypeCode>FRE</cbc:TaxTypeCode>
      </cac:TaxScheme>
    </cac:TaxCategory>
  </cac:TaxSubtotal>
</cac:TaxTotal>
```

---

## Anexo A - Resumen de Catalogos SUNAT Referenciados

| Catalogo | Descripcion | URI |
|----------|------------|-----|
| 01 | Tipo de Documento | `urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01` |
| 02 | Tipo de Moneda (ISO 4217) | - |
| 03 | Tipo de Unidad de Medida (UN/ECE rec 20) | - |
| 05 | Codigos de Tipos de Tributos | `urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo05` |
| 06 | Tipo de Documento de Identidad | `urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06` |
| 07 | Tipo de Afectacion del IGV | `urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo07` |
| 08 | Tipo de Sistema de Calculo del ISC | `urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo08` |
| 12 | Tipo de Documento Relacionado | `urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo12` |
| 13 | Ubigeo (INEI) | - |
| 16 | Tipo de Precio de Venta Unitario | `urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo16` |
| 51 | Tipo de Operacion | `urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo51` |
| 52 | Codigos de Leyendas | `urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo52` |
| 53 | Cargos o Descuentos | `urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo53` |
| 54 | Codigos de Bienes y Servicios sujetos a Detraccion | `urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo54` |
| 59 | Medios de Pago | `urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo59` |
| 61 | Tipo de Percepcion | - |
