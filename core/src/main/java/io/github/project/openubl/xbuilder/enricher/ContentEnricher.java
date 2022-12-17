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

import io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.content.models.standard.general.Note;
import io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocuments;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Perception;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Retention;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocuments;
import io.github.project.openubl.xbuilder.enricher.config.DateProvider;
import io.github.project.openubl.xbuilder.enricher.config.Defaults;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;
import io.github.project.openubl.xbuilder.enricher.kie.RuleUnit;
import io.github.project.openubl.xbuilder.enricher.kie.ruleunits.BodyRuleContext;
import io.github.project.openubl.xbuilder.enricher.kie.ruleunits.BodyRuleUnit;
import io.github.project.openubl.xbuilder.enricher.kie.ruleunits.HeaderRuleContext;
import io.github.project.openubl.xbuilder.enricher.kie.ruleunits.HeaderRuleUnit;

import java.time.LocalDate;
import java.util.stream.Stream;

public class ContentEnricher {

    private final Defaults defaults;
    private final DateProvider dateProvider;

    public ContentEnricher(Defaults defaults, DateProvider dateProvider) {
        this.defaults = defaults;
        this.dateProvider = dateProvider;
    }

    public void enrich(Invoice input) {
        LocalDate systemLocalDate = dateProvider.now();

        Stream
                .of(RulePhase.PhaseType.ENRICH, RulePhase.PhaseType.PROCESS, RulePhase.PhaseType.SUMMARY)
                .forEach(phaseType -> {
                    // Header
                    HeaderRuleContext ruleContextHeader = HeaderRuleContext.builder().localDate(systemLocalDate).build();
                    RuleUnit ruleUnitHeader = new HeaderRuleUnit(phaseType, defaults, ruleContextHeader);
                    ruleUnitHeader.modify(input);

                    // Body
                    BodyRuleContext ruleContextBody = BodyRuleContext
                            .builder()
                            .tasaIgv(input.getTasaIgv())
                            .tasaIcb(input.getTasaIcb())
                            .build();
                    RuleUnit ruleUnitBody = new BodyRuleUnit(phaseType, defaults, ruleContextBody);
                    input.getDetalles().forEach(ruleUnitBody::modify);
                    input.getAnticipos().forEach(ruleUnitBody::modify);
                });
    }

    public void enrich(CreditNote input) {
        enrichNote(input);
    }

    public void enrich(DebitNote input) {
        enrichNote(input);
    }

    public void enrich(VoidedDocuments input) {
        LocalDate systemLocalDate = dateProvider.now();

        Stream
                .of(RulePhase.PhaseType.ENRICH, RulePhase.PhaseType.PROCESS, RulePhase.PhaseType.SUMMARY)
                .forEach(phaseType -> {
                    // Header
                    HeaderRuleContext ruleContextHeader = HeaderRuleContext.builder()
                            .localDate(systemLocalDate)
                            .build();
                    RuleUnit ruleUnitHeader = new HeaderRuleUnit(phaseType, defaults, ruleContextHeader);
                    ruleUnitHeader.modify(input);

                    // Body
                    BodyRuleContext ruleContextBody = BodyRuleContext.builder().build();

                    RuleUnit ruleUnitBody = new BodyRuleUnit(phaseType, defaults, ruleContextBody);
                    input.getComprobantes().forEach(ruleUnitBody::modify);
                });
    }

    public void enrich(SummaryDocuments input) {
        LocalDate systemLocalDate = dateProvider.now();

        Stream
                .of(RulePhase.PhaseType.ENRICH, RulePhase.PhaseType.PROCESS, RulePhase.PhaseType.SUMMARY)
                .forEach(phaseType -> {
                    // Header
                    HeaderRuleContext ruleContextHeader = HeaderRuleContext.builder()
                            .localDate(systemLocalDate)
                            .build();
                    RuleUnit ruleUnitHeader = new HeaderRuleUnit(phaseType, defaults, ruleContextHeader);
                    ruleUnitHeader.modify(input);

                    // Body
                    BodyRuleContext ruleContextBody = BodyRuleContext.builder().build();

                    RuleUnit ruleUnitBody = new BodyRuleUnit(phaseType, defaults, ruleContextBody);
                    input.getComprobantes().forEach(ruleUnitBody::modify);
                });
    }

    public void enrich(Perception input) {
        LocalDate systemLocalDate = dateProvider.now();

        Stream
                .of(RulePhase.PhaseType.ENRICH, RulePhase.PhaseType.PROCESS, RulePhase.PhaseType.SUMMARY)
                .forEach(phaseType -> {
                    // Header
                    HeaderRuleContext ruleContextHeader = HeaderRuleContext.builder()
                            .localDate(systemLocalDate)
                            .build();
                    RuleUnit ruleUnitHeader = new HeaderRuleUnit(phaseType, defaults, ruleContextHeader);
                    ruleUnitHeader.modify(input);

                    // Body
                });
    }

    public void enrich(Retention input) {
        LocalDate systemLocalDate = dateProvider.now();

        Stream
                .of(RulePhase.PhaseType.ENRICH, RulePhase.PhaseType.PROCESS, RulePhase.PhaseType.SUMMARY)
                .forEach(phaseType -> {
                    // Header
                    HeaderRuleContext ruleContextHeader = HeaderRuleContext.builder()
                            .localDate(systemLocalDate)
                            .build();
                    RuleUnit ruleUnitHeader = new HeaderRuleUnit(phaseType, defaults, ruleContextHeader);
                    ruleUnitHeader.modify(input);

                    // Body
                });
    }

    private void enrichNote(Note input) {
        LocalDate systemLocalDate = dateProvider.now();

        Stream
                .of(RulePhase.PhaseType.ENRICH, RulePhase.PhaseType.PROCESS, RulePhase.PhaseType.SUMMARY)
                .forEach(phaseType -> {
                    // Header
                    HeaderRuleContext ruleContextHeader = HeaderRuleContext.builder().localDate(systemLocalDate).build();
                    RuleUnit ruleUnitHeader = new HeaderRuleUnit(phaseType, defaults, ruleContextHeader);
                    ruleUnitHeader.modify(input);

                    // Body
                    BodyRuleContext ruleContextBody = BodyRuleContext
                            .builder()
                            .tasaIgv(input.getTasaIgv())
                            .tasaIcb(input.getTasaIcb())
                            .build();
                    RuleUnit ruleUnitBody = new BodyRuleUnit(phaseType, defaults, ruleContextBody);
                    input.getDetalles().forEach(ruleUnitBody::modify);
                });
    }

}
