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
package io.github.project.openubl.xmlbuilderlib.models.input.standard.invoice;

import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.CuotaDePagoInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.FirmanteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.CuotaDePagoInputModel_Porcentaje100CollectionConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.HighLevelGroupValidation;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocumentInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocumentLineInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.GuiaRemisionRelacionadaInputModel;

import javax.validation.Valid;
import java.util.List;

public class InvoiceInputModel extends DocumentInputModel {

    @Valid
    @CuotaDePagoInputModel_Porcentaje100CollectionConstraint(groups = HighLevelGroupValidation.class)
    private List<CuotaDePagoInputModel> cuotasDePago;

    @Valid
    private List<DocTribRelacionadoInputModel_Invoice> otrosDocumentosTributariosRelacionados;

    @Nullable
    @Valid
    private List<AnticipoInputModel> anticipos;

    public List<CuotaDePagoInputModel> getCuotasDePago() {
        return cuotasDePago;
    }

    public void setCuotasDePago(List<CuotaDePagoInputModel> cuotasDePago) {
        this.cuotasDePago = cuotasDePago;
    }

    @Override
    public List<DocTribRelacionadoInputModel_Invoice> getOtrosDocumentosTributariosRelacionados() {
        return otrosDocumentosTributariosRelacionados;
    }

    public void setOtrosDocumentosTributariosRelacionados(List<DocTribRelacionadoInputModel_Invoice> otrosDocumentosTributariosRelacionados) {
        this.otrosDocumentosTributariosRelacionados = otrosDocumentosTributariosRelacionados;
    }

    public List<AnticipoInputModel> getAnticipos() {
        return anticipos;
    }

    public void setAnticipos(List<AnticipoInputModel> anticipos) {
        this.anticipos = anticipos;
    }

    public static final class Builder {
        protected String serie;
        private Integer numero;
        private Long fechaEmision;
        private ClienteInputModel cliente;
        private ProveedorInputModel proveedor;
        private FirmanteInputModel firmante;
        private List<DocumentLineInputModel> detalle;
        private List<GuiaRemisionRelacionadaInputModel> guiasRemisionRelacionadas;
        private List<CuotaDePagoInputModel> cuotasDePago;
        private List<DocTribRelacionadoInputModel_Invoice> otrosDocumentosTributariosRelacionados;
        private List<AnticipoInputModel> anticipos;

        private Builder() {
        }

        public static Builder anInvoiceInputModel() {
            return new Builder();
        }

        public Builder withSerie(String serie) {
            this.serie = serie;
            return this;
        }

        public Builder withNumero(Integer numero) {
            this.numero = numero;
            return this;
        }

        public Builder withFechaEmision(Long fechaEmision) {
            this.fechaEmision = fechaEmision;
            return this;
        }

        public Builder withCliente(ClienteInputModel cliente) {
            this.cliente = cliente;
            return this;
        }

        public Builder withProveedor(ProveedorInputModel proveedor) {
            this.proveedor = proveedor;
            return this;
        }

        public Builder withFirmante(FirmanteInputModel firmante) {
            this.firmante = firmante;
            return this;
        }

        public Builder withDetalle(List<DocumentLineInputModel> detalle) {
            this.detalle = detalle;
            return this;
        }

        public Builder withGuiasRemisionRelacionadas(List<GuiaRemisionRelacionadaInputModel> guiasRemisionRelacionadas) {
            this.guiasRemisionRelacionadas = guiasRemisionRelacionadas;
            return this;
        }

        public Builder withCuotasDePago(List<CuotaDePagoInputModel> cuotasDePago) {
            this.cuotasDePago = cuotasDePago;
            return this;
        }

        public Builder withOtrosDocumentosTributariosRelacionados(List<DocTribRelacionadoInputModel_Invoice> otrosDocumentosTributariosRelacionados) {
            this.otrosDocumentosTributariosRelacionados = otrosDocumentosTributariosRelacionados;
            return this;
        }

        public Builder withAnticipos(List<AnticipoInputModel> anticipos) {
            this.anticipos = anticipos;
            return this;
        }

        public InvoiceInputModel build() {
            InvoiceInputModel invoiceInputModel = new InvoiceInputModel();
            invoiceInputModel.setSerie(serie);
            invoiceInputModel.setNumero(numero);
            invoiceInputModel.setFechaEmision(fechaEmision);
            invoiceInputModel.setCliente(cliente);
            invoiceInputModel.setProveedor(proveedor);
            invoiceInputModel.setFirmante(firmante);
            invoiceInputModel.setDetalle(detalle);
            invoiceInputModel.setGuiasRemisionRelacionadas(guiasRemisionRelacionadas);
            invoiceInputModel.setCuotasDePago(cuotasDePago);
            invoiceInputModel.setOtrosDocumentosTributariosRelacionados(otrosDocumentosTributariosRelacionados);
            invoiceInputModel.setAnticipos(anticipos);
            return invoiceInputModel;
        }
    }
}
