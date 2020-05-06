    <cac:TaxTotal>
        <cbc:TaxAmount currencyID="${moneda}">${impuestos.importeTotal}</cbc:TaxAmount>
        <#if impuestos.gravadas??>
        <cac:TaxSubtotal>
            <cbc:TaxableAmount currencyID="${moneda}">${impuestos.gravadas.baseImponible}</cbc:TaxableAmount>
            <cbc:TaxAmount currencyID="${moneda}">${impuestos.gravadas.importe}</cbc:TaxAmount>
            <cac:TaxCategory>
                <cbc:ID schemeAgencyName="United Nations Economic Commission for Europe" schemeID="UN/ECE 5305" schemeName="Tax Category Identifie">${impuestos.gravadas.categoria.categoria}</cbc:ID>
                <cac:TaxScheme>
                    <cbc:ID schemeAgencyName="PE:SUNAT" schemeID="UN/ECE 5153" schemeName="Codigo de tributos">${impuestos.gravadas.categoria.code}</cbc:ID>
                    <cbc:Name>${impuestos.gravadas.categoria.nombre}</cbc:Name>
                    <cbc:TaxTypeCode>${impuestos.gravadas.categoria.tipo}</cbc:TaxTypeCode>
                </cac:TaxScheme>
            </cac:TaxCategory>
        </cac:TaxSubtotal>
        </#if>
        <#if impuestos.inafectas??>
        <cac:TaxSubtotal>
            <cbc:TaxableAmount currencyID="${moneda}">${impuestos.inafectas.baseImponible}</cbc:TaxableAmount>
            <cbc:TaxAmount currencyID="${moneda}">${impuestos.inafectas.importe}</cbc:TaxAmount>
            <cac:TaxCategory>
                <cbc:ID schemeAgencyName="United Nations Economic Commission for Europe" schemeID="UN/ECE 5305" schemeName="Tax Category Identifie">${impuestos.inafectas.categoria.categoria}</cbc:ID>
                <cac:TaxScheme>
                    <cbc:ID schemeAgencyName="PE:SUNAT" schemeID="UN/ECE 5153" schemeName="Codigo de tributos">${impuestos.inafectas.categoria.code}</cbc:ID>
                    <cbc:Name>${impuestos.inafectas.categoria.nombre}</cbc:Name>
                    <cbc:TaxTypeCode>${impuestos.inafectas.categoria.tipo}</cbc:TaxTypeCode>
                </cac:TaxScheme>
            </cac:TaxCategory>
        </cac:TaxSubtotal>
        </#if>
        <#if impuestos.exoneradas??>
        <cac:TaxSubtotal>
            <cbc:TaxableAmount currencyID="${moneda}">${impuestos.exoneradas.baseImponible}</cbc:TaxableAmount>
            <cbc:TaxAmount currencyID="${moneda}">${impuestos.exoneradas.importe}</cbc:TaxAmount>
            <cac:TaxCategory>
                <cbc:ID schemeAgencyName="United Nations Economic Commission for Europe" schemeID="UN/ECE 5305" schemeName="Tax Category Identifie">${impuestos.exoneradas.categoria.categoria}</cbc:ID>
                <cac:TaxScheme>
                    <cbc:ID schemeAgencyName="PE:SUNAT" schemeID="UN/ECE 5153" schemeName="Codigo de tributos">${impuestos.exoneradas.categoria.code}</cbc:ID>
                    <cbc:Name>${impuestos.exoneradas.categoria.nombre}</cbc:Name>
                    <cbc:TaxTypeCode>${impuestos.exoneradas.categoria.tipo}</cbc:TaxTypeCode>
                </cac:TaxScheme>
            </cac:TaxCategory>
        </cac:TaxSubtotal>
        </#if>
        <#if impuestos.gratuitas??>
        <cac:TaxSubtotal>
            <cbc:TaxableAmount currencyID="${moneda}">${impuestos.gratuitas.baseImponible}</cbc:TaxableAmount>
            <cbc:TaxAmount currencyID="${moneda}">${impuestos.gratuitas.importe}</cbc:TaxAmount>
            <cac:TaxCategory>
                <cbc:ID schemeAgencyName="United Nations Economic Commission for Europe" schemeID="UN/ECE 5305" schemeName="Tax Category Identifie">${impuestos.gratuitas.categoria.categoria}</cbc:ID>
                <cac:TaxScheme>
                    <cbc:ID schemeAgencyName="PE:SUNAT" schemeID="UN/ECE 5153" schemeName="Codigo de tributos">${impuestos.gratuitas.categoria.code}</cbc:ID>
                    <cbc:Name>${impuestos.gratuitas.categoria.nombre}</cbc:Name>
                    <cbc:TaxTypeCode>${impuestos.gratuitas.categoria.tipo}</cbc:TaxTypeCode>
                </cac:TaxScheme>
            </cac:TaxCategory>
        </cac:TaxSubtotal>
        </#if>
        <#if impuestos.ivap??>
        <cac:TaxSubtotal>
            <cbc:TaxableAmount currencyID="${moneda}">${impuestos.ivap.baseImponible}</cbc:TaxableAmount>
            <cbc:TaxAmount currencyID="${moneda}">${impuestos.ivap.importe}</cbc:TaxAmount>
            <cac:TaxCategory>
                <cbc:ID schemeAgencyName="United Nations Economic Commission for Europe" schemeID="UN/ECE 5305" schemeName="Tax Category Identifie">${impuestos.ivap.categoria.categoria}</cbc:ID>
                <cac:TaxScheme>
                    <cbc:ID schemeAgencyName="PE:SUNAT" schemeID="UN/ECE 5153" schemeName="Codigo de tributos">${impuestos.ivap.categoria.code}</cbc:ID>
                    <cbc:Name>${impuestos.ivap.categoria.nombre}</cbc:Name>
                    <cbc:TaxTypeCode>${impuestos.ivap.categoria.tipo}</cbc:TaxTypeCode>
                </cac:TaxScheme>
            </cac:TaxCategory>
        </cac:TaxSubtotal>
        </#if>
        <#if impuestos.icb??>
        <cac:TaxSubtotal>
            <cbc:TaxAmount currencyID="${moneda}">${impuestos.icb.importe}</cbc:TaxAmount>
            <cac:TaxCategory>
                <cbc:ID schemeAgencyName="United Nations Economic Commission for Europe" schemeID="UN/ECE 5305" schemeName="Tax Category Identifie">${impuestos.icb.categoria.categoria}</cbc:ID>
                <cac:TaxScheme>
                    <cbc:ID schemeAgencyName="PE:SUNAT" schemeID="UN/ECE 5153" schemeName="Codigo de tributos">${impuestos.icb.categoria.code}</cbc:ID>
                    <cbc:Name>${impuestos.icb.categoria.nombre}</cbc:Name>
                    <cbc:TaxTypeCode>${impuestos.icb.categoria.tipo}</cbc:TaxTypeCode>
                </cac:TaxScheme>
            </cac:TaxCategory>
        </cac:TaxSubtotal>
        </#if>
<#--        <#list totalImpuestosIcb as impuesto>-->
<#--            <cac:TaxSubtotal>-->
<#--                <cbc:TaxAmount currencyID="${moneda}">${impuesto.importe}</cbc:TaxAmount>-->
<#--                <cac:TaxCategory>-->
<#--                    <cbc:ID schemeAgencyName="United Nations Economic Commission for Europe" schemeID="UN/ECE 5305" schemeName="Tax Category Identifie">${impuesto.categoria.categoria}</cbc:ID>-->
<#--                    <cac:TaxScheme>-->
<#--                        <cbc:ID schemeAgencyName="PE:SUNAT" schemeID="UN/ECE 5153" schemeName="Codigo de tributos">${impuesto.categoria.code}</cbc:ID>-->
<#--                        <cbc:Name>${impuesto.categoria.nombre}</cbc:Name>-->
<#--                        <cbc:TaxTypeCode>${impuesto.categoria.tipo}</cbc:TaxTypeCode>-->
<#--                    </cac:TaxScheme>-->
<#--                </cac:TaxCategory>-->
<#--            </cac:TaxSubtotal>-->
<#--        </#list>-->
    </cac:TaxTotal>
