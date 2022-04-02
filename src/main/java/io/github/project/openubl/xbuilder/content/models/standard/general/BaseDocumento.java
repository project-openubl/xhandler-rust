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
import lombok.Singular;
import lombok.experimental.SuperBuilder;

@Data
@SuperBuilder
public abstract class BaseDocumento {

    @Singular
    private Map<String, String> leyendas;

    private String moneda;
    private BigDecimal tasaIgv;
    private BigDecimal tasaIcb;

    private String serie;
    private Integer numero;

    private LocalDate fechaEmision;
    private LocalTime horaEmision;

    private Cliente cliente;
    private Proveedor proveedor;
    private Firmante firmante;

    private TotalImpuestos totalImpuestos;

    @Singular
    private List<DocumentoDetalle> detalles;

    private List<GuiaRemisionRelacionada> guiasRemisionRelacionadas;
}
