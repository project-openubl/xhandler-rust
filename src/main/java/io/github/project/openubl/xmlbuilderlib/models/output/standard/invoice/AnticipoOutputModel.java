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
package io.github.project.openubl.xmlbuilderlib.models.output.standard.invoice;

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog12;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog53_Anticipo;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.DocumentoTributarioRelacionadoOutputModel;

import javax.validation.constraints.Min;
import javax.validation.constraints.NotNull;
import java.math.BigDecimal;

public class AnticipoOutputModel extends DocumentoTributarioRelacionadoOutputModel {

    @NotNull
    private Catalog53_Anticipo tipoAnticipo;

    @Min(0)
    @NotNull
    private BigDecimal montoTotalConIgv;

    @Min(0)
    @NotNull
    private BigDecimal montoTotalSinIgv;

    public Catalog53_Anticipo getTipoAnticipo() {
        return tipoAnticipo;
    }

    public void setTipoAnticipo(Catalog53_Anticipo tipoAnticipo) {
        this.tipoAnticipo = tipoAnticipo;
    }

    public BigDecimal getMontoTotalConIgv() {
        return montoTotalConIgv;
    }

    public void setMontoTotalConIgv(BigDecimal montoTotalConIgv) {
        this.montoTotalConIgv = montoTotalConIgv;
    }

    public BigDecimal getMontoTotalSinIgv() {
        return montoTotalSinIgv;
    }

    public void setMontoTotalSinIgv(BigDecimal montoTotalSinIgv) {
        this.montoTotalSinIgv = montoTotalSinIgv;
    }

    public static final class Builder {
        private String serieNumero;
        private Catalog12 tipoDocumento;
        private Catalog53_Anticipo tipoAnticipo;
        private BigDecimal montoTotalConIgv;
        private BigDecimal montoTotalSinIgv;

        private Builder() {
        }

        public static Builder anAnticipoOutputModel() {
            return new Builder();
        }

        public Builder withSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
            return this;
        }

        public Builder withTipoDocumento(Catalog12 tipoDocumento) {
            this.tipoDocumento = tipoDocumento;
            return this;
        }

        public Builder withTipoAnticipo(Catalog53_Anticipo tipoAnticipo) {
            this.tipoAnticipo = tipoAnticipo;
            return this;
        }

        public Builder withMontoTotalConIgv(BigDecimal montoTotalConIgv) {
            this.montoTotalConIgv = montoTotalConIgv;
            return this;
        }

        public Builder withMontoTotalSinIgv(BigDecimal montoTotalSinIgv) {
            this.montoTotalSinIgv = montoTotalSinIgv;
            return this;
        }

        public AnticipoOutputModel build() {
            AnticipoOutputModel anticipoOutputModel = new AnticipoOutputModel();
            anticipoOutputModel.setSerieNumero(serieNumero);
            anticipoOutputModel.setTipoDocumento(tipoDocumento);
            anticipoOutputModel.setTipoAnticipo(tipoAnticipo);
            anticipoOutputModel.setMontoTotalConIgv(montoTotalConIgv);
            anticipoOutputModel.setMontoTotalSinIgv(montoTotalSinIgv);
            return anticipoOutputModel;
        }
    }
}
