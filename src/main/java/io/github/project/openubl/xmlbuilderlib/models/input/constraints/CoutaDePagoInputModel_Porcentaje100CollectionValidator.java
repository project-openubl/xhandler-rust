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
package io.github.project.openubl.xmlbuilderlib.models.input.constraints;

import io.github.project.openubl.xmlbuilderlib.models.input.common.CuotaDePagoInputModel;

import javax.validation.ConstraintValidator;
import javax.validation.ConstraintValidatorContext;
import java.math.BigDecimal;
import java.util.Collection;

public class CoutaDePagoInputModel_Porcentaje100CollectionValidator implements ConstraintValidator<CuotaDePagoInputModel_Porcentaje100CollectionConstraint, Collection<CuotaDePagoInputModel>> {

    public static final String message = "Sumatoria de porcentajes debe de ser 100";

    @Override
    public void initialize(CuotaDePagoInputModel_Porcentaje100CollectionConstraint constraintAnnotation) {
    }

    @Override
    public boolean isValid(Collection<CuotaDePagoInputModel> value, ConstraintValidatorContext context) {
        BigDecimal total = value.stream()
                .map(f -> f.getPorcentaje() != null ? f.getPorcentaje() : BigDecimal.ZERO)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
        boolean isValid = total.compareTo(new BigDecimal(100)) == 0 || total.compareTo(BigDecimal.ZERO) == 0;

        if (!isValid) {
            context.disableDefaultConstraintViolation();
            context.buildConstraintViolationWithTemplate(message)
                    .addConstraintViolation();
        }

        return isValid;
    }

}
