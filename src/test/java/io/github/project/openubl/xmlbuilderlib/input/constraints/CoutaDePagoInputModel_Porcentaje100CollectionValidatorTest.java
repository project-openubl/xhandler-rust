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
package io.github.project.openubl.xmlbuilderlib.input.constraints;

import io.github.project.openubl.xmlbuilderlib.models.input.common.CuotaDePagoInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.CoutaDePagoInputModel_Porcentaje100CollectionValidator;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.CuotaDePagoInputModel_Porcentaje100CollectionConstraint;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.HighLevelGroupValidation;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import javax.validation.ConstraintViolation;
import javax.validation.Validation;
import javax.validation.Validator;
import javax.validation.ValidatorFactory;
import java.math.BigDecimal;
import java.util.Arrays;
import java.util.List;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertTrue;

public class CoutaDePagoInputModel_Porcentaje100CollectionValidatorTest {

    private static Validator validator;

    static class Bean {
        @CuotaDePagoInputModel_Porcentaje100CollectionConstraint(groups = HighLevelGroupValidation.class)
        private List<CuotaDePagoInputModel> cuotasDePago;
    }

    @BeforeAll
    public static void setUpValidator() {
        ValidatorFactory factory = Validation.buildDefaultValidatorFactory();
        validator = factory.getValidator();
    }

    @Test
    void sumPercentaje_is100() {
        List<CuotaDePagoInputModel> cuotas = Arrays.asList(
                CuotaDePagoInputModel.Builder.aFormaPagoCuotaInputModel()
                        .withPorcentaje(new BigDecimal(40))
                        .build(),
                CuotaDePagoInputModel.Builder.aFormaPagoCuotaInputModel()
                        .withPorcentaje(new BigDecimal(60))
                        .build()
        );

        Bean bean = new Bean();
        bean.cuotasDePago = cuotas;

        Set<ConstraintViolation<Bean>> violations = validator.validate(bean, HighLevelGroupValidation.class);
        assertTrue(violations.isEmpty());
    }

    @Test
    void sumPercentaje_is60() {
        List<CuotaDePagoInputModel> cuotas = Arrays.asList(
                CuotaDePagoInputModel.Builder.aFormaPagoCuotaInputModel()
                        .withPorcentaje(new BigDecimal(40))
                        .build(),
                CuotaDePagoInputModel.Builder.aFormaPagoCuotaInputModel()
                        .withPorcentaje(new BigDecimal(20))
                        .build()
        );

        Bean bean = new Bean();
        bean.cuotasDePago = cuotas;

        Set<ConstraintViolation<Bean>> violations = validator.validate(bean, HighLevelGroupValidation.class);
        assertTrue(
                violations.stream().anyMatch(p -> p.getMessage().equals(CoutaDePagoInputModel_Porcentaje100CollectionValidator.message))
        );
    }

    @Test
    void sumPercentaje_isGreaterThan100() {
        List<CuotaDePagoInputModel> cuotas = Arrays.asList(
                CuotaDePagoInputModel.Builder.aFormaPagoCuotaInputModel()
                        .withPorcentaje(new BigDecimal(40))
                        .build(),
                CuotaDePagoInputModel.Builder.aFormaPagoCuotaInputModel()
                        .withPorcentaje(new BigDecimal(100))
                        .build()
        );

        Bean bean = new Bean();
        bean.cuotasDePago = cuotas;

        Set<ConstraintViolation<Bean>> violations = validator.validate(bean, HighLevelGroupValidation.class);
        assertTrue(
                violations.stream().anyMatch(p -> p.getMessage().equals(CoutaDePagoInputModel_Porcentaje100CollectionValidator.message))
        );
    }

    @Test
    void noMontoNeitherPorcentaje_isNegative() {
        List<CuotaDePagoInputModel> cuotas = Arrays.asList(
                CuotaDePagoInputModel.Builder.aFormaPagoCuotaInputModel()
                        .withPorcentaje(new BigDecimal(-40))
                        .build(),
                CuotaDePagoInputModel.Builder.aFormaPagoCuotaInputModel()
                        .withPorcentaje(new BigDecimal(-100))
                        .build()
        );

        Bean bean = new Bean();
        bean.cuotasDePago = cuotas;

        Set<ConstraintViolation<Bean>> violations = validator.validate(bean, HighLevelGroupValidation.class);
        assertTrue(
                violations.stream().anyMatch(p -> p.getMessage().equals(CoutaDePagoInputModel_Porcentaje100CollectionValidator.message))
        );
    }

}
