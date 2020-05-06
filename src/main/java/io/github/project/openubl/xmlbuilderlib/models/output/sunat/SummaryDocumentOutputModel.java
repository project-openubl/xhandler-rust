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
import javax.validation.constraints.NotEmpty;
import javax.validation.constraints.NotNull;
import java.util.List;

public class SummaryDocumentOutputModel {

    @NotBlank
    private String serieNumero;

    @NotBlank
    private String moneda;

    @NotBlank
    private String fechaEmision;

    @NotBlank
    private String fechaEmisionDeComprobantesAsociados;

    @NotNull
    @Valid
    private FirmanteOutputModel firmante;

    @NotNull
    @Valid
    private ProveedorOutputModel proveedor;

    @NotNull
    @NotEmpty
    @Valid
    private List<SummaryDocumentLineOutputModel> detalle;

    public String getSerieNumero() {
        return serieNumero;
    }

    public void setSerieNumero(String serieNumero) {
        this.serieNumero = serieNumero;
    }

    public String getMoneda() {
        return moneda;
    }

    public void setMoneda(String moneda) {
        this.moneda = moneda;
    }

    public String getFechaEmision() {
        return fechaEmision;
    }

    public void setFechaEmision(String fechaEmision) {
        this.fechaEmision = fechaEmision;
    }

    public String getFechaEmisionDeComprobantesAsociados() {
        return fechaEmisionDeComprobantesAsociados;
    }

    public void setFechaEmisionDeComprobantesAsociados(String fechaEmisionDeComprobantesAsociados) {
        this.fechaEmisionDeComprobantesAsociados = fechaEmisionDeComprobantesAsociados;
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

    public List<SummaryDocumentLineOutputModel> getDetalle() {
        return detalle;
    }

    public void setDetalle(List<SummaryDocumentLineOutputModel> detalle) {
        this.detalle = detalle;
    }

    public static final class Builder {
        private String serieNumero;
        private String moneda;
        private String fechaEmision;
        private String fechaEmisionDeComprobantesAsociados;
        private FirmanteOutputModel firmante;
        private ProveedorOutputModel proveedor;
        private List<SummaryDocumentLineOutputModel> detalle;

        private Builder() {
        }

        public static Builder aSummaryDocumentOutputModel() {
            return new Builder();
        }

        public Builder withSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
            return this;
        }

        public Builder withMoneda(String moneda) {
            this.moneda = moneda;
            return this;
        }

        public Builder withFechaEmision(String fechaEmision) {
            this.fechaEmision = fechaEmision;
            return this;
        }

        public Builder withFechaEmisionDeComprobantesAsociados(String fechaEmisionDeComprobantesAsociados) {
            this.fechaEmisionDeComprobantesAsociados = fechaEmisionDeComprobantesAsociados;
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

        public Builder withDetalle(List<SummaryDocumentLineOutputModel> detalle) {
            this.detalle = detalle;
            return this;
        }

        public SummaryDocumentOutputModel build() {
            SummaryDocumentOutputModel summaryDocumentOutputModel = new SummaryDocumentOutputModel();
            summaryDocumentOutputModel.setSerieNumero(serieNumero);
            summaryDocumentOutputModel.setMoneda(moneda);
            summaryDocumentOutputModel.setFechaEmision(fechaEmision);
            summaryDocumentOutputModel.setFechaEmisionDeComprobantesAsociados(fechaEmisionDeComprobantesAsociados);
            summaryDocumentOutputModel.setFirmante(firmante);
            summaryDocumentOutputModel.setProveedor(proveedor);
            summaryDocumentOutputModel.setDetalle(detalle);
            return summaryDocumentOutputModel;
        }
    }
}
