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
package io.github.project.openubl.xmlbuilderlib.models.output.standard;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog16;

import javax.validation.constraints.Digits;
import javax.validation.constraints.Min;
import javax.validation.constraints.NotNull;
import java.math.BigDecimal;

public class DocumentLinePrecioReferenciaOutputModel {

    @Min(0)
    @NotNull
    @Digits(integer = 100, fraction = 2)
    private BigDecimal precio;

    @NotNull
    private Catalog16 tipoPrecio;

    public BigDecimal getPrecio() {
        return precio;
    }

    public void setPrecio(BigDecimal precio) {
        this.precio = precio;
    }

    public Catalog16 getTipoPrecio() {
        return tipoPrecio;
    }

    public void setTipoPrecio(Catalog16 tipoPrecio) {
        this.tipoPrecio = tipoPrecio;
    }

    public static final class Builder {
        private BigDecimal precio;
        private Catalog16 tipoPrecio;

        private Builder() {
        }

        public static Builder aDetallePrecioReferenciaOutputModel() {
            return new Builder();
        }

        public Builder withPrecio(BigDecimal precio) {
            this.precio = precio;
            return this;
        }

        public Builder withTipoPrecio(Catalog16 tipoPrecio) {
            this.tipoPrecio = tipoPrecio;
            return this;
        }

        public DocumentLinePrecioReferenciaOutputModel build() {
            DocumentLinePrecioReferenciaOutputModel detallePrecioReferenciaOutputModel = new DocumentLinePrecioReferenciaOutputModel();
            detallePrecioReferenciaOutputModel.setPrecio(precio);
            detallePrecioReferenciaOutputModel.setTipoPrecio(tipoPrecio);
            return detallePrecioReferenciaOutputModel;
        }
    }
}
