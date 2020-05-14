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
package io.github.project.openubl.xmlbuilderlib.models.input.standard;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog7;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.constraints.CatalogConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.DocumentLineInputModel_CantidadValidaICBConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.DocumentLineInputModel_CantidadValidaICBGroupValidation;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.DocumentLineInputModel_PrecioConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.HighLevelGroupValidation;

import javax.validation.constraints.Digits;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Positive;
import java.math.BigDecimal;

@DocumentLineInputModel_PrecioConstraint(groups = HighLevelGroupValidation.class)
@DocumentLineInputModel_CantidadValidaICBConstraint(groups = DocumentLineInputModel_CantidadValidaICBGroupValidation.class)
public class DocumentLineInputModel {

    @NotNull
    @NotBlank
    private String descripcion;

    private String unidadMedida;

    @NotNull
    @Positive
    @Digits(integer = 100, fraction = 3)
    private BigDecimal cantidad;

    /**
     * Precio sin impuestos
     */
    @Positive
    @Digits(integer = 100, fraction = 2)
    private BigDecimal precioUnitario;

    /**
     * Precio con impuestos
     */
    @Positive
    @Digits(integer = 100, fraction = 2)
    private BigDecimal precioConIgv;

    @CatalogConstraint(value = Catalog7.class)
    private String tipoIgv;

    private boolean icb;

    public String getDescripcion() {
        return descripcion;
    }

    public void setDescripcion(String descripcion) {
        this.descripcion = descripcion;
    }

    public String getUnidadMedida() {
        return unidadMedida;
    }

    public void setUnidadMedida(String unidadMedida) {
        this.unidadMedida = unidadMedida;
    }

    public BigDecimal getCantidad() {
        return cantidad;
    }

    public void setCantidad(BigDecimal cantidad) {
        this.cantidad = cantidad;
    }

    public BigDecimal getPrecioUnitario() {
        return precioUnitario;
    }

    public void setPrecioUnitario(BigDecimal precioUnitario) {
        this.precioUnitario = precioUnitario;
    }

    public BigDecimal getPrecioConIgv() {
        return precioConIgv;
    }

    public void setPrecioConIgv(BigDecimal precioConIgv) {
        this.precioConIgv = precioConIgv;
    }

    public String getTipoIgv() {
        return tipoIgv;
    }

    public void setTipoIgv(String tipoIgv) {
        this.tipoIgv = tipoIgv;
    }

    public boolean isIcb() {
        return icb;
    }

    public void setIcb(boolean icb) {
        this.icb = icb;
    }

    public static final class Builder {
        private String descripcion;
        private String unidadMedida;
        private BigDecimal cantidad;
        private BigDecimal precioUnitario;
        private BigDecimal precioConIgv;
        private String tipoIgv;
        private boolean icb;

        private Builder() {
        }

        public static Builder aDocumentLineInputModel() {
            return new Builder();
        }

        public Builder withDescripcion(String descripcion) {
            this.descripcion = descripcion;
            return this;
        }

        public Builder withUnidadMedida(String unidadMedida) {
            this.unidadMedida = unidadMedida;
            return this;
        }

        public Builder withCantidad(BigDecimal cantidad) {
            this.cantidad = cantidad;
            return this;
        }

        public Builder withPrecioUnitario(BigDecimal precioUnitario) {
            this.precioUnitario = precioUnitario;
            return this;
        }

        public Builder withPrecioConIgv(BigDecimal precioConIgv) {
            this.precioConIgv = precioConIgv;
            return this;
        }

        public Builder withTipoIgv(String tipoIgv) {
            this.tipoIgv = tipoIgv;
            return this;
        }

        public Builder withIcb(boolean icb) {
            this.icb = icb;
            return this;
        }

        public DocumentLineInputModel build() {
            DocumentLineInputModel documentLineInputModel = new DocumentLineInputModel();
            documentLineInputModel.setDescripcion(descripcion);
            documentLineInputModel.setUnidadMedida(unidadMedida);
            documentLineInputModel.setCantidad(cantidad);
            documentLineInputModel.setPrecioUnitario(precioUnitario);
            documentLineInputModel.setPrecioConIgv(precioConIgv);
            documentLineInputModel.setTipoIgv(tipoIgv);
            documentLineInputModel.setIcb(icb);
            return documentLineInputModel;
        }
    }
}

