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
package io.github.project.openubl.xbuilder.enricher.kie.rules;

import io.github.project.openubl.xbuilder.content.catalogs.CatalogContadoCredito;
import io.github.project.openubl.xbuilder.content.models.standard.general.BaseDocumento;
import io.github.project.openubl.xbuilder.enricher.kie.AbstractRule;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;

import java.util.function.Consumer;

import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.isBaseDocumento;
import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.whenBaseDocumento;

@RulePhase(type = RulePhase.PhaseType.ENRICH)
public class FormaDePagoTipoRule extends AbstractRule {

    @Override
    public boolean test(Object object) {
        return isBaseDocumento.test(object);
    }

    @Override
    public void modify(Object object) {
        Consumer<BaseDocumento> consumer = document -> {
            CatalogContadoCredito formaDePagoTipo = document.getFormaDePagoCuotas() == null || document.getFormaDePagoCuotas().isEmpty() ?
                    CatalogContadoCredito.CONTADO :
                    CatalogContadoCredito.CREDITO;
            document.setFormaDePago(formaDePagoTipo.getCode());
        };
        whenBaseDocumento.apply(object).ifPresent(consumer);
    }
}
