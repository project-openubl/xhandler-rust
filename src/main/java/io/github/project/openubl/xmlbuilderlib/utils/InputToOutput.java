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
package io.github.project.openubl.xmlbuilderlib.utils;

import io.github.project.openubl.xmlbuilderlib.config.XMLBuilderConfig;
import io.github.project.openubl.xmlbuilderlib.factory.InvoiceAndNoteOutputModelFactory;
import io.github.project.openubl.xmlbuilderlib.factory.PerceptionRetentionOutputModelFactory;
import io.github.project.openubl.xmlbuilderlib.factory.SummaryDocumentOutputModelFactory;
import io.github.project.openubl.xmlbuilderlib.factory.VoidedDocumentOutputModelFactory;
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

public class InputToOutput {

    private InputToOutput() {
        // Only static methods
    }

    public static InvoiceOutputModel toOutput(InvoiceInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        return InvoiceAndNoteOutputModelFactory.getInvoiceOutput(input, config, systemClock);
    }

    public static CreditNoteOutputModel toOutput(CreditNoteInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        return InvoiceAndNoteOutputModelFactory.getCreditNoteOutput(input, config, systemClock);
    }

    public static DebitNoteOutputModel toOutput(DebitNoteInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        return InvoiceAndNoteOutputModelFactory.getDebitNoteOutput(input, config, systemClock);
    }

    public static VoidedDocumentOutputModel toOutput(VoidedDocumentInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        return VoidedDocumentOutputModelFactory.getVoidedDocument(input, config, systemClock);
    }

    public static SummaryDocumentOutputModel toOutput(SummaryDocumentInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        return SummaryDocumentOutputModelFactory.getSummaryDocument(input, config, systemClock);
    }

    public static PerceptionOutputModel toOutput(PerceptionInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        return PerceptionRetentionOutputModelFactory.getPerception(input, config, systemClock);
    }

    public static RetentionOutputModel toOutput(RetentionInputModel input, XMLBuilderConfig config, SystemClock systemClock) {
        return PerceptionRetentionOutputModelFactory.getRetention(input, config, systemClock);
    }
}
