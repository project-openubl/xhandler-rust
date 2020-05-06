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
import javax.validation.constraints.Pattern;

public class VoidedDocumentLineOutputModel {

    @NotBlank
    @Pattern(regexp = "^([F|B|P|T|R][A-Z]?[0-9]{1,3})$")
    private String serie;

    @NotBlank
    @Pattern(regexp = "^([0-9]{1,8})$")
    private String numero;

    @NotBlank
    private String fechaEmision;

    @NotNull
    private Catalog1 tipoComprobante;

    public String getSerie() {
        return serie;
    }

    public void setSerie(String serie) {
        this.serie = serie;
    }

    public String getNumero() {
        return numero;
    }

    public void setNumero(String numero) {
        this.numero = numero;
    }

    public String getFechaEmision() {
        return fechaEmision;
    }

    public void setFechaEmision(String fechaEmision) {
        this.fechaEmision = fechaEmision;
    }

    public Catalog1 getTipoComprobante() {
        return tipoComprobante;
    }

    public void setTipoComprobante(Catalog1 tipoComprobante) {
        this.tipoComprobante = tipoComprobante;
    }

    public static final class Builder {
        private String serie;
        private String numero;
        private String fechaEmision;
        private Catalog1 tipoComprobante;

        private Builder() {
        }

        public static Builder aVoidedDocumentLineOutputModel() {
            return new Builder();
        }

        public Builder withSerie(String serie) {
            this.serie = serie;
            return this;
        }

        public Builder withNumero(String numero) {
            this.numero = numero;
            return this;
        }

        public Builder withFechaEmision(String fechaEmision) {
            this.fechaEmision = fechaEmision;
            return this;
        }

        public Builder withTipoComprobante(Catalog1 tipoComprobante) {
            this.tipoComprobante = tipoComprobante;
            return this;
        }

        public VoidedDocumentLineOutputModel build() {
            VoidedDocumentLineOutputModel voidedDocumentLineOutputModel = new VoidedDocumentLineOutputModel();
            voidedDocumentLineOutputModel.setSerie(serie);
            voidedDocumentLineOutputModel.setNumero(numero);
            voidedDocumentLineOutputModel.setFechaEmision(fechaEmision);
            voidedDocumentLineOutputModel.setTipoComprobante(tipoComprobante);
            return voidedDocumentLineOutputModel;
        }
    }
}
