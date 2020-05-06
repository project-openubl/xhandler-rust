    <cac:DiscrepancyResponse>
        <cbc:ReferenceID>${serieNumeroComprobanteAfectado}</cbc:ReferenceID>
        <cbc:ResponseCode listAgencyName="PE:SUNAT" listName="SUNAT: Identificador de tipo de nota de debito" listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo10">${tipoDocumentoComprobanteAfectado.code}</cbc:ResponseCode>
        <cbc:Description><![CDATA[${descripcionSustentoDeNota}]]></cbc:Description>
    </cac:DiscrepancyResponse>
<#--    <#if orderCompra??>-->
<#--        <cac:OrderReference>-->
<#--            <cbc:ID>${orderCompra}</cbc:ID>-->
<#--        </cac:OrderReference>-->
<#--    </#if>-->
    <cac:BillingReference>
        <cac:InvoiceDocumentReference>
            <cbc:ID>${serieNumeroComprobanteAfectado}</cbc:ID>
            <cbc:DocumentTypeCode listAgencyName="PE:SUNAT" listName="SUNAT:Identificador de Tipo de Documento" listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01">${tipoDocumentoComprobanteAfectado.code}</cbc:DocumentTypeCode>
        </cac:InvoiceDocumentReference>
    </cac:BillingReference>
