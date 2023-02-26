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

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
import com.fasterxml.jackson.dataformat.yaml.YAMLGenerator;
import com.fasterxml.jackson.dataformat.yaml.YAMLMapper;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule;
import e2e.renderer.XMLAssertUtils;
import io.github.project.openubl.xbuilder.content.jaxb.Unmarshall;
import io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.content.models.standard.guia.DespatchAdvice;
import io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocuments;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Perception;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Retention;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocuments;
import io.github.project.openubl.xbuilder.enricher.ContentEnricher;
import io.github.project.openubl.xbuilder.enricher.config.DateProvider;
import io.github.project.openubl.xbuilder.enricher.config.Defaults;
import io.github.project.openubl.xbuilder.renderer.TemplateProducer;
import io.quarkus.qute.Template;

import java.io.IOException;
import java.math.BigDecimal;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.LocalDate;
import java.util.Map;

public class AbstractTest {

    protected static final Defaults defaults = Defaults.builder()
            .icbTasa(new BigDecimal("0.2"))
            .igvTasa(new BigDecimal("0.18"))
            .ivapTasa(new BigDecimal("0.04"))
            .build();

    protected static final DateProvider dateProvider = () -> LocalDate.of(2019, 12, 24);

    public YAMLMapper getYamlMapper() {
        YAMLMapper mapper = new YAMLMapper(new YAMLFactory());
        mapper.registerModule(new JavaTimeModule());
        mapper.configure(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS, false);
        mapper.configure(YAMLGenerator.Feature.LITERAL_BLOCK_STYLE, true);
        mapper.setSerializationInclusion(JsonInclude.Include.NON_NULL);
        return mapper;
    }

    public void writeYaml(String kind, Object input, String snapshotFilename) throws URISyntaxException, IOException {
        String rootDir = getClass().getName().replaceAll("\\.", "/");

        String snapshotFileContent = Files.readString(Paths.get(getClass().getClassLoader().getResource(rootDir + "/" + snapshotFilename).toURI()));

        Path directoryPath = Paths.get("target", "openubl", "testcases").resolve(rootDir);
        Files.createDirectories(directoryPath);
        Path filePath = directoryPath.resolve(snapshotFilename.replaceAll(".xml", "") + ".yaml");

        getYamlMapper().writeValue(filePath.toFile(), Map.of(
                "kind", kind,
                "input", input,
                "snapshot", snapshotFileContent
        ));
    }

    protected void assertInput(Invoice input, String snapshotFilename) throws Exception {
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getInvoice();
        String xml = template.data(input).render();

        Invoice inputFromXml = Unmarshall.unmarshallInvoice(xml);
        String reconstructedXml = template.data(inputFromXml).render();

        // Then
        XMLAssertUtils.assertSnapshot(xml, reconstructedXml, getClass(), snapshotFilename);
        XMLAssertUtils.assertSendSunat(xml, XMLAssertUtils.INVOICE_XSD);

        writeYaml("Invoice", input, snapshotFilename);
    }

    protected void assertInput(CreditNote input, String snapshotFilename) throws Exception {
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getCreditNote();
        String xml = template.data(input).render();

        CreditNote inputFromXml = Unmarshall.unmarshallCreditNote(xml);
        String reconstructedXml = template.data(inputFromXml).render();

        // Then
        XMLAssertUtils.assertSnapshot(xml, reconstructedXml, getClass(), snapshotFilename);
        XMLAssertUtils.assertSendSunat(xml, XMLAssertUtils.CREDIT_NOTE_XSD);

        writeYaml("CreditNote", input, snapshotFilename);
    }

    protected void assertInput(DebitNote input, String snapshotFilename) throws Exception {
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getDebitNote();
        String xml = template.data(input).render();

        DebitNote inputFromXml = Unmarshall.unmarshallDebitNote(xml);
        String reconstructedXml = template.data(inputFromXml).render();

        // Then
        XMLAssertUtils.assertSnapshot(xml, reconstructedXml, getClass(), snapshotFilename);
        XMLAssertUtils.assertSendSunat(xml, XMLAssertUtils.DEBIT_NOTE_XSD);

        writeYaml("DebitNote", input, snapshotFilename);
    }

    protected void assertInput(VoidedDocuments input, String snapshotFilename) throws Exception {
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getVoidedDocument();
        String xml = template.data(input).render();

        VoidedDocuments inputFromXml = Unmarshall.unmarshallVoidedDocuments(xml);
        String reconstructedXml = template.data(inputFromXml).render();

        // Then
        XMLAssertUtils.assertSnapshot(xml, reconstructedXml, getClass(), snapshotFilename);
        XMLAssertUtils.assertSendSunat(xml, XMLAssertUtils.VOIDED_DOCUMENTS_XSD);

        writeYaml("VoidedDocuments", input, snapshotFilename);
    }

    protected void assertInput(SummaryDocuments input, String snapshotFilename) throws Exception {
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getSummaryDocuments();
        String xml = template.data(input).render();

        SummaryDocuments inputFromXml = Unmarshall.unmarshallSummaryDocuments(xml);
        String reconstructedXml = template.data(inputFromXml).render();

        // Then
        XMLAssertUtils.assertSnapshot(xml, reconstructedXml, getClass(), snapshotFilename);
        XMLAssertUtils.assertSendSunat(xml, XMLAssertUtils.SUMMARY_DOCUMENTS_XSD);

        writeYaml("SummaryDocuments", input, snapshotFilename);
    }

    protected void assertInput(Perception input, String snapshotFilename) throws Exception {
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getPerception();
        String xml = template.data(input).render();

        Perception inputFromXml = Unmarshall.unmarshallPerception(xml);
        String reconstructedXml = template.data(inputFromXml).render();

        // Then
        XMLAssertUtils.assertSnapshot(xml, reconstructedXml, getClass(), snapshotFilename);
        XMLAssertUtils.assertSendSunat(xml, XMLAssertUtils.PERCEPTION_XSD);

        writeYaml("Perception", input, snapshotFilename);
    }

    protected void assertInput(Retention input, String snapshotFilename) throws Exception {
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getRetention();
        String xml = template.data(input).render();

        Retention inputFromXml = Unmarshall.unmarshallRetention(xml);
        String reconstructedXml = template.data(inputFromXml).render();

        // Then
        XMLAssertUtils.assertSnapshot(xml, reconstructedXml, getClass(), snapshotFilename);
        XMLAssertUtils.assertSendSunat(xml, XMLAssertUtils.RETENTION_XSD);

        writeYaml("Retention", input, snapshotFilename);
    }

    protected void assertInput(DespatchAdvice input, String snapshotFilename) throws Exception {
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getDespatchAdvice();
        String xml = template.data(input).render();

        DespatchAdvice inputFromXml = Unmarshall.unmarshallDespatchAdvice(xml);
        String reconstructedXml = template.data(inputFromXml).render();

        // Then
        XMLAssertUtils.assertSnapshot(xml, reconstructedXml, getClass(), snapshotFilename);
        XMLAssertUtils.assertSendSunat(xml, XMLAssertUtils.DESPATCH_ADVICE_XSD);

        writeYaml("DespatchAdvice", input, snapshotFilename);
    }
}
