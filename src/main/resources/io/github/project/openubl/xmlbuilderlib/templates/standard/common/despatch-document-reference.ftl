    <#list guiasRemisionRelacionadas as item>
    <cac:DespatchDocumentReference>
        <cbc:ID>${item.serieNumero}</cbc:ID>
        <cbc:DocumentTypeCode listAgencyName="PE:SUNAT" listName="Tipo de Documento" listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01">${item.tipoDocumento.code}</cbc:DocumentTypeCode>
    </cac:DespatchDocumentReference>
    </#list>
