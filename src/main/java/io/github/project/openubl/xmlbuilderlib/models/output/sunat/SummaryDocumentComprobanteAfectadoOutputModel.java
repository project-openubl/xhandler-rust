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

import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;

public class SummaryDocumentComprobanteAfectadoOutputModel {

    @NotNull
    private Catalog1 tipo;

    @NotBlank
    private String serieNumero;

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

    public static final class Builder {
        private Catalog1 tipo;
        private String serieNumero;

        private Builder() {
        }

        public static Builder aSummaryDocumentComprobanteAfectadoOutputModel() {
            return new Builder();
        }

        public Builder withTipo(Catalog1 tipo) {
            this.tipo = tipo;
            return this;
        }

        public Builder withSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
            return this;
        }

        public SummaryDocumentComprobanteAfectadoOutputModel build() {
            SummaryDocumentComprobanteAfectadoOutputModel summaryDocumentComprobanteAfectadoOutputModel = new SummaryDocumentComprobanteAfectadoOutputModel();
            summaryDocumentComprobanteAfectadoOutputModel.setTipo(tipo);
            summaryDocumentComprobanteAfectadoOutputModel.setSerieNumero(serieNumero);
            return summaryDocumentComprobanteAfectadoOutputModel;
        }
    }
}
