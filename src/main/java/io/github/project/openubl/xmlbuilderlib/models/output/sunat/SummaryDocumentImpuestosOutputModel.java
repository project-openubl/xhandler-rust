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
import javax.validation.constraints.NotNull;

public class SummaryDocumentImpuestosOutputModel {

    @NotNull
    @Valid
    private ImpuestoTotalResumenDiarioOutputModel igv;

    @Valid
    private ImpuestoTotalResumenDiarioOutputModel icb;

    public ImpuestoTotalResumenDiarioOutputModel getIgv() {
        return igv;
    }

    public void setIgv(ImpuestoTotalResumenDiarioOutputModel igv) {
        this.igv = igv;
    }

    public ImpuestoTotalResumenDiarioOutputModel getIcb() {
        return icb;
    }

    public void setIcb(ImpuestoTotalResumenDiarioOutputModel icb) {
        this.icb = icb;
    }

    public static final class Builder {
        private ImpuestoTotalResumenDiarioOutputModel igv;
        private ImpuestoTotalResumenDiarioOutputModel icb;

        private Builder() {
        }

        public static Builder aSummaryDocumentImpuestosOutputModel() {
            return new Builder();
        }

        public Builder withIgv(ImpuestoTotalResumenDiarioOutputModel igv) {
            this.igv = igv;
            return this;
        }

        public Builder withIcb(ImpuestoTotalResumenDiarioOutputModel icb) {
            this.icb = icb;
            return this;
        }

        public SummaryDocumentImpuestosOutputModel build() {
            SummaryDocumentImpuestosOutputModel summaryDocumentImpuestosOutputModel = new SummaryDocumentImpuestosOutputModel();
            summaryDocumentImpuestosOutputModel.setIgv(igv);
            summaryDocumentImpuestosOutputModel.setIcb(icb);
            return summaryDocumentImpuestosOutputModel;
        }
    }
}
