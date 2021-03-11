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

import io.github.project.openubl.xmlbuilderlib.clock.SystemClock;
import io.github.project.openubl.xmlbuilderlib.clock.SystemClockSingleton;
import io.github.project.openubl.xmlbuilderlib.config.Config;
import io.github.project.openubl.xmlbuilderlib.freemarker.FreemarkerExecutor;
import io.github.project.openubl.xmlbuilderlib.hibernate.validator.ValidatorSingleton;
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

import javax.validation.ConstraintViolation;
import javax.validation.ConstraintViolationException;
import javax.validation.Validator;
import java.util.Set;

public class DocumentManager {

    private DocumentManager() {
        // Only static methods
    }

    private static SystemClock getDefaultSystemClock() {
        return SystemClockSingleton.getInstance().getClock();
    }

    private static void validateInput(Object input) {
        Validator validator = ValidatorSingleton.getInstance().getValidator();
        Set<ConstraintViolation<Object>> violations = validator.validate(input, CompleteValidation.class);
        if (!violations.isEmpty()) {
            throw new ConstraintViolationException(violations);
        }
    }

    public static DocumentWrapper<InvoiceOutputModel> createXML(InvoiceInputModel input, Config config) {
        validateInput(input);
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<InvoiceOutputModel> createXML(InvoiceInputModel input, Config config, SystemClock systemClock) {
        validateInput(input);
        InvoiceOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<CreditNoteOutputModel> createXML(CreditNoteInputModel input, Config config) {
        validateInput(input);
        CreditNoteOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<CreditNoteOutputModel> createXML(CreditNoteInputModel input, Config config, SystemClock systemClock) {
        validateInput(input);
        CreditNoteOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<DebitNoteOutputModel> createXML(DebitNoteInputModel input, Config config) {
        validateInput(input);
        DebitNoteOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<DebitNoteOutputModel> createXML(DebitNoteInputModel input, Config config, SystemClock systemClock) {
        DebitNoteOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<VoidedDocumentOutputModel> createXML(VoidedDocumentInputModel input, Config config) {
        validateInput(input);
        VoidedDocumentOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<VoidedDocumentOutputModel> createXML(VoidedDocumentInputModel input, Config config, SystemClock systemClock) {
        validateInput(input);
        VoidedDocumentOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<SummaryDocumentOutputModel> createXML(SummaryDocumentInputModel input, Config config) {
        validateInput(input);
        SummaryDocumentOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<SummaryDocumentOutputModel> createXML(SummaryDocumentInputModel input, Config config, SystemClock systemClock) {
        validateInput(input);
        SummaryDocumentOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<PerceptionOutputModel> createXML(PerceptionInputModel input, Config config) {
        validateInput(input);
        PerceptionOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<PerceptionOutputModel> createXML(PerceptionInputModel input, Config config, SystemClock systemClock) {
        validateInput(input);
        PerceptionOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<RetentionOutputModel> createXML(RetentionInputModel input, Config config) {
        validateInput(input);
        RetentionOutputModel output = InputToOutput.toOutput(input, config, getDefaultSystemClock());
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }

    public static DocumentWrapper<RetentionOutputModel> createXML(RetentionInputModel input, Config config, SystemClock systemClock) {
        validateInput(input);
        RetentionOutputModel output = InputToOutput.toOutput(input, config, systemClock);
        String xml = FreemarkerExecutor.createXML(output);
        return new DocumentWrapper<>(xml, output);
    }
}
