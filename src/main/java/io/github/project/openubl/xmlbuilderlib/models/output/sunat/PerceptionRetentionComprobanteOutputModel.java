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
package io.github.project.openubl.xmlbuilderlib.models.output.sunat;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog1;

import javax.validation.constraints.*;
import java.math.BigDecimal;

public class PerceptionRetentionComprobanteOutputModel {

    @NotBlank
    @Size(min = 3, max = 3)
    private String moneda;

    @NotNull
    private Catalog1 tipo;

    @NotBlank
    @Pattern(regexp = "^[F|B|0-9].*$")
    private String serieNumero;

    @NotBlank
    private String fechaEmision;

    @Min(0)
    @NotNull
    private BigDecimal importeTotal;

    public String getMoneda() {
        return moneda;
    }

    public void setMoneda(String moneda) {
        this.moneda = moneda;
    }

    public Catalog1 getTipo() {
        return tipo;
    }

    public void setTipo(Catalog1 tipo) {
        this.tipo = tipo;
    }

    public String getSerieNumero() {
        return serieNumero;
    }

    public void setSerieNumero(String serieNumero) {
        this.serieNumero = serieNumero;
    }

    public String getFechaEmision() {
        return fechaEmision;
    }

    public void setFechaEmision(String fechaEmision) {
        this.fechaEmision = fechaEmision;
    }

    public BigDecimal getImporteTotal() {
        return importeTotal;
    }

    public void setImporteTotal(BigDecimal importeTotal) {
        this.importeTotal = importeTotal;
    }

    public static final class Builder {
        private String moneda;
        private Catalog1 tipo;
        private String serieNumero;
        private String fechaEmision;
        private BigDecimal importeTotal;

        private Builder() {
        }

        public static Builder aPerceptionRetentionComprobanteOutputModel() {
            return new Builder();
        }

        public Builder withMoneda(String moneda) {
            this.moneda = moneda;
            return this;
        }

        public Builder withTipo(Catalog1 tipo) {
            this.tipo = tipo;
            return this;
        }

        public Builder withSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
            return this;
        }

        public Builder withFechaEmision(String fechaEmision) {
            this.fechaEmision = fechaEmision;
            return this;
        }

        public Builder withImporteTotal(BigDecimal importeTotal) {
            this.importeTotal = importeTotal;
            return this;
        }

        public PerceptionRetentionComprobanteOutputModel build() {
            PerceptionRetentionComprobanteOutputModel perceptionRetentionComprobanteOutputModel = new PerceptionRetentionComprobanteOutputModel();
            perceptionRetentionComprobanteOutputModel.setMoneda(moneda);
            perceptionRetentionComprobanteOutputModel.setTipo(tipo);
            perceptionRetentionComprobanteOutputModel.setSerieNumero(serieNumero);
            perceptionRetentionComprobanteOutputModel.setFechaEmision(fechaEmision);
            perceptionRetentionComprobanteOutputModel.setImporteTotal(importeTotal);
            return perceptionRetentionComprobanteOutputModel;
        }
    }
}
