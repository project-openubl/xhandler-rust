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

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog23;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.constraints.CatalogConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ClienteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.FirmanteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;

import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Pattern;
import javax.validation.constraints.Size;
import java.util.List;

public class RetentionInputModel extends PerceptionRetentionInputModel {

    @NotNull
    @NotBlank
    @Pattern(regexp = "^[R].*$")
    @Size(min = 4, max = 4)
    private String serie;

    @CatalogConstraint(value = Catalog23.class)
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
        private String regimen;
        private ProveedorInputModel proveedor;
        private ClienteInputModel cliente;
        private FirmanteInputModel firmante;
        private List<PerceptionRetentionLineInputModel> detalle;

        private Builder() {
        }

        public static Builder aRetentionInputModel() {
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

        public Builder withRegimen(String regimen) {
            this.regimen = regimen;
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

        public Builder withFirmante(FirmanteInputModel firmante) {
            this.firmante = firmante;
            return this;
        }

        public Builder withDetalle(List<PerceptionRetentionLineInputModel> detalle) {
            this.detalle = detalle;
            return this;
        }

        public RetentionInputModel build() {
            RetentionInputModel retentionInputModel = new RetentionInputModel();
            retentionInputModel.setNumero(numero);
            retentionInputModel.setSerie(serie);
            retentionInputModel.setFechaEmision(fechaEmision);
            retentionInputModel.setObservacion(observacion);
            retentionInputModel.setRegimen(regimen);
            retentionInputModel.setProveedor(proveedor);
            retentionInputModel.setCliente(cliente);
            retentionInputModel.setFirmante(firmante);
            retentionInputModel.setDetalle(detalle);
            return retentionInputModel;
        }
    }
}
