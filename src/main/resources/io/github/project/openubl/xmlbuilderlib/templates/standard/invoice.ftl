<@compress single_line=true>
<#setting number_format="computer">
<?xml version="1.0" encoding="ISO-8859-1"?>
<Invoice xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2"
        <#include "common/namespaces.ftl">
>
    <#include "common/ubl-extensions.ftl">
    <#include "common/general-data.ftl">
<#--    <#if fechaVencimiento??><cbc:DueDate>${fechaVencimiento}</cbc:DueDate></#if>-->
    <cbc:InvoiceTypeCode listID="0101" listAgencyName="PE:SUNAT" listName="Tipo de Documento" listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01">${tipoInvoice.code}</cbc:InvoiceTypeCode>
<#--    <#include "./common/legends.ftl">-->
    <cbc:DocumentCurrencyCode listID="ISO 4217 Alpha" listAgencyName="United Nations Economic Commission for Europe" listName="Currency">${moneda}</cbc:DocumentCurrencyCode>
<#--    <#if orderCompra??>-->
<#--    <cac:OrderReference>-->
<#--        <cbc:ID>${orderCompra}</cbc:ID>-->
<#--    </cac:OrderReference>-->
<#--    </#if>-->
    <#include "common/despatch-document-reference.ftl">
    <#include "common/additional-document-reference.ftl">
    <#list anticipos as item>
    <cac:AdditionalDocumentReference>
        <cbc:ID>${item.serieNumero}</cbc:ID>
        <cbc:DocumentTypeCode listAgencyName="PE:SUNAT" listName="Documento Relacionado" listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo12">${item.tipoDocumento.code}</cbc:DocumentTypeCode>
        <cbc:DocumentStatusCode listName="Anticipo" listAgencyName="PE:SUNAT">${item?index + 1}</cbc:DocumentStatusCode>
        <cac:IssuerParty>
            <cac:PartyIdentification>
                <cbc:ID schemeID="6" schemeName="Documento de Identidad" schemeAgencyName="PE:SUNAT" schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06">${proveedor.ruc}</cbc:ID>
            </cac:PartyIdentification>
        </cac:IssuerParty>
    </cac:AdditionalDocumentReference>
    </#list>
    <#include "../signature.ftl">
    <#include "common/supplier.ftl">
    <#include "common/customer.ftl">
<#--    <#if direccionDeEntrega>-->
<#--    <cac:Delivery>-->
<#--        <cac:DeliveryLocation>-->
<#--            <cac:Address>-->
<#--                <#import "./common/address.ftl" as adressMacro/>-->
<#--                <@adressMacro address=direccionDeEntrega/>-->
<#--            </cac:Address>-->
<#--        </cac:DeliveryLocation>-->
<#--    </cac:Delivery>-->
<#--    </#if>-->
<#--    <#if detraccion>-->
<#--    <cac:PaymentMeans>-->
<#--        <cbc:PaymentMeansCode schemeName="SUNAT:Codigo de medio de pago" schemeAgencyName="PE:SUNAT" schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo59">${detraccion.medioDePago.code}</cbc:PaymentMeansCode>-->
<#--        <#if detraccion.numeroCuentaBanco>-->
<#--        <cac:PayeeFinancialAccount>-->
<#--            <cbc:ID>${detraccion.numeroCuentaBanco}</cbc:ID>-->
<#--        </cac:PayeeFinancialAccount>-->
<#--        </#if>-->
<#--    </cac:PaymentMeans>-->
<#--    <cac:PaymentTerms>-->
<#--        <cbc:PaymentMeansID schemeName="SUNAT:Codigo de detraccion" schemeAgencyName="PE:SUNAT" schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo54">${detraccion.tipoBienServicio.code}</cbc:PaymentMeansID>-->
<#--        <cbc:PaymentPercent>${detraccion.porcentaje}</cbc:PaymentPercent>-->
<#--        <cbc:Amount currencyID="${moneda}">${detraccion.montoTotal}</cbc:Amount>-->
<#--    </cac:PaymentTerms>-->
<#--    </#if>-->
<#--    <#if percepcion>-->
<#--    <cac:PaymentTerms>-->
<#--        <cbc:ID schemeName="SUNAT:Codigo de detraccion" schemeAgencyName="PE:SUNAT" schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo54">${percepcion.tipoBienServicio.code}</cbc:ID>-->
<#--        <cbc:Amount currencyID="${moneda}">${percepcion.montoTotal}</cbc:Amount>-->
<#--    </cac:PaymentTerms>-->
    <cac:PaymentTerms>
        <cbc:ID>FormaPago</cbc:ID>
        <cbc:PaymentMeansID>${formaPago.tipo}</cbc:PaymentMeansID>
        <cbc:Amount currencyID="${moneda}">${formaPago.montoTotal}</cbc:Amount>
    </cac:PaymentTerms>
    <#list formaPago.cuotas as item>
    <cac:PaymentTerms>
        <cbc:ID>FormaPago</cbc:ID>
        <cbc:PaymentMeansID>Cuota${item.id}</cbc:PaymentMeansID>
        <cbc:Amount currencyID="${moneda}">${item.monto}</cbc:Amount>
        <cbc:PaymentDueDate>${item.fechaPago}</cbc:PaymentDueDate>
    </cac:PaymentTerms>
    </#list>
    <#list anticipos as item>
    <cac:PrepaidPayment>
        <cbc:ID>${item?index + 1}</cbc:ID>
        <cbc:PaidAmount currencyID="${moneda}">${item.montoTotal}</cbc:PaidAmount>
    </cac:PrepaidPayment>
    </#list>
    <#if totalAnticipos??>
    <cac:AllowanceCharge>
        <cbc:ChargeIndicator>false</cbc:ChargeIndicator>
        <cbc:AllowanceChargeReasonCode >04</cbc:AllowanceChargeReasonCode>
        <cbc:MultiplierFactorNumeric>1</cbc:MultiplierFactorNumeric>
        <cbc:Amount currencyID="${moneda}">${totalAnticipos}</cbc:Amount>
        <cbc:BaseAmount currencyID="${moneda}">${totalAnticipos}</cbc:BaseAmount>
    </cac:AllowanceCharge>
    </#if>
<#--    <#list cargos as item>-->
<#--    <cac:AllowanceCharge>-->
<#--        <cbc:ChargeIndicator>true</cbc:ChargeIndicator>-->
<#--        <cbc:AllowanceChargeReasonCode schemeName="SUNAT:Codigo de cargos o descuentos" schemeAgencyName="PE:SUNAT" schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo53">${item.tipo.code}</cbc:AllowanceChargeReasonCode>-->
<#--        <cbc:MultiplierFactorNumeric>${item.factor}</cbc:MultiplierFactorNumeric>-->
<#--        <cbc:Amount currencyID="${moneda}">${item.monto}</cbc:Amount>-->
<#--        <cbc:BaseAmount currencyID="${moneda}">${item.montoBase}</cbc:BaseAmount>-->
<#--    </cac:AllowanceCharge>-->
<#--    </#list>-->
<#--    <#list descuentos as item>-->
<#--    <cac:AllowanceCharge>-->
<#--        <cbc:ChargeIndicator>false</cbc:ChargeIndicator>-->
<#--        <cbc:AllowanceChargeReasonCode schemeName="SUNAT:Codigo de cargos o descuentos" schemeAgencyName="PE:SUNAT" schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo53">${item.tipo.code}</cbc:AllowanceChargeReasonCode>-->
<#--        <cbc:MultiplierFactorNumeric>${item.factor}</cbc:MultiplierFactorNumeric>-->
<#--        <cbc:Amount currencyID="${moneda}">${item.monto}</cbc:Amount>-->
<#--        <cbc:BaseAmount currencyID="${moneda}">${item.montoBase}</cbc:BaseAmount>-->
<#--    </cac:AllowanceCharge>-->
<#--    </#list>-->
<#--    <#if percepcion>-->
<#--    <cac:AllowanceCharge>-->
<#--        <cbc:ChargeIndicator>true</cbc:ChargeIndicator>-->
<#--        <cbc:AllowanceChargeReasonCode schemeName="SUNAT:Codigo de cargos o descuentos" schemeAgencyName="PE:SUNAT" schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo53">${percepcion.tipo.code}</cbc:AllowanceChargeReasonCode>-->
<#--        <cbc:MultiplierFactorNumeric>${percepcion.factor}</cbc:MultiplierFactorNumeric>-->
<#--        <cbc:Amount currencyID="${moneda}">${percepcion.monto}</cbc:Amount>-->
<#--        <cbc:BaseAmount currencyID="${moneda}">${percepcion.montoBase}</cbc:BaseAmount>-->
<#--    </cac:AllowanceCharge>-->
<#--    </#if>-->
    <#include "common/tax-total.ftl">
    <cac:LegalMonetaryTotal>
    <#include "common/monetary-total.ftl">
    </cac:LegalMonetaryTotal>
    <#list detalle as item>
    <cac:InvoiceLine>
        <cbc:ID>${item?index + 1}</cbc:ID>
        <cbc:InvoicedQuantity unitCode="${item.unidadMedida}" unitCodeListAgencyName="United Nations Economic Commission for Europe" unitCodeListID="UN/ECE rec 20">${item.cantidad}</cbc:InvoicedQuantity>
        <#include "common/document-line.ftl">
    </cac:InvoiceLine>
    </#list>
</Invoice>
</@compress>
