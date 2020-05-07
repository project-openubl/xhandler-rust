/**
 * Copyright 2019 Project OpenUBL, Inc. and/or its affiliates
 * and other contributors as indicated by the @author tags.
 *
 * Licensed under the Eclipse Public License - v 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.eclipse.org/legal/epl-2.0/
 *
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

public class FreemarkerExecutor {

    private FreemarkerExecutor() {
        // Only static methods
    }

    public static String createXML(InvoiceOutputModel output) {
        StringWriter buffer;
        try {
            Template template = FreemarkerGlobalConfiguration.getInstance().getConfiguration().getTemplate(FreemarkerConstants.INVOICE_TEMPLATE_2_1);

            buffer = new StringWriter();
            template.process(output, buffer);
            buffer.flush();
        } catch (IOException | TemplateException e) {
            throw new IllegalStateException(e);
        }

        return buffer.toString();
    }

    public static String createXML(CreditNoteOutputModel output) {
        StringWriter buffer;
        try {
            Template template = FreemarkerGlobalConfiguration.getInstance().getConfiguration().getTemplate(FreemarkerConstants.CREDIT_NOTE_TEMPLATE_2_1);

            buffer = new StringWriter();
            template.process(output, buffer);
            buffer.flush();
        } catch (IOException | TemplateException e) {
            throw new IllegalStateException(e);
        }

        return buffer.toString();
    }

    public static String createXML(DebitNoteOutputModel output) {
        StringWriter buffer;
        try {
            Template template = FreemarkerGlobalConfiguration.getInstance().getConfiguration().getTemplate(FreemarkerConstants.DEBIT_NOTE_TEMPLATE_2_1);

            buffer = new StringWriter();
            template.process(output, buffer);
            buffer.flush();
        } catch (IOException | TemplateException e) {
            throw new IllegalStateException(e);
        }

        return buffer.toString();
    }

    public static String createXML(VoidedDocumentOutputModel output) {
        StringWriter buffer;
        try {
            Template template = FreemarkerGlobalConfiguration.getInstance().getConfiguration().getTemplate(FreemarkerConstants.VOIDED_DOCUMENT_TEMPLATE_2_0);

            buffer = new StringWriter();
            template.process(output, buffer);
            buffer.flush();
        } catch (IOException | TemplateException e) {
            throw new IllegalStateException(e);
        }

        return buffer.toString();
    }

    public static String createXML(SummaryDocumentOutputModel output) {
        StringWriter buffer;
        try {
            Template template = FreemarkerGlobalConfiguration.getInstance().getConfiguration().getTemplate(FreemarkerConstants.SUMMARY_DOCUMENT_TEMPLATE_2_0);

            buffer = new StringWriter();
            template.process(output, buffer);
            buffer.flush();
        } catch (IOException | TemplateException e) {
            throw new IllegalStateException(e);
        }

        return buffer.toString();
    }

    public static String createXML(PerceptionOutputModel output) {
        StringWriter buffer;
        try {
            Template template = FreemarkerGlobalConfiguration.getInstance().getConfiguration().getTemplate(FreemarkerConstants.PERCEPTION_TEMPLATE_2_0);

            buffer = new StringWriter();
            template.process(output, buffer);
            buffer.flush();
        } catch (IOException | TemplateException e) {
            throw new IllegalStateException(e);
        }

        return buffer.toString();
    }

    public static String createXML(RetentionOutputModel output) {
        StringWriter buffer;
        try {
            Template template = FreemarkerGlobalConfiguration.getInstance().getConfiguration().getTemplate(FreemarkerConstants.RETENTION_TEMPLATE_2_0);

            buffer = new StringWriter();
            template.process(output, buffer);
            buffer.flush();
        } catch (IOException | TemplateException e) {
            throw new IllegalStateException(e);
        }

        return buffer.toString();
    }

    public static String createXML(DespatchAdviceOutputModel output) {
        StringWriter buffer;
        try {
            Template template = FreemarkerGlobalConfiguration.getInstance().getConfiguration().getTemplate(FreemarkerConstants.DESPATCH_ADVICE_TEMPLATE_2_1);

            buffer = new StringWriter();
            template.process(output, buffer);
            buffer.flush();
        } catch (IOException | TemplateException e) {
            throw new IllegalStateException(e);
        }

        return buffer.toString();
    }
}
