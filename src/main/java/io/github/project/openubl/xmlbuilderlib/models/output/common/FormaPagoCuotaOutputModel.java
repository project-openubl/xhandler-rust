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

import javax.validation.constraints.NotBlank;
import javax.validation.constraints.NotNull;
import javax.validation.constraints.Pattern;
import java.math.BigDecimal;

public class FormaPagoCuotaOutputModel {

    @NotBlank
    @Pattern(regexp = "[0-9]{3}")
    private String id;

    @NotNull
    private BigDecimal monto;

    @NotNull
    private String fechaPago;

    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public BigDecimal getMonto() {
        return monto;
    }

    public void setMonto(BigDecimal monto) {
        this.monto = monto;
    }

    public String getFechaPago() {
        return fechaPago;
    }

    public void setFechaPago(String fechaPago) {
        this.fechaPago = fechaPago;
    }

    public static final class Builder {
        private String id;
        private BigDecimal monto;
        private String fechaPago;

        private Builder() {
        }

        public static Builder aFormaPagoCuotaOutputModel() {
            return new Builder();
        }

        public Builder withId(String id) {
            this.id = id;
            return this;
        }

        public Builder withMonto(BigDecimal monto) {
            this.monto = monto;
            return this;
        }

        public Builder withFechaPago(String fechaPago) {
            this.fechaPago = fechaPago;
            return this;
        }

        public FormaPagoCuotaOutputModel build() {
            FormaPagoCuotaOutputModel formaPagoCuotaOutputModel = new FormaPagoCuotaOutputModel();
            formaPagoCuotaOutputModel.setId(id);
            formaPagoCuotaOutputModel.setMonto(monto);
            formaPagoCuotaOutputModel.setFechaPago(fechaPago);
            return formaPagoCuotaOutputModel;
        }
    }
}
