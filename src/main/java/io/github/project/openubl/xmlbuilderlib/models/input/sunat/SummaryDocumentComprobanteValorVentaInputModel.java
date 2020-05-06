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

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import javax.validation.constraints.Min;
import javax.validation.constraints.NotNull;
import java.math.BigDecimal;

@Schema(name = "SummaryDocumentLineComprobanteValorVenta")
public class SummaryDocumentComprobanteValorVentaInputModel {

    @NotNull
    @Min(0)
    @Schema(example = "100", description = "Importe total de la venta. Incluye impuestos, cargos, descuentos")
    private BigDecimal importeTotal;

    @Min(0)
    @Schema(example = "0", description = "Total 'otros cargos' aplicados al comprobante")
    private BigDecimal otrosCargos;

    @Min(0)
    @Schema(example = "0")
    private BigDecimal gravado;

    @Min(0)
    @Schema(example = "0")
    private BigDecimal exonerado;

    @Min(0)
    @Schema(example = "0")
    private BigDecimal inafecto;

    @Min(0)
    @Schema(example = "0")
    private BigDecimal gratuito;

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

    public BigDecimal getGravado() {
        return gravado;
    }

    public void setGravado(BigDecimal gravado) {
        this.gravado = gravado;
    }

    public BigDecimal getExonerado() {
        return exonerado;
    }

    public void setExonerado(BigDecimal exonerado) {
        this.exonerado = exonerado;
    }

    public BigDecimal getInafecto() {
        return inafecto;
    }

    public void setInafecto(BigDecimal inafecto) {
        this.inafecto = inafecto;
    }

    public BigDecimal getGratuito() {
        return gratuito;
    }

    public void setGratuito(BigDecimal gratuito) {
        this.gratuito = gratuito;
    }

    public static final class Builder {
        private BigDecimal importeTotal;
        private BigDecimal otrosCargos;
        private BigDecimal gravado;
        private BigDecimal exonerado;
        private BigDecimal inafecto;
        private BigDecimal gratuito;

        private Builder() {
        }

        public static Builder aSummaryDocumentComprobanteValorVentaInputModel() {
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

        public Builder withGravado(BigDecimal gravado) {
            this.gravado = gravado;
            return this;
        }

        public Builder withExonerado(BigDecimal exonerado) {
            this.exonerado = exonerado;
            return this;
        }

        public Builder withInafecto(BigDecimal inafecto) {
            this.inafecto = inafecto;
            return this;
        }

        public Builder withGratuito(BigDecimal gratuito) {
            this.gratuito = gratuito;
            return this;
        }

        public SummaryDocumentComprobanteValorVentaInputModel build() {
            SummaryDocumentComprobanteValorVentaInputModel summaryDocumentComprobanteValorVentaInputModel = new SummaryDocumentComprobanteValorVentaInputModel();
            summaryDocumentComprobanteValorVentaInputModel.setImporteTotal(importeTotal);
            summaryDocumentComprobanteValorVentaInputModel.setOtrosCargos(otrosCargos);
            summaryDocumentComprobanteValorVentaInputModel.setGravado(gravado);
            summaryDocumentComprobanteValorVentaInputModel.setExonerado(exonerado);
            summaryDocumentComprobanteValorVentaInputModel.setInafecto(inafecto);
            summaryDocumentComprobanteValorVentaInputModel.setGratuito(gratuito);
            return summaryDocumentComprobanteValorVentaInputModel;
        }
    }
}
