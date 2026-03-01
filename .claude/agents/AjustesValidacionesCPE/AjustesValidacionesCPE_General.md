# General


| VALIDACIÓN / CONDICIÓN | TIPO DE RETORNO | CODIGO<br> RETORNO | DESCIPCIÖN DE CÓDIGO DE RETORNO | LISTADOS |
| --- | --- | --- | --- | --- |
| Problema con el servicio de recepción de comprobantes | ERROR | 0100 | El sistema no puede responder su solicitud. Intente nuevamente o comuníquese con su Administrador | - |
| Problema con la autenticación del servicio (usuario y contraseña con los que se invoca el servicio) | ERROR | 0109 | El sistema no puede responder su solicitud. (El servicio de autenticación no está disponible) | - |
| El usuario que invoca el servicio no es emisor ni PSE | ERROR | 0111 | No tiene el perfil para enviar comprobantes electronicos | Contribuyentes asociados a los emisores |
| El nombre del archivo enviado no tiene la estructura RRRRRRRRRRR-TT-AAAAMMDD-NNNN.zip o RRRRRRRRRRR-TT-AAAAMMDD-NNNNN.ZIP<br>Donde: RRRRRRRRRR: Número de RUC, TT: Tipo de archivo (RA, RC o RR), AAAAMMDD: Fecha, NNNNN: Número correlativo del archivo<br>Nota: <br>* En el campo 'Número correlativo del archivo' espera un valor mínimo de 1 y máximo de 99999.<br>* El campo 'Número de RUC' espera que el valor inicie con 1 o 2. | ERROR | 0151 | El nombre del archivo ZIP es incorrecto | - |
| El usuario que invoca el servicio es diferente al RUC del archivo (emisor) y no existe relación vigente entre el usuario que invoca el servicio y el RUC del archivo (relación PSE).<br>La relación PSE y emisor, se considera vigente hasta el 7mo día calendario del mes siguiente de la revocación de la autorización (es decir, la  vigencia es hasta el 7mo dia calendario del mes siguiente a la fecha indicada en el campo fec_fin del listado) | ERROR | 0154 | El RUC del archivo no corresponde al RUC del usuario o el proveedor no esta autorizado a enviar comprobantes del contribuyente | Contribuyentes asociados a los emisores |
| El archivo ZIP esta vacío | ERROR | 0155 | El archivo ZIP esta vacio | - |
| El archivo ZIP esta corrupto | ERROR | 0156 | El archivo ZIP esta corrupto | - |
| El archivo ZIP no tiene archivos | ERROR | 0157 | El archivo ZIP no contiene comprobantes | - |
| El archivo ZIP tiene más de un archivo | ERROR | 0158 | El archivo ZIP contiene demasiados comprobantes para este tipo de envío | - |
| El nombre del archivo XML no tiene la estructura RRRRRRRRRRR-01-SSSS-NNNNNNNN.xml o RRRRRRRRRRR-01-SSSS-NNNNNNNN.XML<br>(Donde: RRRRRRRRRR: RUC, SSSS: Serie, NNNNNNNN: Número) | ERROR | 0159 | El nombre del archivo XML es incorrecto | - |
| El archivo XML esta vacío | ERROR | 0160 | El archivo XML esta vacio | - |
| El nombre del archivo XML no coincide con el nombre del archivo ZIP | ERROR | 0161 | El nombre del archivo XML no coincide con el nombre del archivo ZIP | - |
| No se puede leer (parsear) el archivo XML, incluye validación de XSD | ERROR | 0306 | No se puede leer (parsear) el archivo XML | - |
| ID del certificado del comprobante no corresponde con el ID del certificado del contribuyente o del PSE al que está afiliado el contribuyente | ERROR | 2325 | El certificado usado no es el comunicado a SUNAT | Certificados del emisor |
| El certificado del contribuyente (RUC que invoca el servicio) del listado tiene fecha de baja menor a la fecha de emisión del comprobante | ERROR | 2326 | El certificado usado se encuentra de baja | Certificados del emisor |
| El emisor del comprobante envía desde SEE-Desde los sistemas del contribuyente y no se encuentra autorizado a utilizar este sistema de emisión<br>Nota: No aplica para SEE-OSE | ERROR | 1078 | El emisor no se encuentra autorizado a emitir en el SEE-Desde los sistemas del contribuyente | - |
| La firma no coincide con el comprobante | ERROR | 2335 | El documento electrónico ingresado ha sido alterado | - |
