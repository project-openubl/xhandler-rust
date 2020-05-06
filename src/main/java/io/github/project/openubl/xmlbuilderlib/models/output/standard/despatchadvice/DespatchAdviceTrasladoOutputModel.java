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

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog18;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog20;

import javax.validation.Valid;
import javax.validation.constraints.Min;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Size;
import java.math.BigDecimal;

public class DespatchAdviceTrasladoOutputModel {

    @NotNull
    private Catalog20 motivo;

    private String descripcion;

    @NotBlank
    private String pesoBrutoUnidadMedida;

    @Min(0)
    @NotNull
    private BigDecimal pesoBrutoTotal;

    @Min(0)
    private Integer numeroBultos;

    @NotNull
    private String transbordoProgramado;

    @NotNull
    private Catalog18 modalidad;

    @NotNull
    private String fechaInicio;

    private String codigoPuertoAeropuertoDeEmbarqueOdesembarque;

    @Valid
    private DespatchAdviceTrasladoOutputModel.PuntoOutputModel puntoPartida;

    @Valid
    @NotNull
    private DespatchAdviceTrasladoOutputModel.PuntoOutputModel puntoLlegada;

    public Catalog20 getMotivo() {
        return motivo;
    }

    public void setMotivo(Catalog20 motivo) {
        this.motivo = motivo;
    }

    public String getDescripcion() {
        return descripcion;
    }

    public void setDescripcion(String descripcion) {
        this.descripcion = descripcion;
    }

    public String getPesoBrutoUnidadMedida() {
        return pesoBrutoUnidadMedida;
    }

    public void setPesoBrutoUnidadMedida(String pesoBrutoUnidadMedida) {
        this.pesoBrutoUnidadMedida = pesoBrutoUnidadMedida;
    }

    public BigDecimal getPesoBrutoTotal() {
        return pesoBrutoTotal;
    }

    public void setPesoBrutoTotal(BigDecimal pesoBrutoTotal) {
        this.pesoBrutoTotal = pesoBrutoTotal;
    }

    public Integer getNumeroBultos() {
        return numeroBultos;
    }

    public void setNumeroBultos(Integer numeroBultos) {
        this.numeroBultos = numeroBultos;
    }

    public String getTransbordoProgramado() {
        return transbordoProgramado;
    }

    public void setTransbordoProgramado(String transbordoProgramado) {
        this.transbordoProgramado = transbordoProgramado;
    }

    public Catalog18 getModalidad() {
        return modalidad;
    }

    public void setModalidad(Catalog18 modalidad) {
        this.modalidad = modalidad;
    }

    public String getFechaInicio() {
        return fechaInicio;
    }

    public void setFechaInicio(String fechaInicio) {
        this.fechaInicio = fechaInicio;
    }

    public String getCodigoPuertoAeropuertoDeEmbarqueOdesembarque() {
        return codigoPuertoAeropuertoDeEmbarqueOdesembarque;
    }

    public void setCodigoPuertoAeropuertoDeEmbarqueOdesembarque(String codigoPuertoAeropuertoDeEmbarqueOdesembarque) {
        this.codigoPuertoAeropuertoDeEmbarqueOdesembarque = codigoPuertoAeropuertoDeEmbarqueOdesembarque;
    }

    public PuntoOutputModel getPuntoPartida() {
        return puntoPartida;
    }

    public void setPuntoPartida(PuntoOutputModel puntoPartida) {
        this.puntoPartida = puntoPartida;
    }

    public PuntoOutputModel getPuntoLlegada() {
        return puntoLlegada;
    }

    public void setPuntoLlegada(PuntoOutputModel puntoLlegada) {
        this.puntoLlegada = puntoLlegada;
    }

    public static class PuntoOutputModel {
        @NotBlank
        @Size(min = 6, max = 6)
        private String codigoPostal;

        @NotBlank
        private String direccion;

        public String getCodigoPostal() {
            return codigoPostal;
        }

        public void setCodigoPostal(String codigoPostal) {
            this.codigoPostal = codigoPostal;
        }

        public String getDireccion() {
            return direccion;
        }

        public void setDireccion(String direccion) {
            this.direccion = direccion;
        }
    }

}
