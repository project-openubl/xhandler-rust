package io.github.project.openubl.xbuilder.enricher.kie.rules.enrich.anticipo;

import io.github.project.openubl.xbuilder.content.catalogs.Catalog;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog12;
import io.github.project.openubl.xbuilder.content.models.standard.general.Anticipo;
import io.github.project.openubl.xbuilder.content.models.utils.UBLRegex;
import io.github.project.openubl.xbuilder.enricher.kie.AbstractRule;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;

import java.util.function.Consumer;

import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.*;

@RulePhase(type = RulePhase.PhaseType.ENRICH)
public class ComprobanteTipoRule extends AbstractRule {

    @Override
    public boolean test(Object object) {
        return isAnticipo.test(object);
    }

    @Override
    public void modify(Object object) {
        Consumer<Anticipo> consumer = anticipo -> {
            String comprobanteTipo = null;
            if (anticipo.getComprobanteTipo() == null) {
                if (UBLRegex.FACTURA_SERIE_REGEX.matcher(anticipo.getComprobanteSerieNumero()).matches()) {
                    comprobanteTipo = Catalog12.FACTURA_EMITIDA_POR_ANTICIPOS.getCode();
                } else if (UBLRegex.BOLETA_SERIE_REGEX.matcher(anticipo.getComprobanteSerieNumero()).matches()) {
                    comprobanteTipo = Catalog12.BOLETA_DE_VENTA_EMITIDA_POR_ANTICIPOS.getCode();
                }
            } else {
                Catalog12 catalog12 = Catalog.valueOfCode(Catalog12.class, anticipo.getComprobanteTipo()).orElseThrow(Catalog.invalidCatalogValue);
                comprobanteTipo = catalog12.getCode();
            }

            anticipo.setComprobanteTipo(comprobanteTipo);
        };
        whenAnticipo.apply(object).ifPresent(consumer);
    }
}
