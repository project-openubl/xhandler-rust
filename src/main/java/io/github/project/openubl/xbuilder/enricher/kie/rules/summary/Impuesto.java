package io.github.project.openubl.xbuilder.enricher.kie.rules.summary;

import lombok.Builder;
import lombok.Data;

import java.math.BigDecimal;

@Data
@Builder
public class Impuesto {
    private BigDecimal importe;
    private BigDecimal baseImponible;
}
