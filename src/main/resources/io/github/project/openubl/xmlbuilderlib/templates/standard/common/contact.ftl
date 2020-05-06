<#macro macroContact contact>
            <cac:Contact>
            <#if contact.telefono??>
                <cbc:Telephone>${contact.telefono}</cbc:Telephone>
            </#if>
            <#if contact.email??>
                <cbc:ElectronicMail>${contact.email}</cbc:ElectronicMail>
            </#if>
            </cac:Contact>
</#macro>
