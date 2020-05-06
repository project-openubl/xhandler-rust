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

public class FreemarkerConstants {
    public static final String INVOICE_TEMPLATE_2_1 = "io/github/project/openubl/xmlbuilderlib/templates/standard/invoice.ftl";
    public static final String CREDIT_NOTE_TEMPLATE_2_1 = "io/github/project/openubl/xmlbuilderlib/templates/standard/credit-note.ftl";
    public static final String DEBIT_NOTE_TEMPLATE_2_1 = "io/github/project/openubl/xmlbuilderlib/templates/standard/debit-note.ftl";
    public static final String DESPATCH_ADVICE_TEMPLATE_2_1 = "io/github/project/openubl/xmlbuilderlib/templates/standard/despatch-advice.ftl";

    public static final String VOIDED_DOCUMENT_TEMPLATE_2_0 = "io/github/project/openubl/xmlbuilderlib/templates/sunat/voided-document.ftl";
    public static final String SUMMARY_DOCUMENT_TEMPLATE_2_0 = "io/github/project/openubl/xmlbuilderlib/templates/sunat/summary-document.ftl";
    public static final String PERCEPTION_TEMPLATE_2_0 = "io/github/project/openubl/xmlbuilderlib/templates/sunat/perception.ftl";
    public static final String RETENTION_TEMPLATE_2_0 = "io/github/project/openubl/xmlbuilderlib/templates/sunat/retention.ftl";

    private FreemarkerConstants() {
        // Just constants
    }
}
