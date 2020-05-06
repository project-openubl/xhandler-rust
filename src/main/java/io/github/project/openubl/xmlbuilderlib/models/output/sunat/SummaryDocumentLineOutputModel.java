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

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog19;

import javax.validation.Valid;
import javax.validation.constraints.NotNull;

public class SummaryDocumentLineOutputModel {

    @NotNull
    private Catalog19 tipoOperacion;

    @NotNull
    @Valid
    private SummaryDocumentComprobanteOutputModel comprobante;

    @Valid
    private SummaryDocumentComprobanteAfectadoOutputModel comprobanteAfectado;

    public Catalog19 getTipoOperacion() {
        return tipoOperacion;
    }

    public void setTipoOperacion(Catalog19 tipoOperacion) {
        this.tipoOperacion = tipoOperacion;
    }

    public SummaryDocumentComprobanteOutputModel getComprobante() {
        return comprobante;
    }

    public void setComprobante(SummaryDocumentComprobanteOutputModel comprobante) {
        this.comprobante = comprobante;
    }

    public SummaryDocumentComprobanteAfectadoOutputModel getComprobanteAfectado() {
        return comprobanteAfectado;
    }

    public void setComprobanteAfectado(SummaryDocumentComprobanteAfectadoOutputModel comprobanteAfectado) {
        this.comprobanteAfectado = comprobanteAfectado;
    }

    public static final class Builder {
        private Catalog19 tipoOperacion;
        private SummaryDocumentComprobanteOutputModel comprobante;
        private SummaryDocumentComprobanteAfectadoOutputModel comprobanteAfectado;

        private Builder() {
        }

        public static Builder aSummaryDocumentLineOutputModel() {
            return new Builder();
        }

        public Builder withTipoOperacion(Catalog19 tipoOperacion) {
            this.tipoOperacion = tipoOperacion;
            return this;
        }

        public Builder withComprobante(SummaryDocumentComprobanteOutputModel comprobante) {
            this.comprobante = comprobante;
            return this;
        }

        public Builder withComprobanteAfectado(SummaryDocumentComprobanteAfectadoOutputModel comprobanteAfectado) {
            this.comprobanteAfectado = comprobanteAfectado;
            return this;
        }

        public SummaryDocumentLineOutputModel build() {
            SummaryDocumentLineOutputModel summaryDocumentLineOutputModel = new SummaryDocumentLineOutputModel();
            summaryDocumentLineOutputModel.setTipoOperacion(tipoOperacion);
            summaryDocumentLineOutputModel.setComprobante(comprobante);
            summaryDocumentLineOutputModel.setComprobanteAfectado(comprobanteAfectado);
            return summaryDocumentLineOutputModel;
        }
    }
}
