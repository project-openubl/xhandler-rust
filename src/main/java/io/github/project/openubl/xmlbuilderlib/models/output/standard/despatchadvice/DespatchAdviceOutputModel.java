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
package io.github.project.openubl.xmlbuilderlib.models.output.standard.despatchadvice;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog1;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog21;
import io.github.project.openubl.xmlbuilderlib.models.output.common.ClienteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.FirmanteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.ProveedorOutputModel;

import javax.validation.Valid;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotEmpty;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Pattern;
import java.util.List;

public class DespatchAdviceOutputModel {

    @NotBlank
    @Pattern(regexp = "^[T].*$")
    private String serieNumero;

    @NotBlank
    private String fechaEmision;

    private String observacion;

    @Valid
    private DespatchAdviceOutputModel.DocumentoRelacionadoOutputModel<Catalog1> guiaRemisionDadaDeBaja;

    @Valid
    private DespatchAdviceOutputModel.DocumentoRelacionadoOutputModel<Catalog21> documentoAdicionalRelacionado;

    @Valid
    @NotNull
    private FirmanteOutputModel firmante;

    @Valid
    @NotNull
    private ProveedorOutputModel remitente;

    @Valid
    @NotNull
    private ClienteOutputModel destinatario;

    @Valid
    @NotNull
    private DespatchAdviceTrasladoOutputModel traslado;

    @Valid
    private ClienteOutputModel transportista;

    @Valid
    private ClienteOutputModel conductor;

    @Valid
    private DespatchAdviceOutputModel.VehiculoOutputModel vehiculo;

    @Valid
    @NotNull
    @NotEmpty
    private List<DespatchAdviceLineDetalleOutputModel> detalle;

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

    public String getObservacion() {
        return observacion;
    }

    public void setObservacion(String observacion) {
        this.observacion = observacion;
    }

    public DocumentoRelacionadoOutputModel<Catalog1> getGuiaRemisionDadaDeBaja() {
        return guiaRemisionDadaDeBaja;
    }

    public void setGuiaRemisionDadaDeBaja(DocumentoRelacionadoOutputModel<Catalog1> guiaRemisionDadaDeBaja) {
        this.guiaRemisionDadaDeBaja = guiaRemisionDadaDeBaja;
    }

    public DocumentoRelacionadoOutputModel<Catalog21> getDocumentoAdicionalRelacionado() {
        return documentoAdicionalRelacionado;
    }

    public void setDocumentoAdicionalRelacionado(DocumentoRelacionadoOutputModel<Catalog21> documentoAdicionalRelacionado) {
        this.documentoAdicionalRelacionado = documentoAdicionalRelacionado;
    }

    public FirmanteOutputModel getFirmante() {
        return firmante;
    }

    public void setFirmante(FirmanteOutputModel firmante) {
        this.firmante = firmante;
    }

    public ProveedorOutputModel getRemitente() {
        return remitente;
    }

    public void setRemitente(ProveedorOutputModel remitente) {
        this.remitente = remitente;
    }

    public ClienteOutputModel getDestinatario() {
        return destinatario;
    }

    public void setDestinatario(ClienteOutputModel destinatario) {
        this.destinatario = destinatario;
    }

    public DespatchAdviceTrasladoOutputModel getTraslado() {
        return traslado;
    }

    public void setTraslado(DespatchAdviceTrasladoOutputModel traslado) {
        this.traslado = traslado;
    }

    public ClienteOutputModel getTransportista() {
        return transportista;
    }

    public void setTransportista(ClienteOutputModel transportista) {
        this.transportista = transportista;
    }

    public ClienteOutputModel getConductor() {
        return conductor;
    }

    public void setConductor(ClienteOutputModel conductor) {
        this.conductor = conductor;
    }

    public VehiculoOutputModel getVehiculo() {
        return vehiculo;
    }

    public void setVehiculo(VehiculoOutputModel vehiculo) {
        this.vehiculo = vehiculo;
    }

    public List<DespatchAdviceLineDetalleOutputModel> getDetalle() {
        return detalle;
    }

    public void setDetalle(List<DespatchAdviceLineDetalleOutputModel> detalle) {
        this.detalle = detalle;
    }

    public static class DocumentoRelacionadoOutputModel<T extends Catalog> {
        @NotBlank
        private String serieNumero;

        @NotNull
        private T tipoDocumento;

        public String getSerieNumero() {
            return serieNumero;
        }

        public void setSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
        }

        public T getTipoDocumento() {
            return tipoDocumento;
        }

        public void setTipoDocumento(T tipoDocumento) {
            this.tipoDocumento = tipoDocumento;
        }
    }

    public static class VehiculoOutputModel {
        @NotBlank
        private String placa;

        public String getPlaca() {
            return placa;
        }

        public void setPlaca(String placa) {
            this.placa = placa;
        }
    }

}
