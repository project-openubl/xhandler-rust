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

import io.github.project.openubl.xbuilder.content.catalogs.Catalog;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog5;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog53_Anticipo;
import io.github.project.openubl.xbuilder.content.models.standard.general.Anticipo;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoVentaDetalle;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.content.models.standard.general.TotalImpuestos;
import io.github.project.openubl.xbuilder.enricher.kie.AbstractHeaderRule;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;
import io.github.project.openubl.xbuilder.enricher.kie.rules.utils.DetalleUtils;
import io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Impuesto;

import java.math.BigDecimal;
import java.util.Objects;
import java.util.Optional;
import java.util.function.Consumer;

import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.isInvoice;
import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.whenInvoice;

@RulePhase(type = RulePhase.PhaseType.SUMMARY)
public class TotalImpuestosRule extends AbstractHeaderRule {

    @Override
    public boolean test(Object object) {
        return (
                isInvoice.test(object) &&
                        whenInvoice
                                .apply(object)
                                .map(documento -> documento.getTotalImpuestos() == null && documento.getDetalles() != null)
                                .orElse(false)
        );
    }

    @Override
    public void modify(Object object) {
        Consumer<Invoice> consumer = invoice -> {
            Impuesto ivap = DetalleUtils.calImpuestoByTipo(invoice.getDetalles(), Catalog5.IMPUESTO_ARROZ_PILADO);
            Impuesto gravado = DetalleUtils.calImpuestoByTipo(invoice.getDetalles(), Catalog5.IGV);
            Impuesto inafecto = DetalleUtils.calImpuestoByTipo(invoice.getDetalles(), Catalog5.INAFECTO);
            Impuesto exonerado = DetalleUtils.calImpuestoByTipo(invoice.getDetalles(), Catalog5.EXONERADO);
            Impuesto gratuito = DetalleUtils.calImpuestoByTipo(invoice.getDetalles(), Catalog5.GRATUITO);

            BigDecimal icb = invoice
                    .getDetalles()
                    .stream()
                    .map(DocumentoVentaDetalle::getIcb)
                    .filter(Objects::nonNull)
                    .reduce(BigDecimal.ZERO, BigDecimal::add);

            BigDecimal totalAnticiposGravados = invoice
                    .getAnticipos()
                    .stream()
                    .filter(f -> {
                        Optional<Catalog53_Anticipo> catalog53_anticipo = Catalog.valueOfCode(
                                Catalog53_Anticipo.class,
                                f.getTipo()
                        );
                        return (
                                catalog53_anticipo.isPresent() &&
                                        catalog53_anticipo
                                                .get()
                                                .equals(
                                                        Catalog53_Anticipo.DESCUENTO_GLOBAL_POR_ANTICIPOS_GRAVADOS_AFECTA_BASE_IMPONIBLE_IGV_IVAP
                                                )
                        );
                    })
                    .map(Anticipo::getMonto)
                    .reduce(BigDecimal.ZERO, BigDecimal::add);
            BigDecimal gravadoBaseImponible = gravado.getBaseImponible().subtract(totalAnticiposGravados);
            BigDecimal gravadoImporte = gravadoBaseImponible.multiply(invoice.getTasaIgv());

            BigDecimal total = ivap.getImporte().add(gravadoImporte).add(icb);

            TotalImpuestos totalImpuestos = TotalImpuestos
                    .builder()
                    .ivapImporte(ivap.getImporte())
                    .ivapBaseImponible(ivap.getBaseImponible())
                    .gravadoImporte(gravadoImporte)
                    .gravadoBaseImponible(gravadoBaseImponible)
                    .inafectoImporte(inafecto.getImporte())
                    .inafectoBaseImponible(inafecto.getBaseImponible())
                    .exoneradoImporte(exonerado.getImporte())
                    .exoneradoBaseImponible(exonerado.getBaseImponible())
                    .gratuitoImporte(gratuito.getImporte())
                    .gratuitoBaseImponible(gratuito.getBaseImponible())
                    .icbImporte(icb)
                    .total(total)
                    .build();

            invoice.setTotalImpuestos(totalImpuestos);
        };
        whenInvoice.apply(object).ifPresent(consumer);
    }
}
