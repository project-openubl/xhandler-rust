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
package io.github.project.openubl.xmlbuilderlib.models.input.standard.note.debitNote;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog10;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.constraints.CatalogConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.FirmanteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocumentLineInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.GuiaRemisionRelacionadaInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.note.NoteInputModel;

import javax.validation.Valid;
import java.util.List;

public class DebitNoteInputModel extends NoteInputModel {

    @CatalogConstraint(value = Catalog10.class)
    private String tipoNota;

    @Valid
    private List<DocTribRelacionadoInputModel_DebitNote> otrosDocumentosTributariosRelacionados;

    public String getTipoNota() {
        return tipoNota;
    }

    public void setTipoNota(String tipoNota) {
        this.tipoNota = tipoNota;
    }

    public List<DocTribRelacionadoInputModel_DebitNote> getOtrosDocumentosTributariosRelacionados() {
        return otrosDocumentosTributariosRelacionados;
    }

    public void setOtrosDocumentosTributariosRelacionados(List<DocTribRelacionadoInputModel_DebitNote> otrosDocumentosTributariosRelacionados) {
        this.otrosDocumentosTributariosRelacionados = otrosDocumentosTributariosRelacionados;
    }

    public static final class Builder {
        protected String serie;
        private String serieNumeroComprobanteAfectado;
        private String tipoNota;
        private String descripcionSustento;
        private Integer numero;
        private Long fechaEmision;
        private ClienteInputModel cliente;
        private ProveedorInputModel proveedor;
        private FirmanteInputModel firmante;
        private List<DocumentLineInputModel> detalle;
        private List<GuiaRemisionRelacionadaInputModel> guiasRemisionRelacionadas;
        private List<DocTribRelacionadoInputModel_DebitNote> otrosDocumentosTributariosRelacionados;

        private Builder() {
        }

        public static Builder aDebitNoteInputModel() {
            return new Builder();
        }

        public Builder withSerieNumeroComprobanteAfectado(String serieNumeroComprobanteAfectado) {
            this.serieNumeroComprobanteAfectado = serieNumeroComprobanteAfectado;
            return this;
        }

        public Builder withTipoNota(String tipoNota) {
            this.tipoNota = tipoNota;
            return this;
        }

        public Builder withDescripcionSustento(String descripcionSustento) {
            this.descripcionSustento = descripcionSustento;
            return this;
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

        public Builder withOtrosDocumentosTributariosRelacionados(List<DocTribRelacionadoInputModel_DebitNote> otrosDocumentosTributariosRelacionados) {
            this.otrosDocumentosTributariosRelacionados = otrosDocumentosTributariosRelacionados;
            return this;
        }

        public DebitNoteInputModel build() {
            DebitNoteInputModel debitNoteInputModel = new DebitNoteInputModel();
            debitNoteInputModel.setSerieNumeroComprobanteAfectado(serieNumeroComprobanteAfectado);
            debitNoteInputModel.setTipoNota(tipoNota);
            debitNoteInputModel.setDescripcionSustentoDeNota(descripcionSustento);
            debitNoteInputModel.setSerie(serie);
            debitNoteInputModel.setNumero(numero);
            debitNoteInputModel.setFechaEmision(fechaEmision);
            debitNoteInputModel.setCliente(cliente);
            debitNoteInputModel.setProveedor(proveedor);
            debitNoteInputModel.setFirmante(firmante);
            debitNoteInputModel.setDetalle(detalle);
            debitNoteInputModel.setGuiasRemisionRelacionadas(guiasRemisionRelacionadas);
            debitNoteInputModel.setOtrosDocumentosTributariosRelacionados(otrosDocumentosTributariosRelacionados);
            return debitNoteInputModel;
        }
    }
}
