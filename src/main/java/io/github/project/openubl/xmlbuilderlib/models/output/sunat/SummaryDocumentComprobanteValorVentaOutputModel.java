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

import javax.validation.Valid;
import javax.validation.constraints.Min;
import javax.validation.constraints.NotNull;
import java.math.BigDecimal;

public class SummaryDocumentComprobanteValorVentaOutputModel {

    @NotNull
    @Min(0)
    private BigDecimal importeTotal;

    @Min(0)
    private BigDecimal otrosCargos;

    @Valid
    private TotalValorVentaOutputModel gravado;

    @Valid
    private TotalValorVentaOutputModel exonerado;

    @Valid
    private TotalValorVentaOutputModel inafecto;

    @Valid
    private TotalValorVentaOutputModel gratuito;

    public BigDecimal getImporteTotal() {
        return importeTotal;
    }

    public void setImporteTotal(BigDecimal importeTotal) {
        this.importeTotal = importeTotal;
    }

    public BigDecimal getOtrosCargos() {
        return otrosCargos;
    }

    public void setOtrosCargos(BigDecimal otrosCargos) {
        this.otrosCargos = otrosCargos;
    }

    public TotalValorVentaOutputModel getGravado() {
        return gravado;
    }

    public void setGravado(TotalValorVentaOutputModel gravado) {
        this.gravado = gravado;
    }

    public TotalValorVentaOutputModel getExonerado() {
        return exonerado;
    }

    public void setExonerado(TotalValorVentaOutputModel exonerado) {
        this.exonerado = exonerado;
    }

    public TotalValorVentaOutputModel getInafecto() {
        return inafecto;
    }

    public void setInafecto(TotalValorVentaOutputModel inafecto) {
        this.inafecto = inafecto;
    }

    public TotalValorVentaOutputModel getGratuito() {
        return gratuito;
    }

    public void setGratuito(TotalValorVentaOutputModel gratuito) {
        this.gratuito = gratuito;
    }

    public static final class Builder {
        private BigDecimal importeTotal;
        private BigDecimal otrosCargos;
        private TotalValorVentaOutputModel gravado;
        private TotalValorVentaOutputModel exonerado;
        private TotalValorVentaOutputModel inafecto;
        private TotalValorVentaOutputModel gratuito;

        private Builder() {
        }

        public static Builder aSummaryDocumentComprobanteValorVentaOutputModel() {
            return new Builder();
        }

        public Builder withImporteTotal(BigDecimal importeTotal) {
            this.importeTotal = importeTotal;
            return this;
        }

        public Builder withOtrosCargos(BigDecimal otrosCargos) {
            this.otrosCargos = otrosCargos;
            return this;
        }

        public Builder withGravado(TotalValorVentaOutputModel gravado) {
            this.gravado = gravado;
            return this;
        }

        public Builder withExonerado(TotalValorVentaOutputModel exonerado) {
            this.exonerado = exonerado;
            return this;
        }

        public Builder withInafecto(TotalValorVentaOutputModel inafecto) {
            this.inafecto = inafecto;
            return this;
        }

        public Builder withGratuito(TotalValorVentaOutputModel gratuito) {
            this.gratuito = gratuito;
            return this;
        }

        public SummaryDocumentComprobanteValorVentaOutputModel build() {
            SummaryDocumentComprobanteValorVentaOutputModel summaryDocumentComprobanteValorVentaOutputModel = new SummaryDocumentComprobanteValorVentaOutputModel();
            summaryDocumentComprobanteValorVentaOutputModel.setImporteTotal(importeTotal);
            summaryDocumentComprobanteValorVentaOutputModel.setOtrosCargos(otrosCargos);
            summaryDocumentComprobanteValorVentaOutputModel.setGravado(gravado);
            summaryDocumentComprobanteValorVentaOutputModel.setExonerado(exonerado);
            summaryDocumentComprobanteValorVentaOutputModel.setInafecto(inafecto);
            summaryDocumentComprobanteValorVentaOutputModel.setGratuito(gratuito);
            return summaryDocumentComprobanteValorVentaOutputModel;
        }
    }
}
