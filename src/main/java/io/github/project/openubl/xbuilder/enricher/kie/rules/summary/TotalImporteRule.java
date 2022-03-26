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
package io.github.project.openubl.xbuilder.enricher.kie.rules.summary;

import io.github.project.openubl.xbuilder.content.catalogs.Catalog;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog5;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog7;
import io.github.project.openubl.xbuilder.content.models.standard.general.BaseDocumento;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoDetalle;
import io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporte;
import io.github.project.openubl.xbuilder.enricher.kie.AbstractRule;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;

import java.math.BigDecimal;
import java.util.function.Consumer;

import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.isBaseDocumento;
import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.whenBaseDocumento;

@RulePhase(type = RulePhase.PhaseType.SUMMARY)
public class TotalImporteRule extends AbstractRule {

    @Override
    public boolean test(Object object) {
        return isBaseDocumento.test(object) && whenBaseDocumento.apply(object)
                .map(documento -> documento.getTotalImporte() == null
                        && documento.getDetalles() != null
                )
                .orElse(false);
    }

    @Override
    public void modify(Object object) {
        Consumer<BaseDocumento> consumer = document -> {
            BigDecimal importeSinImpuestos = document.getDetalles().stream()
                    .filter(detalle -> {
                        Catalog7 catalog7 = Catalog.valueOfCode(Catalog7.class, detalle.getIgvTipo()).orElseThrow(Catalog.invalidCatalogValue);
                        return !catalog7.getTaxCategory().equals(Catalog5.GRATUITO);
                    })
                    .map(DocumentoDetalle::getIgvBaseImponible)
                    .reduce(BigDecimal.ZERO, BigDecimal::add);

            BigDecimal totalImpuestos = document.getDetalles().stream()
                    .filter(detalle -> {
                        Catalog7 catalog7 = Catalog.valueOfCode(Catalog7.class, detalle.getIgvTipo()).orElseThrow(Catalog.invalidCatalogValue);
                        return !catalog7.getTaxCategory().equals(Catalog5.GRATUITO);
                    })
                    .map(DocumentoDetalle::getTotalImpuestos)
                    .reduce(BigDecimal.ZERO, BigDecimal::add);

            BigDecimal importe = importeSinImpuestos.add(totalImpuestos);

            document.setTotalImporte(TotalImporte.builder()
                    .importe(importe)
                    .importeSinImpuestos(importeSinImpuestos)
                    .build()
            );
        };
        whenBaseDocumento.apply(object).ifPresent(consumer);
    }

}
