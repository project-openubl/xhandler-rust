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
import javax.validation.constraints.Min;
import javax.validation.constraints.NotEmpty;
import javax.validation.constraints.NotNull;
import java.util.List;

@Schema(name = "SummaryDocument")
public class SummaryDocumentInputModel {

    @Min(1)
    @NotNull
    @Schema(example = "1", description = "Número de Resumen Diario emitido en el día")
    private Integer numero;

    @Schema(example = "1585398109198", description = "Fecha en la que se emite el comprobante de Resumen Diario. Fecha expresada en milliseconds")
    private Long fechaEmision;

    @NotNull
    @Schema(example = "1585398109198", description = "Fecha en la que todos los comprobantes, dentro del resumen, fueron emitidos. Fecha expresada en milliseconds")
    private Long fechaEmisionDeComprobantesAsociados;

    @Valid
    private FirmanteInputModel firmante;

    @NotNull
    @Valid
    private ProveedorInputModel proveedor;

    @NotNull
    @NotEmpty
    @Valid
    private List<SummaryDocumentLineInputModel> detalle;

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

    public Long getFechaEmisionDeComprobantesAsociados() {
        return fechaEmisionDeComprobantesAsociados;
    }

    public void setFechaEmisionDeComprobantesAsociados(Long fechaEmisionDeComprobantesAsociados) {
        this.fechaEmisionDeComprobantesAsociados = fechaEmisionDeComprobantesAsociados;
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

    public List<SummaryDocumentLineInputModel> getDetalle() {
        return detalle;
    }

    public void setDetalle(List<SummaryDocumentLineInputModel> detalle) {
        this.detalle = detalle;
    }

    public static final class Builder {
        private Integer numero;
        private Long fechaEmision;
        private Long fechaEmisionDeComprobantesAsociados;
        private FirmanteInputModel firmante;
        private ProveedorInputModel proveedor;
        private List<SummaryDocumentLineInputModel> detalle;

        private Builder() {
        }

        public static Builder aSummaryDocumentInputModel() {
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

        public Builder withFechaEmisionDeComprobantesAsociados(Long fechaEmisionDeComprobantesAsociados) {
            this.fechaEmisionDeComprobantesAsociados = fechaEmisionDeComprobantesAsociados;
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

        public Builder withDetalle(List<SummaryDocumentLineInputModel> detalle) {
            this.detalle = detalle;
            return this;
        }

        public SummaryDocumentInputModel build() {
            SummaryDocumentInputModel summaryDocumentInputModel = new SummaryDocumentInputModel();
            summaryDocumentInputModel.setNumero(numero);
            summaryDocumentInputModel.setFechaEmision(fechaEmision);
            summaryDocumentInputModel.setFechaEmisionDeComprobantesAsociados(fechaEmisionDeComprobantesAsociados);
            summaryDocumentInputModel.setFirmante(firmante);
            summaryDocumentInputModel.setProveedor(proveedor);
            summaryDocumentInputModel.setDetalle(detalle);
            return summaryDocumentInputModel;
        }
    }
}
