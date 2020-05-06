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
package io.github.project.openubl.xmlbuilderlib.models.input.standard.despatchadvice;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog1;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog21;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.constraints.CatalogConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.FirmanteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;

import javax.validation.Valid;
import javax.validation.constraints.*;
import java.util.List;

public class DespatchAdviceInputModel {

    @NotNull
    @NotBlank
    @Pattern(regexp = "^[T].*$")
    @Size(min = 4, max = 4)
    protected String serie;

    @Min(1)
    @NotNull
    private Integer numero;

    private Long fechaEmision;
    private String observacion;

    @Valid
    private DespatchAdviceInputModel.GuiaRemisionBajaInputModel guiaRemisionDadaDeBaja;

    @Valid
    private DespatchAdviceInputModel.DocumentoAdicionalRelacionadoInputModel documentoAdicionalRelacionado;

    @Valid
    private FirmanteInputModel firmante;

    @Valid
    @NotNull
    private ProveedorInputModel remitente;

    @Valid
    @NotNull
    private ClienteInputModel destinatario;

    @Valid
    @NotNull
    private DespatchAdviceTrasladoInputModel traslado;

    @Valid
    private ClienteInputModel transportista;

    @Valid
    private ClienteInputModel conductor;

    @Valid
    private DespatchAdviceInputModel.VehiculoInputModel vehiculo;

    @Valid
    @NotNull
    @NotEmpty
    private List<DespatchAdviceLineDetalleInputModel> detalle;

    public String getSerie() {
        return serie;
    }

    public void setSerie(String serie) {
        this.serie = serie;
    }

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

    public String getObservacion() {
        return observacion;
    }

    public void setObservacion(String observacion) {
        this.observacion = observacion;
    }

    public GuiaRemisionBajaInputModel getGuiaRemisionDadaDeBaja() {
        return guiaRemisionDadaDeBaja;
    }

    public void setGuiaRemisionDadaDeBaja(GuiaRemisionBajaInputModel guiaRemisionDadaDeBaja) {
        this.guiaRemisionDadaDeBaja = guiaRemisionDadaDeBaja;
    }

    public DocumentoAdicionalRelacionadoInputModel getDocumentoAdicionalRelacionado() {
        return documentoAdicionalRelacionado;
    }

    public void setDocumentoAdicionalRelacionado(DocumentoAdicionalRelacionadoInputModel documentoAdicionalRelacionado) {
        this.documentoAdicionalRelacionado = documentoAdicionalRelacionado;
    }

    public FirmanteInputModel getFirmante() {
        return firmante;
    }

    public void setFirmante(FirmanteInputModel firmante) {
        this.firmante = firmante;
    }

    public ProveedorInputModel getRemitente() {
        return remitente;
    }

    public void setRemitente(ProveedorInputModel remitente) {
        this.remitente = remitente;
    }

    public ClienteInputModel getDestinatario() {
        return destinatario;
    }

    public void setDestinatario(ClienteInputModel destinatario) {
        this.destinatario = destinatario;
    }

    public DespatchAdviceTrasladoInputModel getTraslado() {
        return traslado;
    }

    public void setTraslado(DespatchAdviceTrasladoInputModel traslado) {
        this.traslado = traslado;
    }

    public ClienteInputModel getTransportista() {
        return transportista;
    }

    public void setTransportista(ClienteInputModel transportista) {
        this.transportista = transportista;
    }

    public ClienteInputModel getConductor() {
        return conductor;
    }

    public void setConductor(ClienteInputModel conductor) {
        this.conductor = conductor;
    }

    public VehiculoInputModel getVehiculo() {
        return vehiculo;
    }

    public void setVehiculo(VehiculoInputModel vehiculo) {
        this.vehiculo = vehiculo;
    }

    public List<DespatchAdviceLineDetalleInputModel> getDetalle() {
        return detalle;
    }

    public void setDetalle(List<DespatchAdviceLineDetalleInputModel> detalle) {
        this.detalle = detalle;
    }

    public static class GuiaRemisionBajaInputModel {
        @NotBlank
        private String serieNumero;

        @NotBlank
        @CatalogConstraint(value = Catalog1.class)
        private String tipoDocumento;

        public String getSerieNumero() {
            return serieNumero;
        }

        public void setSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
        }

        public String getTipoDocumento() {
            return tipoDocumento;
        }

        public void setTipoDocumento(String tipoDocumento) {
            this.tipoDocumento = tipoDocumento;
        }
    }

    public static class DocumentoAdicionalRelacionadoInputModel {
        @NotBlank
        private String serieNumero;

        @NotBlank
        @CatalogConstraint(value = Catalog21.class)
        private String tipoDocumento;

        public String getSerieNumero() {
            return serieNumero;
        }

        public void setSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
        }

        public String getTipoDocumento() {
            return tipoDocumento;
        }

        public void setTipoDocumento(String tipoDocumento) {
            this.tipoDocumento = tipoDocumento;
        }
    }

    public static class VehiculoInputModel {
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
