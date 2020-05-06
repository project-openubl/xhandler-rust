        <#--PUNTO1-->
        <#---------->
        <#--TOTAL VALOR DE VENTA-->
        <#--A través de este elemento se debe indicar el valor de venta total de la operación. Es decir-->
        <#--el importe total de la venta sin considerar los descuentos, impuestos u otros tributos a que-->
        <#--se refiere el numeral anterior, pero que incluye cualquier monto de redondeo aplicable.-->
        <cbc:LineExtensionAmount currencyID="${moneda}">${totales.valorVentaSinImpuestos}</cbc:LineExtensionAmount>
        <#---------->
        <#--PUNTO2-->
        <#---------->
        <#--TOTAL PRECIO DE VENTA-->
        <#--A través de este elemento se debe indicar el valor de venta total de la operación incluido-->
        <#--los impuestos.-->
        <cbc:TaxInclusiveAmount currencyID="${moneda}">${totales.valorVentaConImpuestos}</cbc:TaxInclusiveAmount>
        <#---------->
        <#--PUNTO3-->
        <#---------->
        <#--TOTAL DE DESCUENTOS-->
        <#--A través de este elemento se debe indicar el valor total de los descuentos globales-->
        <#--realizados de ser el caso.-->
        <#--Su propósito es permitir consignar en el comprobante de pago:-->
        <#--* La sumatoria de los descuentos de cada línea (descuentos por ítem), ó-->
        <#--* La sumatoria de los descuentos de cada línea (ítem) + descuentos globales-->
<#--        <#if totalDescuentos??><cbc:AllowanceTotalAmount currencyID="${moneda}">${totalDescuentos}</cbc:AllowanceTotalAmount></#if>-->
        <#---------->
        <#--PUNTO4-->
        <#---------->
        <#--SUMATORIA OTROS CARGOS-->
        <#--Corresponde al total de otros cargos cobrados al adquirente o usuario y que no forman-->
        <#--parte de la operación que se factura, es decir no forman parte del(os) valor(es) de ventas-->
        <#--señaladas anteriormente, pero sí forman parte del importe total de la Venta (Ejemplo:-->
        <#--propinas, garantías para devolución de envases, etc.)-->
<#--        <#if totalOtrosCargos??><cbc:ChargeTotalAmount currencyID="${moneda}">${totalOtrosCargos}</cbc:ChargeTotalAmount></#if>-->
<#--        <#if totalOtrosCargos??><cbc:PrepaidAmount currencyID="${moneda}">${totalAnticipos}</cbc:PrepaidAmount></#if>-->
        <#---------->
        <#--PUNTO5-->
        <#---------->
        <#--IMPORTE TOTAL DE LA VENTA, DE LA CESION EN USO O DEL SERVICIO PRESTADO-->
        <#--Corresponde al importe total de la venta, de la cesión en uso o del servicio prestado. Es el-->
        <#--resultado de la suma y/o resta (Según corresponda) de los siguientes puntos: PUNTO2-PUNTO3+PUNTO4-->
        <#--menos los anticipos que hubieran sido recibidos.-->
        <cbc:PayableAmount currencyID="${moneda}">${totales.importeTotal}</cbc:PayableAmount>
