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

import io.github.project.openubl.xmlbuilderlib.models.output.common.FirmanteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.ProveedorOutputModel;

import javax.validation.Valid;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Pattern;

public class VoidedDocumentOutputModel {

    @NotBlank
    @Pattern(regexp = "^([R][A|R])[-]([0-9]{8})[-]([0-9]{1,8})$")
    private String serieNumero;

    @NotBlank
    private String fechaEmision;

    @NotNull
    @Valid
    private FirmanteOutputModel firmante;

    @NotNull
    @Valid
    private ProveedorOutputModel proveedor;

    @NotBlank
    private String descripcionSustento;

    @NotNull
    @Valid
    private VoidedDocumentLineOutputModel comprobante;

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

    public FirmanteOutputModel getFirmante() {
        return firmante;
    }

    public void setFirmante(FirmanteOutputModel firmante) {
        this.firmante = firmante;
    }

    public ProveedorOutputModel getProveedor() {
        return proveedor;
    }

    public void setProveedor(ProveedorOutputModel proveedor) {
        this.proveedor = proveedor;
    }

    public String getDescripcionSustento() {
        return descripcionSustento;
    }

    public void setDescripcionSustento(String descripcionSustento) {
        this.descripcionSustento = descripcionSustento;
    }

    public VoidedDocumentLineOutputModel getComprobante() {
        return comprobante;
    }

    public void setComprobante(VoidedDocumentLineOutputModel comprobante) {
        this.comprobante = comprobante;
    }

    public static final class Builder {
        private String serieNumero;
        private String fechaEmision;
        private FirmanteOutputModel firmante;
        private ProveedorOutputModel proveedor;
        private String descripcionSustento;
        private VoidedDocumentLineOutputModel comprobante;

        private Builder() {
        }

        public static Builder aVoidedDocumentOutputModel() {
            return new Builder();
        }

        public Builder withSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
            return this;
        }

        public Builder withFechaEmision(String fechaEmision) {
            this.fechaEmision = fechaEmision;
            return this;
        }

        public Builder withFirmante(FirmanteOutputModel firmante) {
            this.firmante = firmante;
            return this;
        }

        public Builder withProveedor(ProveedorOutputModel proveedor) {
            this.proveedor = proveedor;
            return this;
        }

        public Builder withDescripcionSustento(String descripcionSustento) {
            this.descripcionSustento = descripcionSustento;
            return this;
        }

        public Builder withComprobante(VoidedDocumentLineOutputModel comprobante) {
            this.comprobante = comprobante;
            return this;
        }

        public VoidedDocumentOutputModel build() {
            VoidedDocumentOutputModel voidedDocumentOutputModel = new VoidedDocumentOutputModel();
            voidedDocumentOutputModel.setSerieNumero(serieNumero);
            voidedDocumentOutputModel.setFechaEmision(fechaEmision);
            voidedDocumentOutputModel.setFirmante(firmante);
            voidedDocumentOutputModel.setProveedor(proveedor);
            voidedDocumentOutputModel.setDescripcionSustento(descripcionSustento);
            voidedDocumentOutputModel.setComprobante(comprobante);
            return voidedDocumentOutputModel;
        }
    }
}
