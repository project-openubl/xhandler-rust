<#macro macroAddress address>
                <#if address.ubigeo??><cbc:ID>${address.ubigeo}</cbc:ID></#if>
                <#if address.codigoLocal??><cbc:AddressTypeCode>${address.codigoLocal}</cbc:AddressTypeCode></#if>
                <#if address.urbanizacion??><cbc:CitySubdivisionName>${address.urbanizacion}</cbc:CitySubdivisionName></#if>
                <#if address.provincia??><cbc:CityName>${address.provincia}</cbc:CityName></#if>
                <#if address.departamento??><cbc:CountrySubentity>${address.departamento}</cbc:CountrySubentity></#if>
                <#if address.distrito??><cbc:District>${address.distrito}</cbc:District></#if>
                <#if address.direccion??>
                <cac:AddressLine>
                    <cbc:Line><![CDATA[${address.direccion}]]></cbc:Line>
                </cac:AddressLine>
                </#if>
                <#if address.codigoPais??>
                <cac:Country>
                    <cbc:IdentificationCode>${address.codigoPais}</cbc:IdentificationCode>
                </cac:Country>
                </#if>
</#macro>
