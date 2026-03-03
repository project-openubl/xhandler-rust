# Parámetros


| cod_parametro | cod_argumento | cod_argumento | cod_argumento | des_argumento | des_argumento | des_argumento |
| --- | --- | --- | --- | --- | --- | --- |
| cod_parametro | Descripción | Tipo | Formato | Descripción | Tipo | Formato |
| 001: Tipo de cambio | Moneda+Fecha de cambio | an14 | XXX-YYYYMMDD<br>Donde: XXX es moneda<br>YYYYMMDD es fecha de cambio | Monto del tipo de cambio | n..8 | (5 enteros, 3 decimales) |
| 002: Regimen de percepción | Código de tipo de regimen de percepción | n2 |  | Porcentaje de la percepción | n..8 | (5 enteros, 3 decimales) |
| 003: Regimen de retención | Código de tipo de regimen de retención | n2 |  | Porcentaje de la retención | n..8 | (5 enteros, 3 decimales) |
| 004: Plazo máximo de envío | Código de comprobante | n2 |  | Número de días | n..3 | (3 enteros) |
| 005: Catálogo 5 | Código de tipos de tributos | n4 |  | Código internacional + Categoría de impuesto |  | XXX-Y<br>Donde: XXX: es código internacional<br>Y: es categoría de impuesto |
| 006: Catálogo 6 | Código de tipo de documento de identidad | an1 |  | Descripción del tipo de documento de identidad | an..100 |  |
| 007: Catálogo 7 | Código de tipo de afectación del IGV | n2 |  | Descripción del tipo de afectación del IGV | an..100 |  |
| 008: Catálogo 8 | Código de tipos de sistema de cálculo del ISC | n2 |  | Descripción de tipos de sistemas de cálculo del ISC | an..100 |  |
| 009: Catálogo 14 | Código de otros conceptos tributarios | n4 |  | Descripción de otros conceptos tributarios | an..100 |  |
| 010: Catálogo 16 | Código de tipo de precio de venta unitario | n2 |  | Descripción de tipo de precio de venta unitario | an..100 |  |
| 011: Catálogo 17 | Código de tipo de operación | n2 |  | Descripción de tipo de operación | an..100 |  |
| 012: Tasa IGV | Fecha de inicio de tasa IGV | n8 | YYYYMMDD | Tasa de IGV | n..8 | (5 enteros, 3 decimales) |
| 013: Catálogo 9 | Código de tipo de nota de crédito | n2 |  | Descripción de tipo de nota de crédito | an..100 |  |
| 014: Catálogo 10 | Código de tipo de nota de débito | n2 |  | Descripción de tipo de nota de débito | an..100 |  |
| 015: Catálogo 1 | Código de tipo de documento | n2 |  | Descripción de tipo de documento | an..100 |  |
| 016: Catálogo 13 | Código de ubigeo | n6 |  | Descripción de ubigeo | an..100 |  |
| 017: Catálogo 11 | Códigos de tipo de valor de venta (resumen diario) | n2 |  | Descripción de tipo de valor de venta | an..100 |  |
| 018: Catálogo 19 | Código de estado del ítem (resumen diario) | n1 |  | Descripción de estado del ítem | an..100 |  |
| 019: Catálogo 22 | Código de regimen de percepciones | n2 |  | Descripción de regimen de percepciones | an..100 |  |
| 020: Catálogo 21 | Código de documentos relacionados (sólo guía de remisión electrónica) | n2 |  | Descripción de documentos relacionados | an..100 |  |
| 021: Catálogo 20 | Código de motivo de traslado | n2 |  | Descripción de motivo de traslado | an..100 |  |
| 022: Catálogo 18 | Código de modalidad de transporte | n2 |  | Descripción de modalidad de transporte | an..100 |  |
| 023: Catálogo 23 | Código de regimen de retenciones | n2 |  | Tasa de retenciones | n..8 | (5 enteros, 3 decimales) |
| 024: Tasa Vigente del IVAP | Fecha | an14 | YYYYMMDD<br>Donde: YYYYMMDD es fecha. | Tasa Vigente | n..8 | (5 enteros, 3 decimales) |
