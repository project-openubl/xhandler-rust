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
package io.github.project.openubl.xbuilder.enricher.kie.rules.process.detalle;

import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoDetalle;
import io.github.project.openubl.xbuilder.enricher.kie.AbstractRule;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;

import java.math.BigDecimal;
import java.util.function.Consumer;

import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.isBaseDocumentoDetalle;
import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.whenBaseDocumentoDetalle;

@RulePhase(type = RulePhase.PhaseType.PROCESS)
public class TotalImpuestosRule extends AbstractRule {

    @Override
    public boolean test(Object object) {
        return isBaseDocumentoDetalle.test(object) && whenBaseDocumentoDetalle.apply(object)
                .map(documento -> documento.getTotalImpuestos() == null
                        && documento.getIgv() != null
                        && documento.getIcb() != null
                )
                .orElse(false);
    }

    @Override
    public void modify(Object object) {
        Consumer<DocumentoDetalle> consumer = detalle -> {
            BigDecimal totalImpuestos = detalle.getIgv().add(detalle.getIcb());
            detalle.setTotalImpuestos(totalImpuestos);
        };
        whenBaseDocumentoDetalle.apply(object).ifPresent(consumer);
    }

}
