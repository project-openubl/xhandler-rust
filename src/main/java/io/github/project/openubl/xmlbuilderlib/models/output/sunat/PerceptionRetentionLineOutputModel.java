/**
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 * <p>
 * Licensed under the Eclipse Public License - v 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * <p>
 * https://www.eclipse.org/legal/epl-2.0/
 * <p>
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package io.github.project.openubl.xmlbuilderlib.models.output.sunat;

import javax.validation.Valid;
import javax.validation.constraints.Min;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import java.math.BigDecimal;

public class PerceptionRetentionLineOutputModel {

    @NotNull
    @Valid
    private PerceptionRetentionComprobanteOutputModel comprobante;


    //

    @Min(1)
    @NotNull
    private Integer numeroCobroPago;

    @NotNull
    @NotBlank
    private String fechaCobroPago;

    @Min(0)
    @NotNull
    private BigDecimal importeCobroPago;


    //


    @Min(0)
    @NotNull
    private BigDecimal importePercibidoRetenido;

    @Min(0)
    @NotNull
    private BigDecimal importeTotalCobradoPagado;


    public PerceptionRetentionComprobanteOutputModel getComprobante() {
        return comprobante;
    }

    public void setComprobante(PerceptionRetentionComprobanteOutputModel comprobante) {
        this.comprobante = comprobante;
    }

    public Integer getNumeroCobroPago() {
        return numeroCobroPago;
    }

    public void setNumeroCobroPago(Integer numeroCobroPago) {
        this.numeroCobroPago = numeroCobroPago;
    }

    public String getFechaCobroPago() {
        return fechaCobroPago;
    }

    public void setFechaCobroPago(String fechaCobroPago) {
        this.fechaCobroPago = fechaCobroPago;
    }

    public BigDecimal getImporteCobroPago() {
        return importeCobroPago;
    }

    public void setImporteCobroPago(BigDecimal importeCobroPago) {
        this.importeCobroPago = importeCobroPago;
    }

    public BigDecimal getImportePercibidoRetenido() {
        return importePercibidoRetenido;
    }

    public void setImportePercibidoRetenido(BigDecimal importePercibidoRetenido) {
        this.importePercibidoRetenido = importePercibidoRetenido;
    }

    public BigDecimal getImporteTotalCobradoPagado() {
        return importeTotalCobradoPagado;
    }

    public void setImporteTotalCobradoPagado(BigDecimal importeTotalCobradoPagado) {
        this.importeTotalCobradoPagado = importeTotalCobradoPagado;
    }

    public static final class Builder {
        private PerceptionRetentionComprobanteOutputModel comprobante;
        private Integer numeroCobroPago;
        private String fechaCobroPago;
        private BigDecimal importeCobroPago;
        private BigDecimal importePercibidoRetenido;
        private BigDecimal importeTotalCobradoPagado;

        private Builder() {
        }

        public static Builder aPerceptionRetentionLineOutputModel() {
            return new Builder();
        }

        public Builder withComprobante(PerceptionRetentionComprobanteOutputModel comprobante) {
            this.comprobante = comprobante;
            return this;
        }

        public Builder withNumeroCobroPago(Integer numeroCobroPago) {
            this.numeroCobroPago = numeroCobroPago;
            return this;
        }

        public Builder withFechaCobroPago(String fechaCobroPago) {
            this.fechaCobroPago = fechaCobroPago;
            return this;
        }

        public Builder withImporteCobroPago(BigDecimal importeCobroPago) {
            this.importeCobroPago = importeCobroPago;
            return this;
        }

        public Builder withImportePercibidoRetenido(BigDecimal importePercibidoRetenido) {
            this.importePercibidoRetenido = importePercibidoRetenido;
            return this;
        }

        public Builder withImporteTotalCobradoPagado(BigDecimal importeTotalCobradoPagado) {
            this.importeTotalCobradoPagado = importeTotalCobradoPagado;
            return this;
        }

        public PerceptionRetentionLineOutputModel build() {
            PerceptionRetentionLineOutputModel perceptionRetentionLineOutputModel = new PerceptionRetentionLineOutputModel();
            perceptionRetentionLineOutputModel.setComprobante(comprobante);
            perceptionRetentionLineOutputModel.setNumeroCobroPago(numeroCobroPago);
            perceptionRetentionLineOutputModel.setFechaCobroPago(fechaCobroPago);
            perceptionRetentionLineOutputModel.setImporteCobroPago(importeCobroPago);
            perceptionRetentionLineOutputModel.setImportePercibidoRetenido(importePercibidoRetenido);
            perceptionRetentionLineOutputModel.setImporteTotalCobradoPagado(importeTotalCobradoPagado);
            return perceptionRetentionLineOutputModel;
        }
    }
}
