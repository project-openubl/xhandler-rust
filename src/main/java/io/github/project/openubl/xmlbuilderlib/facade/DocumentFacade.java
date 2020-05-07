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
package io.github.project.openubl.xmlbuilderlib.facade;

import io.github.project.openubl.xmlbuilderlib.config.XMLBuilderConfig;
import io.github.project.openubl.xmlbuilderlib.freemarker.FreemarkerExecutor;
import io.github.project.openubl.xmlbuilderlib.models.input.constraints.CompleteValidation;
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

import javax.validation.*;
import java.util.Calendar;
import java.util.Set;
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

    private static void validateInput(Object input) {
        ValidatorFactory factory = Validation.buildDefaultValidatorFactory();
        Validator validator = factory.getValidator();
        Set<ConstraintViolation<Object>> violations = validator.validate(input, CompleteValidation.class);
        if (!violations.isEmpty()) {
            throw new ConstraintViolationException(violations);
        }
    }

    public static String createXML(InvoiceInputModel input, XMLBuilderConfig config) {
        validateInput(input);
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(InvoiceInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        validateInput(input);
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(CreditNoteInputModel input, XMLBuilderConfig config) {
        validateInput(input);
        CreditNoteOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(CreditNoteInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        validateInput(input);
        CreditNoteOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(DebitNoteInputModel input, XMLBuilderConfig config) {
        validateInput(input);
        DebitNoteOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(DebitNoteInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        DebitNoteOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(VoidedDocumentInputModel input, XMLBuilderConfig config) {
        validateInput(input);
        VoidedDocumentOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(VoidedDocumentInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        validateInput(input);
        VoidedDocumentOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(SummaryDocumentInputModel input, XMLBuilderConfig config) {
        validateInput(input);
        SummaryDocumentOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(SummaryDocumentInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        validateInput(input);
        SummaryDocumentOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(PerceptionInputModel input, XMLBuilderConfig config) {
        validateInput(input);
        PerceptionOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(PerceptionInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        validateInput(input);
        PerceptionOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(RetentionInputModel input, XMLBuilderConfig config) {
        validateInput(input);
        RetentionOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        return FreemarkerExecutor.createXML(output);
    }

    public static String createXML(RetentionInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        validateInput(input);
        RetentionOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        return FreemarkerExecutor.createXML(output);
    }
}
