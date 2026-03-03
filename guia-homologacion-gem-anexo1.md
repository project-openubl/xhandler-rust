# Guia de Homologacion GEM - Anexo N 1

## Cuadro de Especificacion de los Casos de Prueba

Cuadro con descripcion de los casos de prueba para Facturas, Boletas de Venta y sus Notas.

---

## Descripcion de Columnas

| Columna | Descripcion |
|---------|------------|
| **NUM CASO** | Numero identificador del caso |
| **DESCRIPCION** | Descripcion del caso de prueba |
| **COD GRUPO** | Codigo del grupo (ver tabla de grupos abajo) |
| **TIPO COMP** | Tipo de comprobante: 1=Factura, 3=Boleta, 7=Nota de credito, 8=Nota de debito |
| **SERIE** | Serie del comprobante |
| **CANTIDAD ITEMS/LINEAS** | El numero de items por comprobante |
| **NUMERO CASO AFECTADO** | Numero de caso afectado (para Notas de credito y debito que afectan facturas y boletas) |
| **REP. IMPRESA** | 0=no se requiere representacion impresa PDF, 1=si se requiere |

---

## Tabla de Grupos

| Grupo | Tipo de Documento | Serie | Descripcion |
|-------|-------------------|-------|-------------|
| 1 | Facturas y Notas | FF11 | Ventas Gravadas IGV |
| 2 | Facturas y Notas | FF12 | Ventas inafectas y/o exoneradas |
| 3 | Facturas y Notas | FF13 | Ventas gratuitas |
| 4 | Facturas y Notas | FF14 | Ventas con descuento global |
| 5 | Facturas y Notas | FF30 | Operaciones gravadas con ISC |
| 6 | Facturas y Notas | FF40 | Operaciones con percepcion |
| 7 | Facturas y Notas | FF50 | Operaciones con otro tipo de moneda |
| 8 | Boletas y Notas | BB11 | Ventas Gravadas IGV |
| 9 | Boletas y Notas | BB12 | Ventas inafectas y/o exoneradas |
| 10 | Boletas y Notas | BB13 | Ventas gratuitas |
| 11 | Boletas y Notas | BB14 | Ventas con descuento global |
| 12 | Boletas y Notas | BB50 | Operaciones con otro tipo de moneda |
| 13 | Resumenes Diarios | - | Resumen diario de Boletas |
| 14 | Comunicacion de Baja | - | Comunicacion de Baja |

---

## Grupo 1: Ventas Gravadas IGV (Facturas y Notas, Serie FF11)

| Caso | Descripcion | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|------|-------|-------|---------------|-------------|
| 1 | Factura 1 con 3 items | 1 | FF11 | 3 | 0 | 0 |
| 2 | Factura 2 con 2 items | 1 | FF11 | 2 | 0 | 0 |
| 3 | Factura 3 con 1 items | 1 | FF11 | 1 | 0 | 0 |
| 4 | Factura 4 con 5 items | 1 | FF11 | 5 | 0 | 0 |
| 5 | Factura 5 con 4 items | 1 | FF11 | 4 | 0 | 0 |
| 6 | Nota de credito de factura 2 | 7 | FF11 | 0 | 2 | 0 |
| 7 | Nota de credito de factura 3 | 7 | FF11 | 0 | 3 | 0 |
| 8 | Nota de credito de factura 4 | 7 | FF11 | 0 | 4 | 0 |
| 9 | Nota de debito de factura 2 | 8 | FF11 | 0 | 2 | 0 |
| 10 | Nota de debito de factura 3 | 8 | FF11 | 0 | 3 | 0 |
| 11 | Nota de debito de factura 4 | 8 | FF11 | 0 | 4 | 0 |

---

## Grupo 2: Ventas Inafectas y/o Exoneradas (Facturas y Notas, Serie FF12)

| Caso | Descripcion | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|------|-------|-------|---------------|-------------|
| 12 | Factura 1 con 1 item | 1 | FF12 | 1 | 0 | 0 |
| 13 | Factura 2 con 4 items | 1 | FF12 | 4 | 0 | 0 |
| 14 | Factura 3 con 7 items | 1 | FF12 | 7 | 0 | 0 |
| 15 | Factura 4 con 5 items | 1 | FF12 | 5 | 0 | 0 |
| 16 | Factura 5 con 6 items | 1 | FF12 | 6 | 0 | 0 |
| 17 | Nota de credito de factura 1 | 7 | FF12 | 0 | 12 | 0 |
| 18 | Nota de credito de factura 3 | 7 | FF12 | 0 | 14 | 0 |
| 19 | Nota de credito de factura 5 | 7 | FF12 | 0 | 16 | 0 |
| 20 | Nota de debito de factura 1 | 8 | FF12 | 0 | 12 | 0 |
| 21 | Nota de debito de factura 3 | 8 | FF12 | 0 | 14 | 0 |
| 22 | Nota de debito de factura 5 | 8 | FF12 | 0 | 16 | 0 |

---

## Grupo 3: Ventas Gratuitas (Facturas y Notas, Serie FF13)

| Caso | Descripcion | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|------|-------|-------|---------------|-------------|
| 23 | Factura 1 con 7 items | 1 | FF13 | 7 | 0 | 0 |
| 24 | Factura 2 con 2 items | 1 | FF13 | 2 | 0 | 0 |
| 25 | Factura 3 con 5 items | 1 | FF13 | 5 | 0 | 0 |
| 26 | Factura 4 con 4 items | 1 | FF13 | 4 | 0 | 0 |
| 27 | Factura 5 con 3 items | 1 | FF13 | 3 | 0 | 0 |
| 28 | Nota de credito de factura 2 | 7 | FF13 | 0 | 24 | 0 |
| 29 | Nota de credito de factura 3 | 7 | FF13 | 0 | 25 | 0 |
| 30 | Nota de debito de factura 2 | 8 | FF13 | 0 | 24 | 0 |
| 31 | Nota de debito de factura 3 | 8 | FF13 | 0 | 25 | 0 |

---

## Grupo 4: Ventas con Descuento Global (Facturas y Notas, Serie FF14)

| Caso | Descripcion | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|------|-------|-------|---------------|-------------|
| 32 | Factura 1 con 2 items | 1 | FF14 | 2 | 0 | 0 |
| 33 | Factura 2 con 1 item | 1 | FF14 | 1 | 0 | 0 |
| 34 | Factura 3 con 4 items | 1 | FF14 | 4 | 0 | 0 |
| 35 | Factura 4 con 3 items | 1 | FF14 | 3 | 0 | 0 |
| 36 | Factura 5 con 5 items | 1 | FF14 | 5 | 0 | 0 |
| 37 | Nota de credito de factura 2 | 7 | FF14 | 0 | 33 | 0 |
| 38 | Nota de credito de factura 3 | 7 | FF14 | 0 | 34 | 0 |
| 39 | Nota de credito de factura 5 | 7 | FF14 | 0 | 36 | 0 |
| 40 | Nota de debito de factura 2 | 8 | FF14 | 0 | 33 | 0 |
| 41 | Nota de debito de factura 3 | 8 | FF14 | 0 | 34 | 0 |
| 42 | Nota de debito de factura 5 | 8 | FF14 | 0 | 36 | 0 |

---

## Grupo 5: Operaciones Gravadas con ISC (Facturas y Notas, Serie FF30)

| Caso | Descripcion | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|------|-------|-------|---------------|-------------|
| 43 | Factura 1 con 5 items | 1 | FF30 | 5 | 0 | 0 |
| 44 | Nota de credito de factura 1 | 7 | FF30 | 0 | 43 | 0 |
| 45 | Nota de debito de factura 1 | 8 | FF30 | 0 | 43 | 0 |

---

## Grupo 6: Operaciones con Percepcion (Facturas y Notas, Serie FF40)

| Caso | Descripcion | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|------|-------|-------|---------------|-------------|
| 46 | Factura 1 con 5 items | 1 | FF40 | 5 | 0 | 0 |
| 47 | Nota de credito de factura 1 | 7 | FF40 | 0 | 46 | 0 |
| 48 | Nota de debito de factura 1 | 8 | FF40 | 0 | 46 | 0 |

---

## Grupo 7: Operaciones con Otro Tipo de Moneda (Facturas y Notas, Serie FF50)

| Caso | Descripcion | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|------|-------|-------|---------------|-------------|
| 49 | Factura 1 con 5 items | 1 | FF50 | 5 | 0 | 0 |
| 50 | Nota de credito de factura 1 | 7 | FF50 | 0 | 49 | 0 |
| 51 | Nota de debito de factura 1 | 8 | FF50 | 0 | 49 | 0 |

---

## Grupo 8: Ventas Gravadas IGV (Boletas y Notas, Serie BB11)

| Caso | Descripcion | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|------|-------|-------|---------------|-------------|
| 52 | Boleta de Venta 1 con 4 items | 3 | BB11 | 4 | 0 | 0 |
| 53 | Boleta de Venta 2 con 7 items | 3 | BB11 | 7 | 0 | 0 |
| 54 | Boleta de Venta 3 con 5 items | 3 | BB11 | 5 | 0 | 1 |
| 55 | Boleta de Venta 4 con 3 items | 3 | BB11 | 3 | 0 | 0 |
| 56 | Boleta de Venta 5 con 2 items | 3 | BB11 | 2 | 0 | 0 |
| 57 | Nota de credito de Boleta de Venta 2 | 7 | BB11 | 0 | 53 | 1 |
| 58 | Nota de credito de Boleta de Venta 3 | 7 | BB11 | 0 | 54 | 0 |
| 59 | Nota de credito de Boleta de Venta 4 | 7 | BB11 | 0 | 55 | 0 |
| 60 | Nota de debito de Boleta de Venta 2 | 8 | BB11 | 0 | 53 | 1 |
| 61 | Nota de debito de Boleta de Venta 3 | 8 | BB11 | 0 | 54 | 0 |
| 62 | Nota de debito de Boleta de Venta 4 | 8 | BB11 | 0 | 55 | 0 |

---

## Grupo 9: Ventas Inafectas y/o Exoneradas (Boletas y Notas, Serie BB12)

| Caso | Descripcion | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|------|-------|-------|---------------|-------------|
| 63 | Boleta de Venta 1 con 2 items | 3 | BB12 | 2 | 0 | 0 |
| 64 | Boleta de Venta 2 con 4 items | 3 | BB12 | 4 | 0 | 0 |
| 65 | Boleta de Venta 3 con 7 items | 3 | BB12 | 7 | 0 | 1 |
| 66 | Boleta de Venta 4 con 5 items | 3 | BB12 | 5 | 0 | 0 |
| 67 | Boleta de Venta 5 con 1 item | 3 | BB12 | 1 | 0 | 0 |
| 68 | Nota de credito de Boleta de Venta 1 | 7 | BB12 | 0 | 63 | 0 |
| 69 | Nota de credito de Boleta de Venta 4 | 7 | BB12 | 0 | 66 | 1 |
| 70 | Nota de credito de Boleta de Venta 5 | 7 | BB12 | 0 | 67 | 0 |
| 71 | Nota de debito de Boleta de Venta 1 | 8 | BB12 | 0 | 63 | 0 |
| 72 | Nota de debito de Boleta de Venta 4 | 8 | BB12 | 0 | 66 | 1 |
| 73 | Nota de debito de Boleta de Venta 5 | 8 | BB12 | 0 | 67 | 0 |

---

## Grupo 10: Ventas Gratuitas (Boletas y Notas, Serie BB13)

| Caso | Descripcion | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|------|-------|-------|---------------|-------------|
| 74 | Boleta de Venta 1 con 7 items | 3 | BB13 | 7 | 0 | 0 |
| 75 | Boleta de Venta 2 con 2 items | 3 | BB13 | 2 | 0 | 0 |
| 76 | Boleta de Venta 3 con 5 items | 3 | BB13 | 5 | 0 | 1 |
| 77 | Boleta de Venta 4 con 4 items | 3 | BB13 | 4 | 0 | 0 |
| 78 | Boleta de Venta 5 con 9 items | 3 | BB13 | 9 | 0 | 0 |
| 79 | Nota de credito de Boleta de Venta 1 | 7 | BB13 | 0 | 74 | 1 |
| 80 | Nota de credito de Boleta de Venta 2 | 7 | BB13 | 0 | 75 | 0 |
| 81 | Nota de credito de Boleta de Venta 4 | 7 | BB13 | 0 | 77 | 0 |
| 82 | Nota de debito de Boleta de Venta 1 | 8 | BB13 | 0 | 74 | 1 |
| 83 | Nota de debito de Boleta de Venta 2 | 8 | BB13 | 0 | 75 | 0 |
| 84 | Nota de debito de Boleta de Venta 4 | 8 | BB13 | 0 | 77 | 0 |

---

## Grupo 11: Ventas con Descuento Global (Boletas y Notas, Serie BB14)

| Caso | Descripcion | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|------|-------|-------|---------------|-------------|
| 85 | Boleta de Venta 1 con 10 items | 3 | BB14 | 10 | 0 | 0 |
| 86 | Boleta de Venta 2 con 7 items | 3 | BB14 | 7 | 0 | 1 |
| 87 | Boleta de Venta 3 con 6 items | 3 | BB14 | 6 | 0 | 0 |
| 88 | Boleta de Venta 4 con 9 items | 3 | BB14 | 9 | 0 | 0 |
| 89 | Boleta de Venta 5 con 4 items | 3 | BB14 | 4 | 0 | 0 |
| 90 | Nota de credito de Boleta de Venta 1 | 7 | BB14 | 0 | 85 | 0 |
| 91 | Nota de credito de Boleta de Venta 2 | 7 | BB14 | 0 | 86 | 1 |
| 92 | Nota de credito de Boleta de Venta 4 | 7 | BB14 | 0 | 88 | 0 |
| 93 | Nota de debito de Boleta de Venta 1 | 8 | BB14 | 0 | 85 | 0 |
| 94 | Nota de debito de Boleta de Venta 2 | 8 | BB14 | 0 | 86 | 1 |
| 95 | Nota de debito de Boleta de Venta 4 | 8 | BB14 | 0 | 88 | 0 |

---

## Grupo 12: Operaciones con Otro Tipo de Moneda (Boletas y Notas, Serie BB50)

| Caso | Descripcion | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|------|-------|-------|---------------|-------------|
| 96 | Boleta de Venta 1 con 3 items | 3 | BB50 | 3 | 0 | 0 |
| 97 | Nota de credito de Boleta de Venta 1 | 7 | BB50 | 0 | 96 | 0 |
| 98 | Nota de debito de Boleta de Venta 1 | 8 | BB50 | 0 | 96 | 0 |

---

## Grupo 13: Resumen Diario de Boletas de Venta

| Caso | Descripcion | Cod Grupo | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|-----------|------|-------|-------|---------------|-------------|
| 99 | Resumen diario de Boletas de venta | 13 | RC | - | 5 | 0 | 0 |

---

## Grupo 14: Comunicacion de Baja

| Caso | Descripcion | Cod Grupo | Tipo | Serie | Items | Caso Afectado | Rep. Impresa |
|------|------------|-----------|------|-------|-------|---------------|-------------|
| 100 | Comunicacion de Baja | 14 | RA | - | 5 | 0 | 0 |
