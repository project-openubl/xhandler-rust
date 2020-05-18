/**
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 * <p>
 * Licensed under the Eclipse Public License - v 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * <p>
 * https://www.eclipse.org/legal/epl-2.0/
 * <p>
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package io.github.project.openubl.xmlbuilderlib.freemarker;

import freemarker.template.Template;
import freemarker.template.TemplateException;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.despatchadvice.DespatchAdviceOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.invoice.InvoiceOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.note.creditNote.CreditNoteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.note.debitNote.DebitNoteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.sunat.PerceptionOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.sunat.RetentionOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.sunat.SummaryDocumentOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.sunat.VoidedDocumentOutputModel;

import java.io.IOException;
import java.io.StringWriter;
import java.io.Writer;

public class FreemarkerExecutor {

    private FreemarkerExecutor() {
        // Only static methods
    }


    public static String createXML(InvoiceOutputModel output) {
        return processTemplate(FreemarkerConstants.INVOICE_TEMPLATE_2_1, output);
    }

    public static void createXML(InvoiceOutputModel output, Writer writer) {
        processTemplate(FreemarkerConstants.INVOICE_TEMPLATE_2_1, output, writer);
    }


    public static String createXML(CreditNoteOutputModel output) {
        return processTemplate(FreemarkerConstants.CREDIT_NOTE_TEMPLATE_2_1, output);
    }

    public static void createXML(CreditNoteOutputModel output, Writer writer) {
        processTemplate(FreemarkerConstants.CREDIT_NOTE_TEMPLATE_2_1, output, writer);
    }


    public static String createXML(DebitNoteOutputModel output) {
        return processTemplate(FreemarkerConstants.DEBIT_NOTE_TEMPLATE_2_1, output);
    }

    public static void createXML(DebitNoteOutputModel output, Writer writer) {
        processTemplate(FreemarkerConstants.DEBIT_NOTE_TEMPLATE_2_1, output, writer);
    }


    public static String createXML(VoidedDocumentOutputModel output) {
        return processTemplate(FreemarkerConstants.VOIDED_DOCUMENT_TEMPLATE_2_0, output);
    }

    public static void createXML(VoidedDocumentOutputModel output, Writer writer) {
        processTemplate(FreemarkerConstants.VOIDED_DOCUMENT_TEMPLATE_2_0, output, writer);
    }


    public static String createXML(SummaryDocumentOutputModel output) {
        return processTemplate(FreemarkerConstants.SUMMARY_DOCUMENT_TEMPLATE_2_0, output);
    }

    public static void createXML(SummaryDocumentOutputModel output, Writer writer) {
        processTemplate(FreemarkerConstants.SUMMARY_DOCUMENT_TEMPLATE_2_0, output, writer);
    }


    public static String createXML(PerceptionOutputModel output) {
        return processTemplate(FreemarkerConstants.PERCEPTION_TEMPLATE_2_0, output);
    }

    public static void createXML(PerceptionOutputModel output, Writer writer) {
        processTemplate(FreemarkerConstants.PERCEPTION_TEMPLATE_2_0, output, writer);
    }


    public static String createXML(RetentionOutputModel output) {
        return processTemplate(FreemarkerConstants.RETENTION_TEMPLATE_2_0, output);
    }

    public static void createXML(RetentionOutputModel output, Writer writer) {
        processTemplate(FreemarkerConstants.RETENTION_TEMPLATE_2_0, output, writer);
    }


    public static String createXML(DespatchAdviceOutputModel output) {
        return processTemplate(FreemarkerConstants.DESPATCH_ADVICE_TEMPLATE_2_1, output);
    }

    public static void createXML(DespatchAdviceOutputModel output, Writer writer) {
        processTemplate(FreemarkerConstants.DESPATCH_ADVICE_TEMPLATE_2_1, output, writer);
    }


    private static String processTemplate(String templateName, Object dataModel) {
        StringWriter writer = new StringWriter();
        processTemplate(templateName, dataModel, writer);
        return writer.toString();
    }

    private static void processTemplate(String templateName, Object dataModel, Writer writer) {
        try {
            Template template = FreemarkerGlobalConfiguration.getInstance().getConfiguration().getTemplate(templateName);

            template.process(dataModel, writer);
            writer.flush();
            writer.close();
        } catch (IOException | TemplateException e) {
            throw new IllegalStateException(e);
        }
    }
}
