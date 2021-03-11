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
package io.github.project.openubl.xmlbuilderlib.models.output.common;

import javax.validation.Valid;
import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Pattern;
import javax.validation.constraints.Size;
import java.math.BigDecimal;
import java.util.List;

public class FormaPagoOutputModel {

    public enum Tipo {
        Contado,
        Credito
    }

    @NotNull
    private Tipo tipo;

    @NotNull
    private BigDecimal montoTotal;

    @Valid
    private List<FormaPagoCuotaOutputModel> cuotas;

    public Tipo getTipo() {
        return tipo;
    }

    public void setTipo(Tipo tipo) {
        this.tipo = tipo;
    }

    public BigDecimal getMontoTotal() {
        return montoTotal;
    }

    public void setMontoTotal(BigDecimal montoTotal) {
        this.montoTotal = montoTotal;
    }

    public List<FormaPagoCuotaOutputModel> getCuotas() {
        return cuotas;
    }

    public void setCuotas(List<FormaPagoCuotaOutputModel> cuotas) {
        this.cuotas = cuotas;
    }

    public static final class Builder {
        private Tipo tipo;
        private BigDecimal montoTotal;
        private List<FormaPagoCuotaOutputModel> cuotas;

        private Builder() {
        }

        public static Builder aFormaPagoOutputModel() {
            return new Builder();
        }

        public Builder withTipo(Tipo tipo) {
            this.tipo = tipo;
            return this;
        }

        public Builder withMontoTotal(BigDecimal montoTotal) {
            this.montoTotal = montoTotal;
            return this;
        }

        public Builder withCuotas(List<FormaPagoCuotaOutputModel> cuotas) {
            this.cuotas = cuotas;
            return this;
        }

        public FormaPagoOutputModel build() {
            FormaPagoOutputModel formaPagoOutputModel = new FormaPagoOutputModel();
            formaPagoOutputModel.setTipo(tipo);
            formaPagoOutputModel.setMontoTotal(montoTotal);
            formaPagoOutputModel.setCuotas(cuotas);
            return formaPagoOutputModel;
        }
    }
}
