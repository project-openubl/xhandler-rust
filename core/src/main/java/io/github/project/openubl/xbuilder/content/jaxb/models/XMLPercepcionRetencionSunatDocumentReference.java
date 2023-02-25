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
package io.github.project.openubl.xbuilder.content.jaxb.models;

import lombok.Data;
import lombok.NoArgsConstructor;
import org.eclipse.persistence.oxm.annotations.XmlPath;

import java.math.BigDecimal;
import java.time.LocalDate;

@Data
@NoArgsConstructor
public class XMLPercepcionRetencionSunatDocumentReference {

    @XmlPath("cbc:ID/text()")
    private String id;

    @XmlPath("cbc:ID/@schemeID")
    private String id_schemeId;

    @XmlPath("cbc:IssueDate/text()")
    private LocalDate issueDate;

    @XmlPath("cbc:TotalInvoiceAmount/text()")
    private BigDecimal totalInvoiceAmount;

    @XmlPath("cbc:TotalInvoiceAmount/@currencyID")
    private String totalInvoiceAmount_currencyId;

    @XmlPath("cac:Payment/cbc:ID/text()")
    private Integer payment_id;

    @XmlPath("cac:Payment/cbc:PaidAmount/text()")
    private BigDecimal payment_paidAmount;

    @XmlPath("cac:Payment/cbc:PaidDate/text()")
    private LocalDate payment_paidDate;

    private XMLPercepcionRetencionInformation sunatInformation;
}
