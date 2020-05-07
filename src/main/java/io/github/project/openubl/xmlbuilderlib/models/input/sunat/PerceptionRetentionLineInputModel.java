/**
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 *
 * Licensed under the Eclipse Public License - v 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.eclipse.org/legal/epl-2.0/
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package io.github.project.openubl.xmlbuilderlib.models.input.sunat;

import javax.validation.Valid;
import javax.validation.constraints.Min;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Positive;
import java.math.BigDecimal;

public class PerceptionRetentionLineInputModel {

    @Min(1)
    private Integer numeroCobroPago;

    private Long fechaCobroPago;

    @Positive
    private BigDecimal importeCobroPago;

    @NotNull
    @Valid
    private PerceptionRetentionComprobanteInputModel comprobante;

    public Integer getNumeroCobroPago() {
        return numeroCobroPago;
    }

    public void setNumeroCobroPago(Integer numeroCobroPago) {
        this.numeroCobroPago = numeroCobroPago;
    }

    public Long getFechaCobroPago() {
        return fechaCobroPago;
    }

    public void setFechaCobroPago(Long fechaCobroPago) {
        this.fechaCobroPago = fechaCobroPago;
    }

    public BigDecimal getImporteCobroPago() {
        return importeCobroPago;
    }

    public void setImporteCobroPago(BigDecimal importeCobroPago) {
        this.importeCobroPago = importeCobroPago;
    }

    public PerceptionRetentionComprobanteInputModel getComprobante() {
        return comprobante;
    }

    public void setComprobante(PerceptionRetentionComprobanteInputModel comprobante) {
        this.comprobante = comprobante;
    }

    public static final class Builder {
        private Integer numeroCobroPago;
        private Long fechaCobroPago;
        private BigDecimal importeCobroPago;
        private PerceptionRetentionComprobanteInputModel comprobante;

        private Builder() {
        }

        public static Builder aPerceptionRetentionLineInputModel() {
            return new Builder();
        }

        public Builder withNumeroCobroPago(Integer numeroCobroPago) {
            this.numeroCobroPago = numeroCobroPago;
            return this;
        }

        public Builder withFechaCobroPago(Long fechaCobroPago) {
            this.fechaCobroPago = fechaCobroPago;
            return this;
        }

        public Builder withImporteCobroPago(BigDecimal importeCobroPago) {
            this.importeCobroPago = importeCobroPago;
            return this;
        }

        public Builder withComprobante(PerceptionRetentionComprobanteInputModel comprobante) {
            this.comprobante = comprobante;
            return this;
        }

        public PerceptionRetentionLineInputModel build() {
            PerceptionRetentionLineInputModel perceptionRetentionLineInputModel = new PerceptionRetentionLineInputModel();
            perceptionRetentionLineInputModel.setNumeroCobroPago(numeroCobroPago);
            perceptionRetentionLineInputModel.setFechaCobroPago(fechaCobroPago);
            perceptionRetentionLineInputModel.setImporteCobroPago(importeCobroPago);
            perceptionRetentionLineInputModel.setComprobante(comprobante);
            return perceptionRetentionLineInputModel;
        }
    }
}
