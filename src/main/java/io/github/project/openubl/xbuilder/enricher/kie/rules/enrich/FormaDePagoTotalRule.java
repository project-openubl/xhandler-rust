/*
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 *
 * Licensed under the Apache License - 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package io.github.project.openubl.xbuilder.enricher.kie.rules.enrich;

import io.github.project.openubl.xbuilder.content.models.standard.general.BaseDocumento;
import io.github.project.openubl.xbuilder.content.models.standard.general.CuotaDePago;
import io.github.project.openubl.xbuilder.enricher.kie.AbstractRule;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;

import java.math.BigDecimal;
import java.util.function.Consumer;

import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.isBaseDocumento;
import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.whenBaseDocumento;

@RulePhase(type = RulePhase.PhaseType.ENRICH)
public class FormaDePagoTotalRule extends AbstractRule {

    @Override
    public boolean test(Object object) {
        return isBaseDocumento.test(object) && whenBaseDocumento.apply(object)
                .map(documento -> documento.getFormaDePagoCuotas() != null)
                .orElse(false);
    }

    @Override
    public void modify(Object object) {
        Consumer<BaseDocumento> consumer = document -> {
            BigDecimal formaDePagoTotal = document.getFormaDePagoCuotas().stream()
                    .map(CuotaDePago::getImporte)
                    .reduce(BigDecimal.ZERO, BigDecimal::add);

            document.setFormaDePagoTotal(formaDePagoTotal);
        };
        whenBaseDocumento.apply(object).ifPresent(consumer);
    }
}
