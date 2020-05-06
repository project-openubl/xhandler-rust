    <#list otrosDocumentosTributariosRelacionados as item>
    <cac:AdditionalDocumentReference>
        <cbc:ID>${item.serieNumero}</cbc:ID>
        <cbc:DocumentTypeCode listAgencyName="PE:SUNAT" listName="SUNAT: Identificador de documento relacionado" listURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo12">${item.tipoDocumento.code}</cbc:DocumentTypeCode>
    </cac:AdditionalDocumentReference>
    </#list>
