    <#list leyendas as item>
        <cbc:Note languageLocaleID="item.code"><![CDATA[${item.label}]]></cbc:Note>
    </#list>
    <#if observacion??><cbc:Note><![CDATA[${observacion}]]></cbc:Note></#if>

