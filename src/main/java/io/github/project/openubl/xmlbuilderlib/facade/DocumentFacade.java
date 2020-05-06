package io.github.project.openubl.xmlbuilderlib.facade;

import io.github.project.openubl.xmlbuilderlib.config.XMLBuilderConfig;
import io.github.project.openubl.xmlbuilderlib.freemarker.FreemarkerExecutor;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.invoice.InvoiceInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.note.creditNote.CreditNoteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.note.debitNote.DebitNoteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.PerceptionInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.RetentionInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.SummaryDocumentInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.sunat.VoidedDocumentInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.invoice.InvoiceOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.note.creditNote.CreditNoteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.note.debitNote.DebitNoteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.sunat.PerceptionOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.sunat.RetentionOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.sunat.SummaryDocumentOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.sunat.VoidedDocumentOutputModel;
import io.github.project.openubl.xmlbuilderlib.utils.InputToOutput;
import io.github.project.openubl.xmlbuilderlib.utils.SystemClock;

import java.util.Calendar;
import java.util.TimeZone;

public class DocumentFacade {

    private DocumentFacade() {
        // Only static methods
    }

    private static SystemClock getDefaultSystemClock() {
        return new SystemClock() {
            @Override
            public TimeZone getTimeZone() {
                return TimeZone.getTimeZone("America/Lima");
            }

            @Override
            public Calendar getCalendarInstance() {
                return Calendar.getInstance();
            }
        };
    }

    public static String createXML(InvoiceInputModel input, XMLBuilderConfig config) {
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(InvoiceInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(CreditNoteInputModel input, XMLBuilderConfig config) {
        CreditNoteOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(CreditNoteInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        CreditNoteOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(DebitNoteInputModel input, XMLBuilderConfig config) {
        DebitNoteOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(DebitNoteInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        DebitNoteOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(VoidedDocumentInputModel input, XMLBuilderConfig config) {
        VoidedDocumentOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(VoidedDocumentInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        VoidedDocumentOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(SummaryDocumentInputModel input, XMLBuilderConfig config) {
        SummaryDocumentOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(SummaryDocumentInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        SummaryDocumentOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(PerceptionInputModel input, XMLBuilderConfig config) {
        PerceptionOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(PerceptionInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        PerceptionOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(RetentionInputModel input, XMLBuilderConfig config) {
        RetentionOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(RetentionInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        RetentionOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }
}
