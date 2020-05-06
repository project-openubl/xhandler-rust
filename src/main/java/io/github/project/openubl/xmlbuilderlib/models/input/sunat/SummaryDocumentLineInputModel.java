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

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog19;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.constraints.CatalogConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.SummaryDocumentLineInputModel_ComprobanteAfectadoRequeridoConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.SummaryDocumentLineInputModel_ComprobanteAfectadoRequeridoGroupValidation;
import org.eclipse.microprofile.openapi.annotations.media.Schema;

import javax.validation.Valid;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;

@SummaryDocumentLineInputModel_ComprobanteAfectadoRequeridoConstraint(groups = SummaryDocumentLineInputModel_ComprobanteAfectadoRequeridoGroupValidation.class)
@Schema(name = "SummaryDocumentLine")
public class SummaryDocumentLineInputModel {

    @NotNull
    @NotBlank
    @CatalogConstraint(value = Catalog19.class)
    @Schema(example = "AGREGAR", description = "Catalogo 19", enumeration = {
            "ADICIONAR", "1",
            "MODIFICAR", "2",
            "ANULADO", "3"
    })
    private String tipoOperacion;

    @NotNull
    @Valid
    @Schema(description = "Comprobante a incluir dentro del Resumen Diario")
    private SummaryDocumentComprobanteInputModel comprobante;

    @Valid
    @Schema(description = "Si 'comprobante' es una Nota de Credito/Débito acá se incluye en comprobante asociado a la Nota Crédito/Débito")
    private SummaryDocumentComprobanteAfectadoInputModel comprobanteAfectado;

    public String getTipoOperacion() {
        return tipoOperacion;
    }

    public void setTipoOperacion(String tipoOperacion) {
        this.tipoOperacion = tipoOperacion;
    }

    public SummaryDocumentComprobanteInputModel getComprobante() {
        return comprobante;
    }

    public void setComprobante(SummaryDocumentComprobanteInputModel comprobante) {
        this.comprobante = comprobante;
    }

    public SummaryDocumentComprobanteAfectadoInputModel getComprobanteAfectado() {
        return comprobanteAfectado;
    }

    public void setComprobanteAfectado(SummaryDocumentComprobanteAfectadoInputModel comprobanteAfectado) {
        this.comprobanteAfectado = comprobanteAfectado;
    }

    public static final class Builder {
        private String tipoOperacion;
        private SummaryDocumentComprobanteInputModel comprobante;
        private SummaryDocumentComprobanteAfectadoInputModel comprobanteAfectado;

        private Builder() {
        }

        public static Builder aSummaryDocumentLineInputModel() {
            return new Builder();
        }

        public Builder withTipoOperacion(String tipoOperacion) {
            this.tipoOperacion = tipoOperacion;
            return this;
        }

        public Builder withComprobante(SummaryDocumentComprobanteInputModel comprobante) {
            this.comprobante = comprobante;
            return this;
        }

        public Builder withComprobanteAfectado(SummaryDocumentComprobanteAfectadoInputModel comprobanteAfectado) {
            this.comprobanteAfectado = comprobanteAfectado;
            return this;
        }

        public SummaryDocumentLineInputModel build() {
            SummaryDocumentLineInputModel summaryDocumentLineInputModel = new SummaryDocumentLineInputModel();
            summaryDocumentLineInputModel.setTipoOperacion(tipoOperacion);
            summaryDocumentLineInputModel.setComprobante(comprobante);
            summaryDocumentLineInputModel.setComprobanteAfectado(comprobanteAfectado);
            return summaryDocumentLineInputModel;
        }
    }
}
