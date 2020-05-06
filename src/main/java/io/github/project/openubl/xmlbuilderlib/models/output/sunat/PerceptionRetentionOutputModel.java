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

import io.github.project.openubl.xmlbuilderlib.models.output.common.ClienteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.FirmanteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.ProveedorOutputModel;

import javax.validation.Valid;
import javax.validation.constraints.*;
import java.math.BigDecimal;
import java.util.List;

public abstract class PerceptionRetentionOutputModel {

    @NotNull
    @NotBlank
    protected String serieNumero;

    @NotNull
    @NotBlank
    protected String fechaEmision;

    protected String observacion;

    @NotNull
    @Size(min = 3, max = 3)
    protected String moneda;

    @NotNull
    @Min(0)
    protected BigDecimal importeTotalPercibidoRetenido;

    @NotNull
    @Min(0)
    protected BigDecimal importeTotalCobradoPagado;

    @NotNull
    @Valid
    protected ProveedorOutputModel proveedor;

    @NotNull
    @Valid
    protected ClienteOutputModel cliente;

    @NotNull
    @Valid
    protected FirmanteOutputModel firmante;

    @NotNull
    @NotEmpty
    @Valid
    protected List<PerceptionRetentionLineOutputModel> detalle;

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

    public String getMoneda() {
        return moneda;
    }

    public void setMoneda(String moneda) {
        this.moneda = moneda;
    }

    public String getObservacion() {
        return observacion;
    }

    public void setObservacion(String observacion) {
        this.observacion = observacion;
    }

    public BigDecimal getImporteTotalPercibidoRetenido() {
        return importeTotalPercibidoRetenido;
    }

    public void setImporteTotalPercibidoRetenido(BigDecimal importeTotalPercibidoRetenido) {
        this.importeTotalPercibidoRetenido = importeTotalPercibidoRetenido;
    }

    public ProveedorOutputModel getProveedor() {
        return proveedor;
    }

    public void setProveedor(ProveedorOutputModel proveedor) {
        this.proveedor = proveedor;
    }

    public ClienteOutputModel getCliente() {
        return cliente;
    }

    public void setCliente(ClienteOutputModel cliente) {
        this.cliente = cliente;
    }

    public FirmanteOutputModel getFirmante() {
        return firmante;
    }

    public void setFirmante(FirmanteOutputModel firmante) {
        this.firmante = firmante;
    }

    public List<PerceptionRetentionLineOutputModel> getDetalle() {
        return detalle;
    }

    public void setDetalle(List<PerceptionRetentionLineOutputModel> detalle) {
        this.detalle = detalle;
    }

    public BigDecimal getImporteTotalCobradoPagado() {
        return importeTotalCobradoPagado;
    }

    public void setImporteTotalCobradoPagado(BigDecimal importeTotalCobradoPagado) {
        this.importeTotalCobradoPagado = importeTotalCobradoPagado;
    }

    public static abstract class Builder {
        protected String serieNumero;
        protected String fechaEmision;
        protected String observacion;
        protected String moneda;
        protected BigDecimal importeTotalPercibidoRetenido;
        protected BigDecimal importeTotalCobradoPagado;
        protected ProveedorOutputModel proveedor;
        protected ClienteOutputModel cliente;
        protected FirmanteOutputModel firmante;
        protected List<PerceptionRetentionLineOutputModel> detalle;

        public Builder withSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
            return this;
        }

        public Builder withFechaEmision(String fechaEmision) {
            this.fechaEmision = fechaEmision;
            return this;
        }

        public Builder withObservacion(String observacion) {
            this.observacion = observacion;
            return this;
        }

        public Builder withMoneda(String moneda) {
            this.moneda = moneda;
            return this;
        }

        public Builder withImporteTotalPercibidoRetenido(BigDecimal importeTotalPercibidoRetenido) {
            this.importeTotalPercibidoRetenido = importeTotalPercibidoRetenido;
            return this;
        }

        public Builder withImporteTotalCobradoPagado(BigDecimal importeTotalCobradoPagado) {
            this.importeTotalCobradoPagado = importeTotalCobradoPagado;
            return this;
        }

        public Builder withProveedor(ProveedorOutputModel proveedor) {
            this.proveedor = proveedor;
            return this;
        }

        public Builder withCliente(ClienteOutputModel cliente) {
            this.cliente = cliente;
            return this;
        }

        public Builder withFirmante(FirmanteOutputModel firmante) {
            this.firmante = firmante;
            return this;
        }

        public Builder withDetalle(List<PerceptionRetentionLineOutputModel> detalle) {
            this.detalle = detalle;
            return this;
        }
    }
}
