    <#import "address.ftl" as adressMacro/>
    <#import "contact.ftl" as contactMacro/>
    <cac:AccountingCustomerParty>
        <cac:Party>
            <cac:PartyIdentification>
                <cbc:ID schemeID="${cliente.tipoDocumentoIdentidad.code}" schemeAgencyName="PE:SUNAT" schemeName="SUNAT:Identificador de Documento de Identidad" schemeURI="urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06">${cliente.numeroDocumentoIdentidad}</cbc:ID>
            </cac:PartyIdentification>
            <cac:PartyLegalEntity>
                <cbc:RegistrationName><![CDATA[${cliente.nombre}]]></cbc:RegistrationName>
                <#if cliente.direccion??>
                <cac:RegistrationAddress>
                    <@adressMacro.macroAddress address=cliente.direccion />
                </cac:RegistrationAddress>
                </#if>
            </cac:PartyLegalEntity>
            <#if cliente.contacto??>
                <@contactMacro.macroContact contact=cliente.contacto />
            </#if>
        </cac:Party>
    </cac:AccountingCustomerParty>
