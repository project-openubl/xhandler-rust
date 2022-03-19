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

import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.enricher.config.DateProvider;
import io.github.project.openubl.xbuilder.enricher.config.Defaults;
import io.github.project.openubl.xbuilder.enricher.kie.RuleUnit;
import io.github.project.openubl.xbuilder.enricher.kie.ruleunits.EnrichRuleUnit;
import io.github.project.openubl.xbuilder.enricher.kie.ruleunits.ProcessRuleUnit;

import java.util.Arrays;

public class ContentEnricher {

    private final Defaults defaults;
    private final DateProvider dateProvider;

    public ContentEnricher(Defaults defaults, DateProvider dateProvider) {
        this.defaults = defaults;
        this.dateProvider = dateProvider;
    }

    public void enrich(Invoice input) {
        RuleUnit enrichRuleUnit = new EnrichRuleUnit(defaults, dateProvider.now());
        RuleUnit processRuleUnit = new ProcessRuleUnit(defaults, dateProvider.now());

        Arrays.asList(enrichRuleUnit, processRuleUnit).forEach(ruleUnit -> {
            ruleUnit.modify(input);
            input.getDetalles().forEach(ruleUnit::modify);
        });
    }
}
