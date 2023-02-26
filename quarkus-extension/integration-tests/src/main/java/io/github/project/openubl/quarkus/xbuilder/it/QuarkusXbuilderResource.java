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
package io.github.project.openubl.quarkus.xbuilder.it;

import io.github.project.openubl.quarkus.xbuilder.XBuilder;
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
import io.quarkus.qute.Template;
import io.vertx.core.json.JsonObject;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.ws.rs.Consumes;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;
import javax.xml.bind.JAXBException;
import java.io.IOException;
import java.time.LocalDate;

import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.CREDIT_NOTE;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.DEBIT_NOTE;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.DESPATCH_ADVICE;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.INVOICE;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.PERCEPTION;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.RETENTION;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.SUMMARY_DOCUMENTS;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.VOIDED_DOCUMENTS;

@Consumes(MediaType.APPLICATION_JSON)
@Produces(MediaType.TEXT_PLAIN)
@ApplicationScoped
@Path("/quarkus-xbuilder")
public class QuarkusXbuilderResource {

    @Inject
    XBuilder xBuilder;

    @POST
    @Path("Invoice/from-json")
    public String createInvoiceXml(JsonObject json) {
        Invoice invoice = json.mapTo(Invoice.class);

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(invoice);

        Template template = xBuilder.getTemplate(INVOICE);
        return template.data(invoice).render();
    }

    @Consumes(MediaType.TEXT_PLAIN)
    @POST
    @Path("Invoice/from-xml")
    public String createInvoicePojo(String xml) throws JAXBException, IOException {
        Invoice invoice = Unmarshall.unmarshallInvoice(xml);

        Template template = xBuilder.getTemplate(INVOICE);
        return template.data(invoice).render();
    }

    @POST
    @Path("CreditNote/from-json")
    public String createCreditNote(JsonObject json) {
        CreditNote creditNote = json.mapTo(CreditNote.class);

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(creditNote);

        Template template = xBuilder.getTemplate(CREDIT_NOTE);
        return template.data(creditNote).render();
    }

    @POST
    @Path("DebitNote/from-json")
    public String createDebitNote(JsonObject json) {
        DebitNote debitNote = json.mapTo(DebitNote.class);

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(debitNote);

        Template template = xBuilder.getTemplate(DEBIT_NOTE);
        return template.data(debitNote).render();
    }

    @POST
    @Path("VoidedDocuments/from-json")
    public String createVoidedDocuments(JsonObject json) {
        VoidedDocuments voidedDocuments = json.mapTo(VoidedDocuments.class);

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(voidedDocuments);

        Template template = xBuilder.getTemplate(VOIDED_DOCUMENTS);
        return template.data(voidedDocuments).render();
    }

    @POST
    @Path("SummaryDocuments/from-json")
    public String createSummaryDocuments(JsonObject json) {
        SummaryDocuments summaryDocuments = json.mapTo(SummaryDocuments.class);

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(summaryDocuments);

        Template template = xBuilder.getTemplate(SUMMARY_DOCUMENTS);
        return template.data(summaryDocuments).render();
    }

    @POST
    @Path("Perception/from-json")
    public String createPerception(JsonObject json) {
        Perception perception = json.mapTo(Perception.class);

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(perception);

        Template template = xBuilder.getTemplate(PERCEPTION);
        return template.data(perception).render();
    }

    @POST
    @Path("Retention/from-json")
    public String createRetention(JsonObject json) {
        Retention retention = json.mapTo(Retention.class);

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(retention);

        Template template = xBuilder.getTemplate(RETENTION);
        return template.data(retention).render();
    }

    @POST
    @Path("DespatchAdvice/from-json")
    public String createDespatchAdvice(JsonObject json) {
        DespatchAdvice despatchAdvice = json.mapTo(DespatchAdvice.class);

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(despatchAdvice);

        Template template = xBuilder.getTemplate(DESPATCH_ADVICE);
        return template.data(despatchAdvice).render();
    }

}
