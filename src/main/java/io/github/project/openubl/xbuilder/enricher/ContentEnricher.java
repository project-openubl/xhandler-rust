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
package io.github.project.openubl.xbuilder.enricher;

import io.github.project.openubl.xbuilder.content.models.standard.general.BaseDocumento;
import io.github.project.openubl.xbuilder.enricher.config.DateProvider;
import io.github.project.openubl.xbuilder.enricher.config.Defaults;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;
import io.github.project.openubl.xbuilder.enricher.kie.RuleUnit;
import io.github.project.openubl.xbuilder.enricher.kie.ruleunits.DefaultRuleUnit;

import java.util.Arrays;
import java.util.List;

public class ContentEnricher {

    private final Defaults defaults;
    private final DateProvider dateProvider;

    public ContentEnricher(Defaults defaults, DateProvider dateProvider) {
        this.defaults = defaults;
        this.dateProvider = dateProvider;
    }

    private List<RuleUnit> getRuleUnits() {
        RuleUnit enrichRuleUnit = new DefaultRuleUnit(RulePhase.PhaseType.ENRICH, defaults, dateProvider.now());
        RuleUnit processRuleUnit = new DefaultRuleUnit(RulePhase.PhaseType.PROCESS, defaults, dateProvider.now());
        RuleUnit summaryRuleUnit = new DefaultRuleUnit(RulePhase.PhaseType.SUMMARY, defaults, dateProvider.now());

        return Arrays.asList(enrichRuleUnit, processRuleUnit, summaryRuleUnit);
    }

    /**
     * Enrich Invoice, CreditNote, DebitNote
     */
    public void enrich(BaseDocumento input) {
        List<RuleUnit> ruleUnits = getRuleUnits();

        ruleUnits.forEach(ruleUnit -> {
            ruleUnit.modify(input);
            input.getDetalles().forEach(ruleUnit::modify);
        });
    }

}
