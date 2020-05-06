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

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog18;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog20;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.constraints.CatalogConstraint;

import javax.validation.Valid;
import javax.validation.constraints.Min;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Size;
import java.math.BigDecimal;

public class DespatchAdviceTrasladoInputModel {

    @NotNull
    @CatalogConstraint(value = Catalog20.class)
    private String motivo;

    private String descripcion;

    @NotBlank
    private String pesoBrutoUnidadMedida;

    @Min(0)
    @NotNull
    private BigDecimal pesoBrutoTotal;

    @Min(0)
    private Integer numeroBultos;

    private Boolean transbordoProgramado;

    @NotNull
    @CatalogConstraint(value = Catalog18.class)
    private String modalidad;

    @NotNull
    private Long fechaInicio;

    private String codigoPuertoAeropuertoDeEmbarqueOdesembarque;

    @Valid
    private DespatchAdviceTrasladoInputModel.PuntoInputModel puntoPartida;

    @NotNull
    @Valid
    private DespatchAdviceTrasladoInputModel.PuntoInputModel puntoLlegada;

    public String getMotivo() {
        return motivo;
    }

    public void setMotivo(String motivo) {
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

    public Boolean getTransbordoProgramado() {
        return transbordoProgramado;
    }

    public void setTransbordoProgramado(Boolean transbordoProgramado) {
        this.transbordoProgramado = transbordoProgramado;
    }

    public String getModalidad() {
        return modalidad;
    }

    public void setModalidad(String modalidad) {
        this.modalidad = modalidad;
    }

    public Long getFechaInicio() {
        return fechaInicio;
    }

    public void setFechaInicio(Long fechaInicio) {
        this.fechaInicio = fechaInicio;
    }

    public String getCodigoPuertoAeropuertoDeEmbarqueOdesembarque() {
        return codigoPuertoAeropuertoDeEmbarqueOdesembarque;
    }

    public void setCodigoPuertoAeropuertoDeEmbarqueOdesembarque(String codigoPuertoAeropuertoDeEmbarqueOdesembarque) {
        this.codigoPuertoAeropuertoDeEmbarqueOdesembarque = codigoPuertoAeropuertoDeEmbarqueOdesembarque;
    }

    public PuntoInputModel getPuntoPartida() {
        return puntoPartida;
    }

    public void setPuntoPartida(PuntoInputModel puntoPartida) {
        this.puntoPartida = puntoPartida;
    }

    public PuntoInputModel getPuntoLlegada() {
        return puntoLlegada;
    }

    public void setPuntoLlegada(PuntoInputModel puntoLlegada) {
        this.puntoLlegada = puntoLlegada;
    }

    public static class PuntoInputModel {
        @NotBlank
        @Size(min = 6, max = 6)
        private String codigoPostal;

        @NotBlank
        private String direccion;

        public PuntoInputModel() {
        }

        public PuntoInputModel(String codigoPostal, String direccion) {
            this.codigoPostal = codigoPostal;
            this.direccion = direccion;
        }

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
