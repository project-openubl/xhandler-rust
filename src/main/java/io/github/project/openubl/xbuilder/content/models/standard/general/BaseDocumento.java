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

public class BaseDocumento {
    public String moneda;
    public String serie;
    public Integer numero;

    public LocalDate fechaEmision;
    public LocalTime horaEmision;

    public Cliente cliente;
    public Proveedor proveedor;
    public Firmante firmante;

    public List<DocumentoDetalle> detalle;

    public TotalImporte totalImporte;
    public TotalImpuestos totalImpuestos;

    public String formaDePago;
    public BigDecimal formaDePagoTotal;
    public List<CuotaDePago> formaDePagoCuotas;

    public List<GuiaRemisionRelacionada> guiasRemisionRelacionadas;
}
