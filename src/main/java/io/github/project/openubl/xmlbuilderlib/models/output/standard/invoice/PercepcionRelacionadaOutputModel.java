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

import io.github.project.openubl.xmlbuilderlib.models.catalogs.Catalog53;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.DocumentoTributarioRelacionadoOutputModel;

import javax.validation.constraints.NotNull;
import java.math.BigDecimal;

public class PercepcionRelacionadaOutputModel extends DocumentoTributarioRelacionadoOutputModel {

    @NotNull
    private Catalog53 tipo;

    @NotNull
    private BigDecimal monto;

    @NotNull
    private BigDecimal porcentaje;

    public Catalog53 getTipo() {
        return tipo;
    }

    public void setTipo(Catalog53 tipo) {
        this.tipo = tipo;
    }

    public BigDecimal getMonto() {
        return monto;
    }

    public void setMonto(BigDecimal monto) {
        this.monto = monto;
    }

    public BigDecimal getPorcentaje() {
        return porcentaje;
    }

    public void setPorcentaje(BigDecimal porcentaje) {
        this.porcentaje = porcentaje;
    }
}
