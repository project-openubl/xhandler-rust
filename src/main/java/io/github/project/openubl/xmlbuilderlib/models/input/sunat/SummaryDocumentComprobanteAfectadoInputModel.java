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

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog1;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.constraints.CatalogConstraint;
import org.eclipse.microprofile.openapi.annotations.media.Schema;

import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Pattern;

public class SummaryDocumentComprobanteAfectadoInputModel {

    @NotNull
    @CatalogConstraint(value = Catalog1.class)
    @Schema(example = "BOLETA", description = "Catalog 01", enumeration = {
            "FACTURA", "01",
            "BOLETA", "03",
            "NOTA_CREDITO", "07",
            "NOTA_DEBITO", "08"
    })
    private String tipo;

    @NotNull
    @NotBlank
    @Pattern(regexp = "^([A-Z]{1,3}[0-9]{1,3})[\\-]([0-9]{1,8})$")
    @Schema(example = "B001-1", description = "Serie y número del coprobante afectado por Nota de Crédito/Débito")
    private String serieNumero;

    public String getTipo() {
        return tipo;
    }

    public void setTipo(String tipo) {
        this.tipo = tipo;
    }

    public String getSerieNumero() {
        return serieNumero;
    }

    public void setSerieNumero(String serieNumero) {
        this.serieNumero = serieNumero;
    }

    public static final class Builder {
        private String tipo;
        private String serieNumero;

        private Builder() {
        }

        public static Builder aSummaryDocumentComprobanteAfectadoInputModel() {
            return new Builder();
        }

        public Builder withTipo(String tipo) {
            this.tipo = tipo;
            return this;
        }

        public Builder withSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
            return this;
        }

        public SummaryDocumentComprobanteAfectadoInputModel build() {
            SummaryDocumentComprobanteAfectadoInputModel summaryDocumentComprobanteAfectadoInputModel = new SummaryDocumentComprobanteAfectadoInputModel();
            summaryDocumentComprobanteAfectadoInputModel.setTipo(tipo);
            summaryDocumentComprobanteAfectadoInputModel.setSerieNumero(serieNumero);
            return summaryDocumentComprobanteAfectadoInputModel;
        }
    }
}
