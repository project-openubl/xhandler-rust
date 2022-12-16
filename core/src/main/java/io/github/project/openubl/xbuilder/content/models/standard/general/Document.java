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
import io.swagger.v3.oas.annotations.media.ArraySchema;
import io.swagger.v3.oas.annotations.media.Schema;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.Singular;
import lombok.experimental.SuperBuilder;

import java.math.BigDecimal;
import java.time.LocalDate;
import java.time.LocalTime;
import java.util.List;
import java.util.Map;

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
    @Schema(minLength = 3, maxLength = 3)
    private String moneda;

    /**
     * Tasa del IGV. Ejemplo: 0.18
     */
    @Schema(description = "Ejemplo: 0.18", minimum = "0", maximum = "1")
    private BigDecimal tasaIgv;

    /**
     * Tasa del IBC. Ejemplo: 0.2
     */
    @Schema(description = "Ejemplo: 0.2", minimum = "0")
    private BigDecimal tasaIcb;

    /**
     * Serie del comprobante
     */
    @Schema(requiredMode = Schema.RequiredMode.REQUIRED, minLength = 4, pattern = "^[F|f|B|b].*$")
    private String serie;

    /**
     * Número del comprobante
     */
    @Schema(requiredMode = Schema.RequiredMode.REQUIRED, minimum = "1", maximum = "99999999")
    private Integer numero;

    /**
     * Fecha de emisión del comprobante. Ejemplo 2022-12-25 (YYYY-MM-SS)
     */
    @Schema(description = "Format: \"YYYY-MM-SS\". Ejemplo: 2022-12-25", pattern = "^\\d{4}-\\d{2}-\\d{2}$")
    private LocalDate fechaEmision;

    /**
     * Hora de emisión del comprobante. Ejemplo 12:00:00 (HH:MM:SS)
     */
    @Schema(description = "Format: \"HH:MM:SS\". Ejemplo 12:00:00", pattern = "^\\d{2}:\\d{2}:\\d{2}$")
    private LocalTime horaEmision;

    /**
     * Orden de compra
     */
    private String ordenDeCompra;

    /**
     * Cliente
     */
    @Schema(requiredMode = Schema.RequiredMode.REQUIRED)
    private Cliente cliente;

    /**
     * Proveedor del bien o servicio
     */
    @Schema(requiredMode = Schema.RequiredMode.REQUIRED)
    private Proveedor proveedor;

    /**
     * Persona que firma electrónicamente el comprobante. Si es NULL los datos del proveedor son usados.
     */
    @Schema(
            description = "Persona que firma electrónicamente el comprobante. Si NULL los datos del proveedor son usados."
    )
    private Firmante firmante;

    /**
     * Total de impuestos a pagar
     */
    private TotalImpuestos totalImpuestos;

    /**
     * Detalle del comprobante
     */
    @Singular
    @ArraySchema(minItems = 1, schema = @Schema(requiredMode = Schema.RequiredMode.REQUIRED))
    private List<DocumentoDetalle> detalles;

    /**
     * Guias de remision relacionadas
     */
    @Singular
    @ArraySchema
    private List<Guia> guias;

    /**
     * Otros documentos relacionados
     */
    @Singular
    @ArraySchema
    private List<DocumentoRelacionado> documentosRelacionados;
}
