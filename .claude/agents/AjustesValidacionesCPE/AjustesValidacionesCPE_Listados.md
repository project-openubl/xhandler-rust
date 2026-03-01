# Listados


| Listado de contribuyentes | Listado de contribuyentes | Listado de contribuyentes | Listado de contribuyentes | Listado de contribuyentes |
| --- | --- | --- | --- | --- |
| Alcance: | Todo los contribuyentes | Todo los contribuyentes | Todo los contribuyentes | Todo los contribuyentes |
| Campo | Descripción | PK | Tipo | Formato |
| num_ruc | Numero del RUC del contribuyente | Si | n11 |  |
| ind_estado | Indicador de estado del contribuyente | No | n2 |  |
| ind_condicion | Indicador de condición del domicilio fiscal | No | n2 |  |
|  |  |  |  |  |
|  |  |  |  |  |
| Listado de los padrones de los contribuyentes | Listado de los padrones de los contribuyentes | Listado de los padrones de los contribuyentes | Listado de los padrones de los contribuyentes | Listado de los padrones de los contribuyentes |
| Alcance: | Todo los contribuyentes | Todo los contribuyentes | Todo los contribuyentes | Todo los contribuyentes |
| Campo | Descripción | PK | Tipo | Formato |
| num_ruc | Numero del RUC del contribuyente | Si | n11 |  |
| ind_padron | Indicador del padrón del contribuyente | SI | n2 | 01: Agente de percepción de ventas internas<br>02: Agente de percepción de combustibles<br>03: Agente de retención<br>04: Exceptuada de la percepción<br>05: Exportador de Servicios<br>10: Buen contribuyente<br>11: Autorizado a versión UBL 2.0<br>12: Obligado a enviar código de producto<br>13: Afiliados al SEE-Empresas supervisadas<br>14:  Inscrito en el Registro de Establecimientos Autorizados (REA) |
|  |  |  |  |  |
|  |  |  |  |  |
| Listado de contribuyentes asociados a los emisores | Listado de contribuyentes asociados a los emisores | Listado de contribuyentes asociados a los emisores | Listado de contribuyentes asociados a los emisores | Listado de contribuyentes asociados a los emisores |
| Alcance: | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE |
| Campo | Descripción | PK | Tipo | Observaciones |
| num_ruc | Número de RUC del emisor | Si | n11 |  |
| num_ruc_asociado | Número de RUC del asociado | Si | n11 |  |
| ind_tip_asociacion | Indicador de tipo de asociación | Si | n1 | 1: PSE<br>2: OSE |
| fec_inicio | Fecha de inicio | Si | an10 | YYYY-MM-DD |
| fec_fin | Fecha de fin (fecha de solicitud de la baja) | No | an10 | YYYY-MM-DD |
|  |  |  |  |  |
|  |  |  |  |  |
| Listado de certificados del emisor | Listado de certificados del emisor | Listado de certificados del emisor | Listado de certificados del emisor | Listado de certificados del emisor |
| Alcance: | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE |
| Campo | Descripción | PK | Tipo | Observaciones |
| num_ruc | Número de RUC del emisor | Si | n11 |  |
| num_id_ca | Número del ID del CA | Si | n10 |  |
| num_id_cd | Número del ID de la serie del certificado digital | Si | an..100 |  |
| fec_alta | Fecha de alta | Si | an25 | YYYY-MM-DD HH:MM:SS.nnnnn |
| fec_baja | Fecha de baja | No | an25 | YYYY-MM-DD HH:MM:SS.nnnnn |
|  |  |  |  |  |
|  |  |  |  |  |
| Listado de comprobantes de pago electrónicos | Listado de comprobantes de pago electrónicos | Listado de comprobantes de pago electrónicos | Listado de comprobantes de pago electrónicos | Listado de comprobantes de pago electrónicos |
| Alcance: | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE |
| Campo | Descripción | PK | Tipo | Observaciones |
| num_ruc | Numero de RUC del emisor | Si | n11 |  |
| cod_cpe | Código de tipo de comprobante | Si | n2 |  |
| num_serie_cpe | Numero de serie del comprobante | Si | an4 |  |
| num_cpe | Numero del comprobante | Si | n..8 |  |
| ind_estado_cpe | Indicador de estado del comprobante | No | n1 | 2: Anulado<br>1: Aceptado<br>0: Rechazado |
| fec_emision_cpe | Fecha y hora de emisión del comprobante | No | an25 | YYYY-MM-DD HH:MM:SS.nnnnn |
| mto_importe_cpe | Monto del importe total | No | n..23 | Para mantener la estructura del archivo, se enviará con cero (0) este campo |
| cod_moneda_cpe | Codigo de moneda del comprobante | No | an3 | Para mantener la estructura del archivo, se enviará con valor vacío este campo |
| cod_mot_traslado | Código de motivo de traslado | No | n2 | Información exclusiva si el comprobante es guía de remisión. |
| cod_mod_traslado | Código de modalidad de traslado | No | n2 | Información exclusiva si el comprobante es guía de remisión. |
| ind_transbordo | Indicador de transbordo programado | No | n1 | Información exclusiva si el comprobante es guía de remisión.<br>1:  Con transbordo programado<br>0: Sin transbordo programado |
| fec_ini_traslado | Fecha de inicio de traslado | No | n1 | Información exclusiva si el comprobante es guía de remisión. |
| ind_for_pag | Indicador de forma de pago | No | n1 | Información exclusiva si el comprobante es factura. <br>0: Contado <br>1: Crédito |
| Ind_percepcion | Indicador de percepción | No | n1 | Información exclusiva si el comprobante es factura o boleta (envío individual)<br>0: No tiene percepción <br>1: Si tiene percepción |
|  |  |  |  |  |
|  |  |  |  |  |
| Listado de autorizaciones de comprobantes de pago físicos | Listado de autorizaciones de comprobantes de pago físicos | Listado de autorizaciones de comprobantes de pago físicos | Listado de autorizaciones de comprobantes de pago físicos | Listado de autorizaciones de comprobantes de pago físicos |
| Alcance: | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE |
| Campo | Descripción | PK | Tipo | Observaciones |
| num_ruc | Número de RUC del emisor | Si | n11 |  |
| cod_cpe | Código de tipo de comprobante | Si | n2 |  |
| num_serie_cpe | Número de serie del comprobante | Si | n4 |  |
| num_ini_cpe | Número de inicio del comprobante | Si | n8 |  |
| num_fin_cpe | Número de fin del comprobante | No | n8 |  |
|  |  |  |  |  |
|  |  |  |  |  |
| Listado de autorizaciones de rangos de contingencia | Listado de autorizaciones de rangos de contingencia | Listado de autorizaciones de rangos de contingencia | Listado de autorizaciones de rangos de contingencia | Listado de autorizaciones de rangos de contingencia |
| Alcance: | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE |
| Campo | Descripción | PK | Tipo | Formato |
| num_ruc | Número de RUC del emisor | Si | n11 |  |
| cod_cpe | Código de tipo de comprobante | Si | n2 |  |
| num_serie_cpe | Número de serie del comprobante | Si | n4 |  |
| num_ini_cpe | Número de inicio del comprobante | Si | n8 |  |
| num_fin_cpe | Número de fin del comprobante | No | n8 |  |
|  |  |  |  |  |
|  |  |  |  |  |
|  |  |  |  |  |
| Parámetros | Parámetros | Parámetros | Parámetros | Parámetros |
| Alcance: | Para todos los OSEs | Para todos los OSEs | Para todos los OSEs | Para todos los OSEs |
| Campo | Descripción | PK | Tipo | Observaciones |
| cod_parametro | Código de parámetro | Si | n3 | 001: Tipo de cambio<br>002: Regimen de percepción<br>003: Regimen de retención |
| cod_argumento | Código de argumento | Si | an..25 | Ver hoja de parámetros |
| des_argumento | Descripción del argumento | No | an..100 | Ver hoja de parámetros |
|  |  |  |  |  |
|  |  |  |  |  |
| Listado de Establecimientos Anexos | Listado de Establecimientos Anexos | Listado de Establecimientos Anexos | Listado de Establecimientos Anexos | Listado de Establecimientos Anexos |
| Alcance: | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE |
| Campo | Descripción | PK | Tipo | Observaciones |
| num_ruc | Numero de RUC del emisor | Si | n11 |  |
| cod_estab | Código de establecimiento anexo | Si | n4 |  |
| cod_tip_estab | Tipo de establecimiento anexo | No | n2 |  |
|  |  |  |  |  |
|  |  |  |  |  |
| Listado de padrones con vigencia | Listado de padrones con vigencia | Listado de padrones con vigencia | Listado de padrones con vigencia | Listado de padrones con vigencia |
| Alcance: | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE | De los contribuyentes asociados al OSE |
| Campo | Descripción | PK | Tipo | Observaciones |
| num_ruc | Numero del RUC del contribuyente | Si | n11 |  |
| ind_padron | Tipo de padrón | Si | n2 | 01: Autorizado a IGV 10%<br>02: Sujetos sin capacidad operativa (SSCO) |
| fec_inivig | Fecha de inicio de vigencia | Si | an10 | YYYY-MM-DD |
| fec_finvig | Fecha de fin de vigencia | No | an10 | YYYY-MM-DD |
