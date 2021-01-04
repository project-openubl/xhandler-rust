    <#import "address.ftl" as adressMacro/>
    <#import "contact.ftl" as contactMacro/>
    <cac:AccountingSupplierParty>
        <cac:Party>
            <cac:PartyIdentification>
                <cbc:ID schemeID="6" schemeAgencyName="PE:SUNAT" schemeName="Documento de Identidad" schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06">${proveedor.ruc}</cbc:ID>
            </cac:PartyIdentification>
            <#if proveedor.nombreComercial??>
            <cac:PartyName>
                <cbc:Name>${proveedor.nombreComercial}</cbc:Name>
            </cac:PartyName>
            </#if>
            <cac:PartyLegalEntity>
                <cbc:RegistrationName><![CDATA[${proveedor.razonSocial}]]></cbc:RegistrationName>
                <#if proveedor.direccion??>
                <cac:RegistrationAddress>
                    <@adressMacro.macroAddress address=proveedor.direccion />
                </cac:RegistrationAddress>
                </#if>
            </cac:PartyLegalEntity>
            <#if proveedor.contacto??>
                <@contactMacro.macroContact contact=proveedor.contacto />
            </#if>
        </cac:Party>
    </cac:AccountingSupplierParty>
