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

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog1;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.constraints.CatalogConstraint;

import javax.validation.constraints.*;
import java.math.BigDecimal;

public class PerceptionRetentionComprobanteInputModel {

    @NotNull
    @Size(min = 3, max = 3)
    private String moneda;

    @NotNull
    @CatalogConstraint(value = Catalog1.class)
    private String tipo;

    @NotNull
    @NotBlank
    @Pattern(regexp = "^[F|B|0-9].*$")
    private String serieNumero;

    @NotNull
    private Long fechaEmision;

    @NotNull
    @Positive
    @Digits(integer = 100, fraction = 2)
    private BigDecimal importeTotal;

    public String getMoneda() {
        return moneda;
    }

    public void setMoneda(String moneda) {
        this.moneda = moneda;
    }

    public String getTipo() {
        return tipo;
    }

    public void setTipo(String tipo) {
        this.tipo = tipo;
    }

    public String getSerieNumero() {
        return serieNumero;
    }

    public void setSerieNumero(String serieNumero) {
        this.serieNumero = serieNumero;
    }

    public Long getFechaEmision() {
        return fechaEmision;
    }

    public void setFechaEmision(Long fechaEmision) {
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
        private String tipo;
        private String serieNumero;
        private Long fechaEmision;
        private BigDecimal importeTotal;

        private Builder() {
        }

        public static Builder aPerceptionRetentionComprobanteInputModel() {
            return new Builder();
        }

        public Builder withMoneda(String moneda) {
            this.moneda = moneda;
            return this;
        }

        public Builder withTipo(String tipo) {
            this.tipo = tipo;
            return this;
        }

        public Builder withSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
            return this;
        }

        public Builder withFechaEmision(Long fechaEmision) {
            this.fechaEmision = fechaEmision;
            return this;
        }

        public Builder withImporteTotal(BigDecimal importeTotal) {
            this.importeTotal = importeTotal;
            return this;
        }

        public PerceptionRetentionComprobanteInputModel build() {
            PerceptionRetentionComprobanteInputModel perceptionRetentionComprobanteInputModel = new PerceptionRetentionComprobanteInputModel();
            perceptionRetentionComprobanteInputModel.setMoneda(moneda);
            perceptionRetentionComprobanteInputModel.setTipo(tipo);
            perceptionRetentionComprobanteInputModel.setSerieNumero(serieNumero);
            perceptionRetentionComprobanteInputModel.setFechaEmision(fechaEmision);
            perceptionRetentionComprobanteInputModel.setImporteTotal(importeTotal);
            return perceptionRetentionComprobanteInputModel;
        }
    }
}
