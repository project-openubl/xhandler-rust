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
package io.github.project.openubl.xmlbuilderlib.models.output.standard.invoice;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog1;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.DocumentOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.DocumentoTributarioRelacionadoOutputModel;

import javax.validation.Valid;
import javax.validation.constraints.NotNull;
import java.util.List;

public class InvoiceOutputModel extends DocumentOutputModel {

    @NotNull
    private Catalog1 tipoInvoice;

    @NotNull
    @Valid
    private List<AnticipoOutputModel> anticipos;

    public Catalog1 getTipoInvoice() {
        return tipoInvoice;
    }

    public void setTipoInvoice(Catalog1 tipoInvoice) {
        this.tipoInvoice = tipoInvoice;
    }

    public List<AnticipoOutputModel> getAnticipos() {
        return anticipos;
    }

    public void setAnticipos(List<AnticipoOutputModel> anticipos) {
        this.anticipos = anticipos;
    }

    public static final class Builder extends DocumentOutputModel.Builder {
        private Catalog1 tipoInvoice;
        private List<AnticipoOutputModel> anticipos;

        private Builder() {
        }

        public static Builder anInvoiceOutputModel() {
            return new Builder();
        }

        public Builder withTipoInvoice(Catalog1 tipoInvoice) {
            this.tipoInvoice = tipoInvoice;
            return this;
        }

        public Builder withAnticipos(List<AnticipoOutputModel> anticipos) {
            this.anticipos = anticipos;
            return this;
        }

        public InvoiceOutputModel build() {
            InvoiceOutputModel invoiceOutputModel = new InvoiceOutputModel();
            invoiceOutputModel.setMoneda(moneda);
            invoiceOutputModel.setSerieNumero(serieNumero);
            invoiceOutputModel.setHoraEmision(horaEmision);
            invoiceOutputModel.setFechaEmision(fechaEmision);
            invoiceOutputModel.setCliente(cliente);
            invoiceOutputModel.setFirmante(firmante);
            invoiceOutputModel.setProveedor(proveedor);
            invoiceOutputModel.setTotales(totales);
            invoiceOutputModel.setImpuestos(impuestos);
            invoiceOutputModel.setDetalle(detalle);
            invoiceOutputModel.setTipoInvoice(tipoInvoice);
            invoiceOutputModel.setFormaPago(formaPago);
            invoiceOutputModel.setGuiasRemisionRelacionadas(guiasRemisionRelacionadas);
            invoiceOutputModel.setOtrosDocumentosTributariosRelacionados(otrosDocumentosTributariosRelacionados);
            invoiceOutputModel.setAnticipos(anticipos);
            return invoiceOutputModel;
        }
    }
}
