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
package io.github.project.openubl.xbuilder.enricher.kie.rules.enrich.header.note.debitnote;

import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.isDebitNote;
import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.whenDebitNote;

import io.github.project.openubl.xbuilder.content.catalogs.Catalog10;
import io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote;
import io.github.project.openubl.xbuilder.enricher.kie.AbstractHeaderRule;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;
import java.util.function.Consumer;

@RulePhase(type = RulePhase.PhaseType.ENRICH)
public class TipoNotaDebitoRule extends AbstractHeaderRule {

    @Override
    public boolean test(Object object) {
        return (
            isDebitNote.test(object) &&
            whenDebitNote.apply(object).map(note -> note.getTipoNota() == null).orElse(false)
        );
    }

    @Override
    public void modify(Object object) {
        Consumer<DebitNote> consumer = note -> note.setTipoNota(Catalog10.AUMENTO_EN_EL_VALOR.getCode());
        whenDebitNote.apply(object).ifPresent(consumer);
    }
}
