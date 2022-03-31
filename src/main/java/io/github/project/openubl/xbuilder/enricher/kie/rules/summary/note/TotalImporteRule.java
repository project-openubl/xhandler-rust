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
package io.github.project.openubl.xbuilder.enricher.kie.rules.summary.note;

import io.github.project.openubl.xbuilder.content.models.standard.general.BaseDocumentoNota;
import io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteInvoice;
import io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteNote;
import io.github.project.openubl.xbuilder.enricher.kie.AbstractRule;
import io.github.project.openubl.xbuilder.enricher.kie.RulePhase;
import io.github.project.openubl.xbuilder.enricher.kie.rules.summary.utils.DetalleUtils;

import java.math.BigDecimal;
import java.util.function.Consumer;

import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.isNote;
import static io.github.project.openubl.xbuilder.enricher.kie.rules.utils.Helpers.whenNote;

@RulePhase(type = RulePhase.PhaseType.SUMMARY)
public class TotalImporteRule extends AbstractRule {

    @Override
    public boolean test(Object object) {
        return isNote.test(object) && whenNote.apply(object)
                .map(note -> note.getTotalImporte() == null && note.getDetalles() != null)
                .orElse(false);
    }

    @Override
    public void modify(Object object) {
        Consumer<BaseDocumentoNota> consumer = note -> {
            BigDecimal importeSinImpuestos = DetalleUtils.getImporteSinImpuestos(note.getDetalles());
            BigDecimal totalImpuestos = DetalleUtils.getTotalImpuestos(note.getDetalles());

            BigDecimal importe = importeSinImpuestos
                    .add(totalImpuestos);

            note.setTotalImporte(TotalImporteNote.builder()
                    .importe(importe)
                    .importeSinImpuestos(importeSinImpuestos)
                    .build()
            );
        };
        whenNote.apply(object).ifPresent(consumer);
    }

}
