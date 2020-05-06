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

@Schema(name = "VoidedDocumentLine")
public class VoidedDocumentLineInputModel {

    @NotBlank
    @Pattern(regexp = "^([F|B|P|R|T][A-Z]?[0-9]{0,3})[\\-]([0-9]{1,8})$")
    @Schema(example = "F001-1", description = "Serie y número del comprobante a dar de baja")
    private String serieNumero;

    @NotBlank
    @CatalogConstraint(value = Catalog1.class)
    @Schema(example = "FACTURA", description = "Catalog 01", enumeration = {
            "FACTURA", "01",
            "BOLETA", "03",
            "NOTA_CREDITO", "07",
            "NOTA_DEBITO", "08",
            "GUIA_REMISION_REMITENTE", "09",
            "GUIA_REMISION_TRANSPORTISTA", "31",
            "GUIA_REMISION_REMITENTE_COMPLEMENTARIA", "71",
            "GUIA_REMISION_TRANSPORTISTA_COMPLEMENTARIA", "72",
            "RETENCION", "20",
            "PERCEPCION", "40",
    })
    private String tipoComprobante;

    @NotNull
    @Schema(example = "1585398109198", description = "Fecha en la que se emitió el comprobante a dar de baja. Fecha expresada en milliseconds")
    private Long fechaEmision;

    public String getSerieNumero() {
        return serieNumero;
    }

    public void setSerieNumero(String serieNumero) {
        this.serieNumero = serieNumero;
    }

    public String getTipoComprobante() {
        return tipoComprobante;
    }

    public void setTipoComprobante(String tipoComprobante) {
        this.tipoComprobante = tipoComprobante;
    }

    public Long getFechaEmision() {
        return fechaEmision;
    }

    public void setFechaEmision(Long fechaEmision) {
        this.fechaEmision = fechaEmision;
    }

    public static final class Builder {
        private String serieNumero;
        private String tipoComprobante;
        private Long fechaEmision;

        private Builder() {
        }

        public static Builder aVoidedDocumentLineInputModel() {
            return new Builder();
        }

        public Builder withSerieNumero(String serieNumero) {
            this.serieNumero = serieNumero;
            return this;
        }

        public Builder withTipoComprobante(String tipoComprobante) {
            this.tipoComprobante = tipoComprobante;
            return this;
        }

        public Builder withFechaEmision(Long fechaEmision) {
            this.fechaEmision = fechaEmision;
            return this;
        }

        public VoidedDocumentLineInputModel build() {
            VoidedDocumentLineInputModel voidedDocumentLineInputModel = new VoidedDocumentLineInputModel();
            voidedDocumentLineInputModel.setSerieNumero(serieNumero);
            voidedDocumentLineInputModel.setTipoComprobante(tipoComprobante);
            voidedDocumentLineInputModel.setFechaEmision(fechaEmision);
            return voidedDocumentLineInputModel;
        }
    }
}
