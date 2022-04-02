package io.github.project.openubl.xbuilder.enricher.kie.ruleunits;

import io.github.project.openubl.xbuilder.enricher.kie.RuleContext;
import lombok.Builder;
import lombok.Getter;

import java.math.BigDecimal;

@Getter
@Builder
public class BodyRuleContext implements RuleContext {
    private BigDecimal tasaIgv;
    private BigDecimal tasaIcb;
}
