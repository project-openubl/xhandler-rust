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
package io.github.project.openubl.xbuilder.enricher.kie.rules.utils;

import io.github.project.openubl.xbuilder.content.models.common.Document;
import io.github.project.openubl.xbuilder.content.models.standard.general.Anticipo;
import io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoVentaDetalle;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.content.models.standard.general.Note;
import io.github.project.openubl.xbuilder.content.models.standard.general.SalesDocument;
import io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocuments;
import io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocumentsItem;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Perception;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Retention;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocuments;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocumentsItem;

import java.util.Optional;
import java.util.function.Function;
import java.util.function.Predicate;

public class Helpers {

    public static final Predicate<Object> isInvoice = o -> o instanceof Invoice;
    public static final Predicate<Object> isCreditNote = o -> o instanceof CreditNote;
    public static final Predicate<Object> isDebitNote = o -> o instanceof DebitNote;
    public static final Predicate<Object> isNote = o -> o instanceof Note;

    public static final Predicate<Object> isVoidedDocuments = o -> o instanceof VoidedDocuments;
    public static final Predicate<Object> isVoidedDocumentsItem = o -> o instanceof VoidedDocumentsItem;
    public static final Predicate<Object> isSummaryDocuments = o -> o instanceof SummaryDocuments;
    public static final Predicate<Object> isSummaryDocumentsItem = o -> o instanceof SummaryDocumentsItem;

    public static final Predicate<Object> isPerception = o -> o instanceof Perception;
    public static final Predicate<Object> isRetention = o -> o instanceof Retention;

    public static final Predicate<Object> isDocument = isInvoice.or(isCreditNote).or(isDebitNote)
            .or(isVoidedDocuments).or(isSummaryDocuments)
            .or(isPerception).or(isRetention);

    public static final Predicate<Object> isSalesDocument = isInvoice.or(isCreditNote).or(isDebitNote);
    public static final Predicate<Object> isSalesDocumentItem = o -> o instanceof DocumentoVentaDetalle;

    public static final Predicate<Object> isAnticipo = o -> o instanceof Anticipo;

    public static final Function<Object, Optional<Invoice>> whenInvoice = o -> {
        if (o instanceof Invoice) {
            return Optional.of((Invoice) o);
        }
        return Optional.empty();
    };

    public static final Function<Object, Optional<Document>> whenDocument = o -> {
        if (o instanceof Document) {
            return Optional.of((Document) o);
        }
        return Optional.empty();
    };

    public static final Function<Object, Optional<CreditNote>> whenCreditNote = o -> {
        if (o instanceof CreditNote) {
            return Optional.of((CreditNote) o);
        }
        return Optional.empty();
    };

    public static final Function<Object, Optional<DebitNote>> whenDebitNote = o -> {
        if (o instanceof DebitNote) {
            return Optional.of((DebitNote) o);
        }
        return Optional.empty();
    };

    public static final Function<Object, Optional<SalesDocument>> whenSalesDocument = o -> {
        if (o instanceof SalesDocument) {
            return Optional.of((SalesDocument) o);
        }
        return Optional.empty();
    };

    public static final Function<Object, Optional<Note>> whenNote = o -> {
        if (o instanceof Note) {
            return Optional.of((Note) o);
        }
        return Optional.empty();
    };

    public static final Function<Object, Optional<DocumentoVentaDetalle>> whenSalesDocumentItem = o -> {
        if (o instanceof DocumentoVentaDetalle) {
            return Optional.of((DocumentoVentaDetalle) o);
        }
        return Optional.empty();
    };

    public static final Function<Object, Optional<Anticipo>> whenAnticipo = o -> {
        if (o instanceof Anticipo) {
            return Optional.of((Anticipo) o);
        }
        return Optional.empty();
    };

    public static final Function<Object, Optional<VoidedDocumentsItem>> whenVoidedDocumentsItem = o -> {
        if (o instanceof VoidedDocumentsItem) {
            return Optional.of((VoidedDocumentsItem) o);
        }
        return Optional.empty();
    };

    public static final Function<Object, Optional<SummaryDocumentsItem>> whenSummaryDocumentsItem = o -> {
        if (o instanceof SummaryDocumentsItem) {
            return Optional.of((SummaryDocumentsItem) o);
        }
        return Optional.empty();
    };
}
