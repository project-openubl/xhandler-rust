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

import io.github.project.openubl.xmlbuilderlib.models.output.common.ClienteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.FirmanteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.FormaPagoOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.ProveedorOutputModel;

import javax.validation.Valid;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotEmpty;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Size;
import java.util.List;

public abstract class DocumentOutputModel {

    @NotBlank
    @Size(min = 3, max = 3)
    protected String moneda;

    @NotBlank
    protected String serieNumero;

    @NotBlank
    protected String horaEmision;

    @NotBlank
    protected String fechaEmision;

    @Valid
    @NotNull
    protected ClienteOutputModel cliente;

    @Valid
    @NotNull
    protected FirmanteOutputModel firmante;

    @Valid
    @NotNull
    protected ProveedorOutputModel proveedor;

    @Valid
    @NotNull
    protected DocumentMonetaryTotalOutputModel totales;

    @Valid
    @NotNull
    protected DocumentImpuestosOutputModel impuestos;

    @Valid
    @NotEmpty
    protected List<DocumentLineOutputModel> detalle;

    @Valid
    protected FormaPagoOutputModel formaPago;

    @Valid
    @NotEmpty
    protected List<GuiaRemisionRelacionadaOutputModel> guiasRemisionRelacionadas;

    public String getMoneda() {
        return moneda;
    }

    public void setMoneda(String moneda) {
        this.moneda = moneda;
    }

    public String getSerieNumero() {
        return serieNumero;
    }

    public void setSerieNumero(String serieNumero) {
        this.serieNumero = serieNumero;
    }

    public String getHoraEmision() {
        return horaEmision;
    }

    public void setHoraEmision(String horaEmision) {
        this.horaEmision = horaEmision;
    }

    public String getFechaEmision() {
        return fechaEmision;
    }

    public void setFechaEmision(String fechaEmision) {
        this.fechaEmision = fechaEmision;
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

    public ProveedorOutputModel getProveedor() {
        return proveedor;
    }

    public void setProveedor(ProveedorOutputModel proveedor) {
        this.proveedor = proveedor;
    }

    public DocumentMonetaryTotalOutputModel getTotales() {
        return totales;
    }

    public void setTotales(DocumentMonetaryTotalOutputModel totales) {
        this.totales = totales;
    }

    public DocumentImpuestosOutputModel getImpuestos() {
        return impuestos;
    }

    public void setImpuestos(DocumentImpuestosOutputModel impuestos) {
        this.impuestos = impuestos;
    }

    public List<DocumentLineOutputModel> getDetalle() {
        return detalle;
    }

    public void setDetalle(List<DocumentLineOutputModel> detalle) {
        this.detalle = detalle;
    }

    public FormaPagoOutputModel getFormaPago() {
        return formaPago;
    }

    public void setFormaPago(FormaPagoOutputModel formaPago) {
        this.formaPago = formaPago;
    }

    public List<GuiaRemisionRelacionadaOutputModel> getGuiasRemisionRelacionadas() {
        return guiasRemisionRelacionadas;
    }

    public void setGuiasRemisionRelacionadas(List<GuiaRemisionRelacionadaOutputModel> guiasRemisionRelacionadas) {
        this.guiasRemisionRelacionadas = guiasRemisionRelacionadas;
    }

    public static class Builder {
        protected String moneda;
        protected String serieNumero;
        protected String horaEmision;
        protected String fechaEmision;
        protected ClienteOutputModel cliente;
        protected FirmanteOutputModel firmante;
        protected ProveedorOutputModel proveedor;
        protected DocumentMonetaryTotalOutputModel totales;
        protected DocumentImpuestosOutputModel impuestos;
        protected List<DocumentLineOutputModel> detalle;
        protected FormaPagoOutputModel formaPago;
        protected List<GuiaRemisionRelacionadaOutputModel> guiasRemisionRelacionadas;

        protected Builder() {
        }

        public static Builder aDocumentOutputModel() {
            return new Builder();
        }

        public Builder withMoneda(String moneda) {
            this.moneda = moneda;
            return this;
        }

        public Builder withSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
            return this;
        }

        public Builder withHoraEmision(String horaEmision) {
            this.horaEmision = horaEmision;
            return this;
        }

        public Builder withFechaEmision(String fechaEmision) {
            this.fechaEmision = fechaEmision;
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

        public Builder withProveedor(ProveedorOutputModel proveedor) {
            this.proveedor = proveedor;
            return this;
        }

        public Builder withTotales(DocumentMonetaryTotalOutputModel totales) {
            this.totales = totales;
            return this;
        }

        public Builder withImpuestos(DocumentImpuestosOutputModel impuestos) {
            this.impuestos = impuestos;
            return this;
        }

        public Builder withDetalle(List<DocumentLineOutputModel> detalle) {
            this.detalle = detalle;
            return this;
        }

        public Builder withFormaPago(FormaPagoOutputModel formaPago) {
            this.formaPago = formaPago;
            return this;
        }

        public Builder withGuiasRemisionRelacionadas(List<GuiaRemisionRelacionadaOutputModel> guiasRemisionRelacionadas) {
            this.guiasRemisionRelacionadas = guiasRemisionRelacionadas;
            return this;
        }
    }
}
