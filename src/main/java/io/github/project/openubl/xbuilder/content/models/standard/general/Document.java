/*
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 *
 * Licensed under the Apache License - 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package io.github.project.openubl.xbuilder.content.models.standard.general;

import io.github.project.openubl.xbuilder.content.models.common.Cliente;
import io.github.project.openubl.xbuilder.content.models.common.Firmante;
import io.github.project.openubl.xbuilder.content.models.common.Proveedor;
import java.math.BigDecimal;
import java.time.LocalDate;
import java.time.LocalTime;
import java.util.List;
import java.util.Map;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.Singular;
import lombok.experimental.SuperBuilder;

@Data
@SuperBuilder
@NoArgsConstructor
public abstract class Document {

    /**
     * Leyendas asociadas al comprobante
     */
    @Singular
    private Map<String, String> leyendas;

    /**
     * Moneda en la que se emite el comprobante
     */
    private String moneda;

    /**
     * Tasa del IGV. Ejemplo: 0.18
     */
    private BigDecimal tasaIgv;

    /**
     * Tasa del IBC. Ejemplo: 0.2
     */
    private BigDecimal tasaIcb;

    /**
     * Serie del comprobante
     */
    private String serie;

    /**
     * Número del comprobante
     */
    private Integer numero;

    /**
     * Fecha de emisión del comprobante
     */
    private LocalDate fechaEmision;

    /**
     * Hora de emisión del comprobante
     */
    private LocalTime horaEmision;

    /**
     * Orden de compra
     */
    private String ordenDeCompra;

    /**
     * Cliente
     */
    private Cliente cliente;

    /**
     * Proveedor del bien o servicio
     */
    private Proveedor proveedor;

    /**
     * Persona que firma electrónicamente el comprobante
     */
    private Firmante firmante;

    /**
     * Total de impuestos a pagar
     */
    private TotalImpuestos totalImpuestos;

    /**
     * Detalle del comprobante
     */
    @Singular
    private List<DocumentoDetalle> detalles;

    /**
     * Guias de remision relacionadas
     */
    @Singular
    private List<Guia> guias;

    /**
     * Otros documentos relacionados
     */
    @Singular
    private List<DocumentoRelacionado> documentosRelacionados;
}
