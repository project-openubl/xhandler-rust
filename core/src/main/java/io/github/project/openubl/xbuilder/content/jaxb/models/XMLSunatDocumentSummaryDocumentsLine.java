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


import java.math.BigDecimal;
import java.util.List;

@Data
@NoArgsConstructor
public class XMLSunatDocumentSummaryDocumentsLine {

    @XmlPath("cbc:DocumentTypeCode/text()")
    private String documentTypeCode;

    @XmlPath("cbc:ID/text()")
    private String documentId;

    @XmlPath("cac:AccountingCustomerParty/cbc:CustomerAssignedAccountID/text()")
    private String accountingCustomerParty_customerAssignedAccountId;

    @XmlPath("cac:AccountingCustomerParty/cbc:AdditionalAccountID/text()")
    private String accountingCustomerParty_additionalAccountID;

    @XmlPath("cac:BillingReference")
    private BillingReference billingReference;

    @XmlPath("cac:Status/cbc:ConditionCode/text()")
    private String status_conditionCode;

    @XmlPath("sac:TotalAmount/text()")
    private BigDecimal totalAmount;

    @XmlPath("sac:TotalAmount/@currencyID")
    private String totalAmount_currencyID;

    @XmlPath("sac:BillingPayment")
    private List<BillingPayment> billingPayments;

    @XmlPath("cac:AllowanceCharge/cbc:Amount/text()")
    private BigDecimal allowanceCharge_amount;

    @XmlPath("cac:TaxTotal")
    private List<TaxTotalSummaryDocuments> taxTotals;

    @Data
    @NoArgsConstructor
    public static class BillingPayment {
        @XmlPath("cbc:PaidAmount/text()")
        BigDecimal paidAmount;

        @XmlPath("cbc:InstructionID/text()")
        String instructionId;
    }

    @Data
    @NoArgsConstructor
    public static class BillingReference {
        @XmlPath("cac:InvoiceDocumentReference/cbc:ID/text()")
        private String invoiceDocumentReference_id;

        @XmlPath("cac:InvoiceDocumentReference/cbc:DocumentTypeCode/text()")
        private String invoiceDocumentReference_documentTypeCode;
    }

    @Data
    @NoArgsConstructor
    public static class TaxTotalSummaryDocuments {
        @XmlPath("cbc:TaxAmount/text()")
        BigDecimal taxAmount;

        @XmlPath("cac:TaxSubtotal")
        TaxSubtotalTaxTotalSummaryDocuments taxSubtotals;
    }

    @Data
    @NoArgsConstructor
    public static class TaxSubtotalTaxTotalSummaryDocuments {
        @XmlPath("cbc:TaxAmount/text()")
        private BigDecimal taxAmount;

        @XmlPath("cac:TaxCategory/cac:TaxScheme/cbc:ID/text()")
        private String code;
    }
}
