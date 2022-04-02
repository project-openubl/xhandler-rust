package io.github.project.openubl.xbuilder.enricher.kie.ruleunits;

import io.github.project.openubl.xbuilder.enricher.kie.RuleContext;
import java.math.BigDecimal;
import lombok.Builder;
import lombok.Getter;

@Getter
@Builder
public class BodyRuleContext implements RuleContext {

    private BigDecimal tasaIgv;
    private BigDecimal tasaIcb;
}
