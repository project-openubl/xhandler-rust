package io.github.project.openubl.xbuilder.enricher.kie.rules.enrich.anticipo;

import io.github.project.openubl.xbuilder.content.catalogs.Catalog;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog53_Anticipo;
import io.github.project.openubl.xbuilder.content.models.standard.general.Anticipo;
import io.github.project.openubl.xbuilder.enricher.kie.AbstractRule;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;

import java.util.function.Consumer;

import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.isAnticipo;
import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.whenAnticipo;

@RulePhase(type = RulePhase.PhaseType.ENRICH)
public class TipoAnticipoRule extends AbstractRule {

    @Override
    public boolean test(Object object) {
        return isAnticipo.test(object);
    }

    @Override
    public void modify(Object object) {
        Consumer<Anticipo> consumer = anticipo -> {
            String tipoAnticipo;
            if (anticipo.getComprobanteTipo() == null) {
                tipoAnticipo = Catalog53_Anticipo.DESCUENTO_GLOBAL_POR_ANTICIPOS_GRAVADOS_AFECTA_BASE_IMPONIBLE_IGV_IVAP.getCode();
            } else {
                Catalog53_Anticipo catalog53 = Catalog.valueOfCode(Catalog53_Anticipo.class, anticipo.getTipo()).orElseThrow(Catalog.invalidCatalogValue);
                tipoAnticipo = catalog53.getCode();
            }

            anticipo.setTipo(tipoAnticipo);
        };
        whenAnticipo.apply(object).ifPresent(consumer);
    }
}
