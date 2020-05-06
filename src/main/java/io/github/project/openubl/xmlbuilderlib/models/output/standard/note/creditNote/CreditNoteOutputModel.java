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
package io.github.project.openubl.xmlbuilderlib.models.output.standard.note.creditNote;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog1;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog9;
import io.github.project.openubl.xmlbuilderlib.models.output.common.ClienteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.FirmanteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.ProveedorOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.DocumentImpuestosOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.DocumentLineOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.DocumentMonetaryTotalOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.note.NoteOutputModel;

import javax.validation.constraints.NotNull;
import java.util.List;

public class CreditNoteOutputModel extends NoteOutputModel {

    @NotNull
    protected Catalog9 tipoNota;

    public Catalog9 getTipoNota() {
        return tipoNota;
    }

    public void setTipoNota(Catalog9 tipoNota) {
        this.tipoNota = tipoNota;
    }

    public static final class Builder extends NoteOutputModel.Builder {
        protected String serieNumeroComprobanteAfectado;
        protected String descripcionSustentoDeNota;
        protected Catalog1 tipoDocumentoComprobanteAfectado;
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
        protected Catalog9 tipoNota;

        private Builder() {
        }

        public static Builder aCreditNoteOutputModel() {
            return new Builder();
        }

        public Builder withTipoNota(Catalog9 tipoNota) {
            this.tipoNota = tipoNota;
            return this;
        }

        public Builder withSerieNumeroComprobanteAfectado(String serieNumeroComprobanteAfectado) {
            this.serieNumeroComprobanteAfectado = serieNumeroComprobanteAfectado;
            return this;
        }

        public Builder withDescripcionSustentoDeNota(String descripcionSustentoDeNota) {
            this.descripcionSustentoDeNota = descripcionSustentoDeNota;
            return this;
        }

        public Builder withTipoDocumentoComprobanteAfectado(Catalog1 tipoDocumentoComprobanteAfectado) {
            this.tipoDocumentoComprobanteAfectado = tipoDocumentoComprobanteAfectado;
            return this;
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

        public CreditNoteOutputModel build() {
            CreditNoteOutputModel creditNoteOutputModel = new CreditNoteOutputModel();
            creditNoteOutputModel.setTipoNota(tipoNota);
            creditNoteOutputModel.setSerieNumeroComprobanteAfectado(serieNumeroComprobanteAfectado);
            creditNoteOutputModel.setDescripcionSustentoDeNota(descripcionSustentoDeNota);
            creditNoteOutputModel.setTipoDocumentoComprobanteAfectado(tipoDocumentoComprobanteAfectado);
            creditNoteOutputModel.setMoneda(moneda);
            creditNoteOutputModel.setSerieNumero(serieNumero);
            creditNoteOutputModel.setHoraEmision(horaEmision);
            creditNoteOutputModel.setFechaEmision(fechaEmision);
            creditNoteOutputModel.setCliente(cliente);
            creditNoteOutputModel.setFirmante(firmante);
            creditNoteOutputModel.setProveedor(proveedor);
            creditNoteOutputModel.setTotales(totales);
            creditNoteOutputModel.setImpuestos(impuestos);
            creditNoteOutputModel.setDetalle(detalle);
            return creditNoteOutputModel;
        }
    }
}
