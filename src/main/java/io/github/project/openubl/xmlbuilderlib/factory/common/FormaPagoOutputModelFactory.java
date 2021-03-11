/**
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 * <p>
 * Licensed under the Eclipse Public License - v 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * <p>
 * https://www.eclipse.org/legal/epl-2.0/
 * <p>
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
import java.util.*;
import java.util.stream.Collectors;

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
                    .withTipo("Contado")
                    .withCuotas(Collections.emptyList());
        } else {
            List<FormaPagoCuotaOutputModel> cuotasOutput = new ArrayList<>();

            builder
                    .withTipo("Credito")
                    .withCuotas(cuotasOutput);

            for (int i = 0; i < cuotasDePago.size(); i++) {
                CuotaDePagoInputModel item = cuotasDePago.get(i);

                FormaPagoCuotaOutputModel output = FormaPagoCuotaOutputModel.Builder.aFormaPagoCuotaOutputModel()
                        .withId(String.format("%03d", i))
                        .withMonto(item.getMonto())
                        .withFechaPago(toGregorianCalendarDate(item.getFechaPago(), timeZone))
                        .build();

                cuotasOutput.add(output);
            }
        }

        return builder.build();
    }
}
