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
package io.github.project.openubl.xbuilder.enricher.kie.rules.summary.header.invoice;

import io.github.project.openubl.xbuilder.content.models.standard.general.Anticipo;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteInvoice;
import io.github.project.openubl.xbuilder.enricher.kie.AbstractHeaderRule;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;
import io.github.project.openubl.xbuilder.enricher.kie.rules.utils.DetalleUtils;

import java.math.BigDecimal;
import java.util.Objects;
import java.util.function.Consumer;

import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.isInvoice;
import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.whenInvoice;

@RulePhase(type = RulePhase.PhaseType.SUMMARY)
public class TotalImporteRule extends AbstractHeaderRule {

    @Override
    public boolean test(Object object) {
        return (
                isInvoice.test(object) &&
                        whenInvoice
                                .apply(object)
                                .map(invoice -> invoice.getTotalImporte() == null && invoice.getDetalles() != null)
                                .orElse(false)
        );
    }

    @Override
    public void modify(Object object) {
        Consumer<Invoice> consumer = invoice -> {
            BigDecimal totalImpuestos = DetalleUtils.getTotalImpuestos(invoice.getDetalles());

            BigDecimal importeSinImpuestos = DetalleUtils.getImporteSinImpuestos(invoice.getDetalles());
            BigDecimal importeConImpuestos = importeSinImpuestos.add(totalImpuestos);

            BigDecimal anticipos = invoice
                    .getAnticipos()
                    .stream()
                    .map(Anticipo::getMonto)
                    .filter(Objects::nonNull)
                    .reduce(BigDecimal.ZERO, BigDecimal::add);

            BigDecimal importeTotal = importeConImpuestos.subtract(anticipos);

            invoice.setTotalImporte(
                    TotalImporteInvoice
                            .builder()
                            .importeSinImpuestos(importeSinImpuestos)
                            .importeConImpuestos(importeConImpuestos)
                            .anticipos(anticipos)
                            .importe(importeTotal)
                            .build()
            );
        };
        whenInvoice.apply(object).ifPresent(consumer);
    }
}
