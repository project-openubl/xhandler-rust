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
package io.github.project.openubl.xmlbuilderlib.models.input.common;

import io.github.project.openubl.xmlbuilderlib.models.input.constraints.CuotaDePagoInputModel_MontoPorcentajeConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.HighLevelGroupValidation;

import javax.validation.constraints.Max;
import javax.validation.constraints.Min;
import javax.validation.constraints.NotNull;
import java.math.BigDecimal;

@CuotaDePagoInputModel_MontoPorcentajeConstraint(groups = HighLevelGroupValidation.class)
public class CuotaDePagoInputModel {

    @Min(0)
    private BigDecimal monto;

    @Min(0)
    @Max(100)
    private BigDecimal porcentaje;

    @NotNull
    private Long fechaPago;

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

    public Long getFechaPago() {
        return fechaPago;
    }

    public void setFechaPago(Long fechaPago) {
        this.fechaPago = fechaPago;
    }

    public static final class Builder {
        private BigDecimal monto;
        private BigDecimal porcentaje;
        private Long fechaPago;

        private Builder() {
        }

        public static Builder aFormaPagoCuotaInputModel() {
            return new Builder();
        }

        public Builder withMonto(BigDecimal monto) {
            this.monto = monto;
            return this;
        }

        public Builder withPorcentaje(BigDecimal porcentaje) {
            this.porcentaje = porcentaje;
            return this;
        }

        public Builder withFechaPago(Long fechaPago) {
            this.fechaPago = fechaPago;
            return this;
        }

        public CuotaDePagoInputModel build() {
            CuotaDePagoInputModel cuotaDePagoInputModel = new CuotaDePagoInputModel();
            cuotaDePagoInputModel.setMonto(monto);
            cuotaDePagoInputModel.setPorcentaje(porcentaje);
            cuotaDePagoInputModel.setFechaPago(fechaPago);
            return cuotaDePagoInputModel;
        }
    }
}
