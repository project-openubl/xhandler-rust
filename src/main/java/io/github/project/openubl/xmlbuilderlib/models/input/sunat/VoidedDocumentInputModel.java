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

import io.github.project.openubl.xmlbuilderlib.models.input.common.FirmanteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import org.eclipse.microprofile.openapi.annotations.media.Schema;

import javax.validation.Valid;
import javax.validation.constraints.Max;
import javax.validation.constraints.Min;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;

@Schema(name = "VoidedDocument")
public class VoidedDocumentInputModel {

    @NotNull
    @Min(1)
    @Max(99999999)
    @Schema(example = "1", description = "Número de Comprobante de Baja emitida durante el día")
    private Integer numero;

    @Schema(example = "1585398109198", description = "Fecha en la que se emite el comprobante de Baja. Fecha expresada en milliseconds")
    private Long fechaEmision;

    @Valid
    private FirmanteInputModel firmante;

    @NotNull
    @Valid
    private ProveedorInputModel proveedor;

    @NotNull
    @NotBlank
    @Schema(example = "mi razón", description = "Razón por la que se da de baja el comprobante")
    private String descripcionSustento;

    @NotNull
    @Valid
    private VoidedDocumentLineInputModel comprobante;

    public Integer getNumero() {
        return numero;
    }

    public void setNumero(Integer numero) {
        this.numero = numero;
    }

    public Long getFechaEmision() {
        return fechaEmision;
    }

    public void setFechaEmision(Long fechaEmision) {
        this.fechaEmision = fechaEmision;
    }

    public FirmanteInputModel getFirmante() {
        return firmante;
    }

    public void setFirmante(FirmanteInputModel firmante) {
        this.firmante = firmante;
    }

    public ProveedorInputModel getProveedor() {
        return proveedor;
    }

    public void setProveedor(ProveedorInputModel proveedor) {
        this.proveedor = proveedor;
    }

    public String getDescripcionSustento() {
        return descripcionSustento;
    }

    public void setDescripcionSustento(String descripcionSustento) {
        this.descripcionSustento = descripcionSustento;
    }

    public VoidedDocumentLineInputModel getComprobante() {
        return comprobante;
    }

    public void setComprobante(VoidedDocumentLineInputModel comprobante) {
        this.comprobante = comprobante;
    }

    public static final class Builder {
        private Integer numero;
        private Long fechaEmision;
        private FirmanteInputModel firmante;
        private ProveedorInputModel proveedor;
        private String descripcionSustento;
        private VoidedDocumentLineInputModel comprobante;

        private Builder() {
        }

        public static Builder aVoidedDocumentInputModel() {
            return new Builder();
        }

        public Builder withNumero(Integer numero) {
            this.numero = numero;
            return this;
        }

        public Builder withFechaEmision(Long fechaEmision) {
            this.fechaEmision = fechaEmision;
            return this;
        }

        public Builder withFirmante(FirmanteInputModel firmante) {
            this.firmante = firmante;
            return this;
        }

        public Builder withProveedor(ProveedorInputModel proveedor) {
            this.proveedor = proveedor;
            return this;
        }

        public Builder withDescripcionSustento(String descripcionSustento) {
            this.descripcionSustento = descripcionSustento;
            return this;
        }

        public Builder withComprobante(VoidedDocumentLineInputModel comprobante) {
            this.comprobante = comprobante;
            return this;
        }

        public VoidedDocumentInputModel build() {
            VoidedDocumentInputModel voidedDocumentInputModel = new VoidedDocumentInputModel();
            voidedDocumentInputModel.setNumero(numero);
            voidedDocumentInputModel.setFechaEmision(fechaEmision);
            voidedDocumentInputModel.setFirmante(firmante);
            voidedDocumentInputModel.setProveedor(proveedor);
            voidedDocumentInputModel.setDescripcionSustento(descripcionSustento);
            voidedDocumentInputModel.setComprobante(comprobante);
            return voidedDocumentInputModel;
        }
    }
}
