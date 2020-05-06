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
package io.github.project.openubl.xmlbuilderlib.models.input.sunat;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog22;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.constraints.CatalogConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.FirmanteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import org.eclipse.microprofile.openapi.annotations.media.Schema;

import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Pattern;
import javax.validation.constraints.Size;
import java.util.List;

public class PerceptionInputModel extends PerceptionRetentionInputModel {

    @NotNull
    @NotBlank
    @Pattern(regexp = "^[P].*$")
    @Size(min = 4, max = 4)
    private String serie;

    @CatalogConstraint(value = Catalog22.class)
    @Schema(example = "VENTA_INTERNA", description = "Catalogo 22", enumeration = {
            "VENTA_INTERNA", "01",
            "ADQUISICION_DE_COMBUSTIBLE", "02",
            "AGENTE_DE_PERCEPCION_CON_TASA_ESPECIAL", "03"
    })
    private String regimen;

    public String getSerie() {
        return serie;
    }

    public void setSerie(String serie) {
        this.serie = serie;
    }

    public String getRegimen() {
        return regimen;
    }

    public void setRegimen(String regimen) {
        this.regimen = regimen;
    }

    public static final class Builder {
        private Integer numero;
        private String serie;
        private Long fechaEmision;
        private String observacion;
        private ProveedorInputModel proveedor;
        private ClienteInputModel cliente;
        private String regimen;
        private FirmanteInputModel firmante;
        private List<PerceptionRetentionLineInputModel> detalle;

        private Builder() {
        }

        public static Builder aPerceptionInputModel() {
            return new Builder();
        }

        public Builder withNumero(Integer numero) {
            this.numero = numero;
            return this;
        }

        public Builder withSerie(String serie) {
            this.serie = serie;
            return this;
        }

        public Builder withFechaEmision(Long fechaEmision) {
            this.fechaEmision = fechaEmision;
            return this;
        }

        public Builder withObservacion(String observacion) {
            this.observacion = observacion;
            return this;
        }

        public Builder withProveedor(ProveedorInputModel proveedor) {
            this.proveedor = proveedor;
            return this;
        }

        public Builder withCliente(ClienteInputModel cliente) {
            this.cliente = cliente;
            return this;
        }

        public Builder withRegimen(String regimen) {
            this.regimen = regimen;
            return this;
        }

        public Builder withFirmante(FirmanteInputModel firmante) {
            this.firmante = firmante;
            return this;
        }

        public Builder withDetalle(List<PerceptionRetentionLineInputModel> detalle) {
            this.detalle = detalle;
            return this;
        }

        public PerceptionInputModel build() {
            PerceptionInputModel perceptionInputModel = new PerceptionInputModel();
            perceptionInputModel.setNumero(numero);
            perceptionInputModel.setSerie(serie);
            perceptionInputModel.setFechaEmision(fechaEmision);
            perceptionInputModel.setObservacion(observacion);
            perceptionInputModel.setProveedor(proveedor);
            perceptionInputModel.setCliente(cliente);
            perceptionInputModel.setRegimen(regimen);
            perceptionInputModel.setFirmante(firmante);
            perceptionInputModel.setDetalle(detalle);
            return perceptionInputModel;
        }
    }
}
