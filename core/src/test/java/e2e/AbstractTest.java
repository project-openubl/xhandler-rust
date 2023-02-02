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
package e2e;

import e2e.renderer.XMLAssertUtils;
import io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.enricher.ContentEnricher;
import io.github.project.openubl.xbuilder.enricher.config.DateProvider;
import io.github.project.openubl.xbuilder.enricher.config.Defaults;
import io.github.project.openubl.xbuilder.renderer.TemplateProducer;
import io.quarkus.qute.Template;

import java.math.BigDecimal;
import java.time.LocalDate;

public class AbstractTest {

    protected static final Defaults defaults = Defaults.builder()
            .moneda("PEN")
            .unidadMedida("NIU")
            .icbTasa(new BigDecimal("0.2"))
            .igvTasa(new BigDecimal("0.18"))
            .ivapTasa(new BigDecimal("0.04"))
            .build();

    protected static final DateProvider dateProvider = () -> LocalDate.of(2019, 12, 24);

    protected void assertInput(Invoice input, String snapshot) throws Exception {
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getInvoice();
        String xml = template.data(input).render();

        // Then
        XMLAssertUtils.assertSnapshot(xml, getClass(), snapshot);
        XMLAssertUtils.assertSendSunat(xml, XMLAssertUtils.INVOICE_XSD);
    }

    protected void assertInput(CreditNote input, String snapshot) throws Exception {
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getCreditNote();
        String xml = template.data(input).render();

        // Then
        XMLAssertUtils.assertSnapshot(xml, getClass(), snapshot);
        XMLAssertUtils.assertSendSunat(xml, XMLAssertUtils.CREDIT_NOTE_XSD);
    }

    protected void assertInput(DebitNote input, String snapshot) throws Exception {
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getDebitNote();
        String xml = template.data(input).render();

        // Then
        XMLAssertUtils.assertSnapshot(xml, getClass(), snapshot);
        XMLAssertUtils.assertSendSunat(xml, XMLAssertUtils.DEBIT_NOTE_XSD);
    }
}
