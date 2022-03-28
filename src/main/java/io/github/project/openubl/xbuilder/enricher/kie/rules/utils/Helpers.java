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

import io.github.project.openubl.xbuilder.content.models.standard.general.*;

import java.util.Optional;
import java.util.function.Function;
import java.util.function.Predicate;

public class Helpers {

    public static final Predicate<Object> isInvoice = o -> o instanceof Invoice;
    public static final Predicate<Object> isCreditNote = o -> o instanceof CreditNote;
    public static final Predicate<Object> isDebitNote = o -> o instanceof DebitNote;
    public static final Predicate<Object> isNote = o -> o instanceof BaseDocumentoNota;

    public static final Predicate<Object> isBaseDocumento = isInvoice.or(isCreditNote).or(isDebitNote);
    public static final Predicate<Object> isBaseDocumentoDetalle = o -> o instanceof DocumentoDetalle;

    public static final Function<Object, Optional<Invoice>> whenInvoice = o -> {
        if (o instanceof Invoice) {
            return Optional.of((Invoice) o);
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

    public static final Function<Object, Optional<BaseDocumento>> whenBaseDocumento = o -> {
        if (o instanceof BaseDocumento) {
            return Optional.of((BaseDocumento) o);
        }
        return Optional.empty();
    };

    public static final Function<Object, Optional<BaseDocumentoNota>> whenNote = o -> {
        if (o instanceof BaseDocumentoNota) {
            return Optional.of((BaseDocumentoNota) o);
        }
        return Optional.empty();
    };

    public static final Function<Object, Optional<DocumentoDetalle>> whenBaseDocumentoDetalle = o -> {
        if (o instanceof DocumentoDetalle) {
            return Optional.of((DocumentoDetalle) o);
        }
        return Optional.empty();
    };
}
