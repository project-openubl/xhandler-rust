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
import io.github.project.openubl.xbuilder.content.models.standard.general.TotalImpuestos;
import io.github.project.openubl.xbuilder.enricher.kie.AbstractRule;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;

import java.math.BigDecimal;
import java.util.List;
import java.util.Objects;
import java.util.function.Consumer;
import java.util.function.Supplier;
import java.util.stream.Stream;

import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.isBaseDocumento;
import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.whenBaseDocumento;

@RulePhase(type = RulePhase.PhaseType.SUMMARY)
public class TotalImpuestosRule extends AbstractRule {

    @Override
    public boolean test(Object object) {
        return isBaseDocumento.test(object) && whenBaseDocumento.apply(object)
                .map(documento -> documento.getTotalImpuestos() == null
                        && documento.getDetalles() != null
                )
                .orElse(false);
    }

    @Override
    public void modify(Object object) {
        Consumer<BaseDocumento> consumer = document -> {
            Impuesto ivap = calImpuestoByTipo(document.getDetalles(), Catalog5.IMPUESTO_ARROZ_PILADO);
            Impuesto gravado = calImpuestoByTipo(document.getDetalles(), Catalog5.IGV);
            Impuesto inafecto = calImpuestoByTipo(document.getDetalles(), Catalog5.INAFECTO);
            Impuesto exonerado = calImpuestoByTipo(document.getDetalles(), Catalog5.EXONERADO);
            Impuesto gratuito = calImpuestoByTipo(document.getDetalles(), Catalog5.GRATUITO);

            BigDecimal icb = document.getDetalles().stream()
                    .map(DocumentoDetalle::getIcb)
                    .filter(Objects::nonNull)
                    .reduce(BigDecimal.ZERO, BigDecimal::add);

            BigDecimal total = ivap.getImporte().add(gravado.getImporte()).add(icb);

            TotalImpuestos totalImpuestos = TotalImpuestos.builder()
                    .ivapImporte(ivap.getImporte())
                    .ivapBaseImponible(ivap.getBaseImponible())
                    .gravadoImporte(gravado.getImporte())
                    .gravadoBaseImponible(gravado.getBaseImponible())
                    .inafectoImporte(inafecto.getImporte())
                    .inafectoBaseImponible(inafecto.getBaseImponible())
                    .exoneradoImporte(exonerado.getImporte())
                    .exoneradoBaseImponible(exonerado.getBaseImponible())
                    .gratuitoImporte(gratuito.getImporte())
                    .gratuitoBaseImponible(gratuito.getBaseImponible())
                    .icbImporte(icb)
                    .total(total)
                    .build();

            document.setTotalImpuestos(totalImpuestos);
        };
        whenBaseDocumento.apply(object).ifPresent(consumer);
    }

    private Impuesto calImpuestoByTipo(List<DocumentoDetalle> detalle, Catalog5 categoria) {
        Supplier<Stream<DocumentoDetalle>> stream = () -> detalle.stream().filter($il -> {
            Catalog7 catalog7 = Catalog.valueOfCode(Catalog7.class, $il.getIgvTipo()).orElseThrow(Catalog.invalidCatalogValue);
            return catalog7.getTaxCategory().equals(categoria);
        });

        BigDecimal baseImponible = stream.get().map(DocumentoDetalle::getIgvBaseImponible).reduce(BigDecimal.ZERO, BigDecimal::add);
        BigDecimal importe = stream.get().map(DocumentoDetalle::getIgv).reduce(BigDecimal.ZERO, BigDecimal::add);

        return Impuesto.builder()
                .importe(importe)
                .baseImponible(baseImponible)
                .build();
    }
}
