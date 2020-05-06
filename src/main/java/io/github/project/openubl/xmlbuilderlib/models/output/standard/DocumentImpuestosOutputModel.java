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
package io.github.project.openubl.xmlbuilderlib.models.output.standard;

import javax.validation.Valid;
import javax.validation.constraints.Digits;
import javax.validation.constraints.Min;
import javax.validation.constraints.NotNull;
import java.math.BigDecimal;

public class DocumentImpuestosOutputModel {

    @Min(0)
    @NotNull
    @Digits(integer = 100, fraction = 2)
    private BigDecimal importeTotal;

    @Valid
    private ImpuestoTotalOutputModel ivap;

    @Valid
    private ImpuestoTotalOutputModel gravadas;

    @Valid
    private ImpuestoTotalOutputModel inafectas;

    @Valid
    private ImpuestoTotalOutputModel exoneradas;

    @Valid
    private ImpuestoTotalOutputModel gratuitas;

    @Valid
    private ImpuestoTotalICBOutputModel icb;

    public BigDecimal getImporteTotal() {
        return importeTotal;
    }

    public void setImporteTotal(BigDecimal importeTotal) {
        this.importeTotal = importeTotal;
    }

    public ImpuestoTotalOutputModel getIvap() {
        return ivap;
    }

    public void setIvap(ImpuestoTotalOutputModel ivap) {
        this.ivap = ivap;
    }

    public ImpuestoTotalOutputModel getGravadas() {
        return gravadas;
    }

    public void setGravadas(ImpuestoTotalOutputModel gravadas) {
        this.gravadas = gravadas;
    }

    public ImpuestoTotalOutputModel getInafectas() {
        return inafectas;
    }

    public void setInafectas(ImpuestoTotalOutputModel inafectas) {
        this.inafectas = inafectas;
    }

    public ImpuestoTotalOutputModel getExoneradas() {
        return exoneradas;
    }

    public void setExoneradas(ImpuestoTotalOutputModel exoneradas) {
        this.exoneradas = exoneradas;
    }

    public ImpuestoTotalOutputModel getGratuitas() {
        return gratuitas;
    }

    public void setGratuitas(ImpuestoTotalOutputModel gratuitas) {
        this.gratuitas = gratuitas;
    }

    public ImpuestoTotalICBOutputModel getIcb() {
        return icb;
    }

    public void setIcb(ImpuestoTotalICBOutputModel icb) {
        this.icb = icb;
    }

    public static final class Builder {
        private BigDecimal importeTotal;
        private ImpuestoTotalOutputModel ivap;
        private ImpuestoTotalOutputModel gravadas;
        private ImpuestoTotalOutputModel inafectas;
        private ImpuestoTotalOutputModel exoneradas;
        private ImpuestoTotalOutputModel gratuitas;
        private ImpuestoTotalICBOutputModel icb;

        private Builder() {
        }

        public static Builder aDocumentImpuestosOutputModel() {
            return new Builder();
        }

        public Builder withImporteTotal(BigDecimal importeTotal) {
            this.importeTotal = importeTotal;
            return this;
        }

        public Builder withIvap(ImpuestoTotalOutputModel ivap) {
            this.ivap = ivap;
            return this;
        }

        public Builder withGravadas(ImpuestoTotalOutputModel gravadas) {
            this.gravadas = gravadas;
            return this;
        }

        public Builder withInafectas(ImpuestoTotalOutputModel inafectas) {
            this.inafectas = inafectas;
            return this;
        }

        public Builder withExoneradas(ImpuestoTotalOutputModel exoneradas) {
            this.exoneradas = exoneradas;
            return this;
        }

        public Builder withGratuitas(ImpuestoTotalOutputModel gratuitas) {
            this.gratuitas = gratuitas;
            return this;
        }

        public Builder withIcb(ImpuestoTotalICBOutputModel icb) {
            this.icb = icb;
            return this;
        }

        public DocumentImpuestosOutputModel build() {
            DocumentImpuestosOutputModel documentImpuestosOutputModel = new DocumentImpuestosOutputModel();
            documentImpuestosOutputModel.setImporteTotal(importeTotal);
            documentImpuestosOutputModel.setIvap(ivap);
            documentImpuestosOutputModel.setGravadas(gravadas);
            documentImpuestosOutputModel.setInafectas(inafectas);
            documentImpuestosOutputModel.setExoneradas(exoneradas);
            documentImpuestosOutputModel.setGratuitas(gratuitas);
            documentImpuestosOutputModel.setIcb(icb);
            return documentImpuestosOutputModel;
        }
    }
}
