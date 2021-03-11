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
package io.github.project.openubl.xmlbuilderlib.factory.common;

import io.github.project.openubl.xmlbuilderlib.models.input.common.CuotaDePagoInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.FirmanteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.common.ProveedorInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.FirmanteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.FormaPagoCuotaOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.FormaPagoOutputModel;

import java.math.BigDecimal;
import java.math.RoundingMode;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.TimeZone;

import static io.github.project.openubl.xmlbuilderlib.utils.DateUtils.toGregorianCalendarDate;

public class FormaPagoOutputModelFactory {

    private FormaPagoOutputModelFactory() {
        // Only static methods
    }

    public static FirmanteOutputModel getFirmante(FirmanteInputModel input) {
        return FirmanteOutputModel.Builder.aFirmanteOutputModel()
                .withRuc(input.getRuc())
                .withRazonSocial(input.getRazonSocial())
                .build();
    }

    public static FirmanteOutputModel getFirmante(ProveedorInputModel input) {
        return FirmanteOutputModel.Builder.aFirmanteOutputModel()
                .withRuc(input.getRuc())
                .withRazonSocial(input.getRazonSocial())
                .build();
    }

    public static FormaPagoOutputModel getFormaPago(List<CuotaDePagoInputModel> cuotasDePago, BigDecimal montoTotal, TimeZone timeZone) {
        FormaPagoOutputModel.Builder builder = FormaPagoOutputModel.Builder.aFormaPagoOutputModel()
                .withMontoTotal(montoTotal);

        if (cuotasDePago == null || cuotasDePago.isEmpty()) {
            builder
                    .withTipo(FormaPagoOutputModel.Tipo.Contado)
                    .withCuotas(Collections.emptyList());
        } else {
            List<FormaPagoCuotaOutputModel> cuotasOutput = new ArrayList<>();

            builder
                    .withTipo(FormaPagoOutputModel.Tipo.Credito)
                    .withCuotas(cuotasOutput);

            BigDecimal subtotal = BigDecimal.ZERO;
            for (int i = 0; i < cuotasDePago.size(); i++) {
                CuotaDePagoInputModel item = cuotasDePago.get(i);

                BigDecimal monto;
                if (item.getMonto() != null) {
                    monto = item.getMonto();
                } else if (item.getPorcentaje() != null) {
                    if (i != cuotasDePago.size() - 1) {
                        monto = montoTotal.multiply(item.getPorcentaje()).divide(new BigDecimal(100), 2, RoundingMode.HALF_EVEN);
                    } else {
                        monto = montoTotal.subtract(subtotal);
                    }
                } else {
                    throw new IllegalStateException("Monto or porcentaje must be present");
                }

                subtotal = subtotal.add(monto);

                FormaPagoCuotaOutputModel output = FormaPagoCuotaOutputModel.Builder.aFormaPagoCuotaOutputModel()
                        .withId(String.format("%03d", i))
                        .withMonto(monto)
                        .withFechaPago(toGregorianCalendarDate(item.getFechaPago(), timeZone))
                        .build();

                cuotasOutput.add(output);
            }
        }

        return builder.build();
    }
}
