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
package io.github.project.openubl.xmlbuilderlib.models.input.standard.invoice;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog12_Anticipo;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.constraints.CatalogConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocTribRelacionadoInputModel;

import javax.validation.constraints.Min;
import javax.validation.constraints.NotNull;
import java.math.BigDecimal;

public class AnticipoInputModel extends DocTribRelacionadoInputModel {

    @CatalogConstraint(value = Catalog12_Anticipo.class)
    protected String tipoDocumento;

    @Min(0)
    @NotNull
    private BigDecimal montoTotal;

    @Override
    public String getTipoDocumento() {
        return tipoDocumento;
    }

    @Override
    public void setTipoDocumento(String tipoDocumento) {
        this.tipoDocumento = tipoDocumento;
    }

    public BigDecimal getMontoTotal() {
        return montoTotal;
    }

    public void setMontoTotal(BigDecimal montoTotal) {
        this.montoTotal = montoTotal;
    }

    public static final class Builder {
        protected String serieNumero;
        protected String tipoDocumento;
        private BigDecimal montoTotal;

        private Builder() {
        }

        public static Builder anAnticipoInputModel() {
            return new Builder();
        }

        public Builder withSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
            return this;
        }

        public Builder withTipoDocumento(String tipoDocumento) {
            this.tipoDocumento = tipoDocumento;
            return this;
        }

        public Builder withMontoTotal(BigDecimal montoTotal) {
            this.montoTotal = montoTotal;
            return this;
        }

        public AnticipoInputModel build() {
            AnticipoInputModel anticipoInputModel = new AnticipoInputModel();
            anticipoInputModel.setSerieNumero(serieNumero);
            anticipoInputModel.setTipoDocumento(tipoDocumento);
            anticipoInputModel.setMontoTotal(montoTotal);
            return anticipoInputModel;
        }
    }
}
