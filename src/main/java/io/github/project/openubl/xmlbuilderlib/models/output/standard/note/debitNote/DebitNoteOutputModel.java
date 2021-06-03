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
package io.github.project.openubl.xmlbuilderlib.models.output.standard.note.debitNote;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog1;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog10;
import io.github.project.openubl.xmlbuilderlib.models.output.common.ClienteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.FirmanteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.FormaPagoOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.ProveedorOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.DocumentImpuestosOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.DocumentLineOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.DocumentMonetaryTotalOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.note.NoteOutputModel;

import javax.validation.constraints.NotNull;
import java.util.List;

public class DebitNoteOutputModel extends NoteOutputModel {

    @NotNull
    protected Catalog10 tipoNota;

    public Catalog10 getTipoNota() {
        return tipoNota;
    }

    public void setTipoNota(Catalog10 tipoNota) {
        this.tipoNota = tipoNota;
    }

    public static final class Builder extends NoteOutputModel.Builder {
        protected String moneda;
        protected String serieNumero;
        protected String horaEmision;
        protected String fechaEmision;
        protected String serieNumeroComprobanteAfectado;
        protected Catalog10 tipoNota;
        protected ClienteOutputModel cliente;
        protected String descripcionSustentoDeNota;
        protected FirmanteOutputModel firmante;
        protected Catalog1 tipoDocumentoComprobanteAfectado;
        protected ProveedorOutputModel proveedor;
        protected DocumentMonetaryTotalOutputModel totales;
        protected DocumentImpuestosOutputModel impuestos;
        protected List<DocumentLineOutputModel> detalle;
        protected FormaPagoOutputModel formaPago;

        private Builder() {
        }

        public static Builder aDebitNoteOutputModel() {
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

        public Builder withSerieNumeroComprobanteAfectado(String serieNumeroComprobanteAfectado) {
            this.serieNumeroComprobanteAfectado = serieNumeroComprobanteAfectado;
            return this;
        }

        public Builder withTipoNota(Catalog10 tipoNota) {
            this.tipoNota = tipoNota;
            return this;
        }

        public Builder withCliente(ClienteOutputModel cliente) {
            this.cliente = cliente;
            return this;
        }

        public Builder withDescripcionSustentoDeNota(String descripcionSustentoDeNota) {
            this.descripcionSustentoDeNota = descripcionSustentoDeNota;
            return this;
        }

        public Builder withFirmante(FirmanteOutputModel firmante) {
            this.firmante = firmante;
            return this;
        }

        public Builder withTipoDocumentoComprobanteAfectado(Catalog1 tipoDocumentoComprobanteAfectado) {
            this.tipoDocumentoComprobanteAfectado = tipoDocumentoComprobanteAfectado;
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

        public DebitNoteOutputModel build() {
            DebitNoteOutputModel debitNoteOutputModel = new DebitNoteOutputModel();
            debitNoteOutputModel.setMoneda(moneda);
            debitNoteOutputModel.setSerieNumero(serieNumero);
            debitNoteOutputModel.setHoraEmision(horaEmision);
            debitNoteOutputModel.setFechaEmision(fechaEmision);
            debitNoteOutputModel.setSerieNumeroComprobanteAfectado(serieNumeroComprobanteAfectado);
            debitNoteOutputModel.setTipoNota(tipoNota);
            debitNoteOutputModel.setCliente(cliente);
            debitNoteOutputModel.setDescripcionSustentoDeNota(descripcionSustentoDeNota);
            debitNoteOutputModel.setFirmante(firmante);
            debitNoteOutputModel.setTipoDocumentoComprobanteAfectado(tipoDocumentoComprobanteAfectado);
            debitNoteOutputModel.setProveedor(proveedor);
            debitNoteOutputModel.setTotales(totales);
            debitNoteOutputModel.setImpuestos(impuestos);
            debitNoteOutputModel.setDetalle(detalle);
            debitNoteOutputModel.setFormaPago(formaPago);
            debitNoteOutputModel.setGuiasRemisionRelacionadas(guiasRemisionRelacionadas);
            debitNoteOutputModel.setOtrosDocumentosTributariosRelacionados(otrosDocumentosTributariosRelacionados);
            return debitNoteOutputModel;
        }
    }
}
