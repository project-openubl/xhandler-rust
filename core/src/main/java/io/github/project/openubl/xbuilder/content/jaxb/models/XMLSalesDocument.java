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
import java.time.LocalTime;
import java.util.List;

@Data
@NoArgsConstructor
public class XMLSalesDocument {

    @XmlPath("cbc:ID/text()")
    private String documentId;

    @XmlPath("cbc:IssueDate/text()")
    private LocalDate issueDate;

    @XmlPath("cbc:IssueTime/text()")
    private LocalTime issueTime;

    @XmlPath("cbc:DueDate/text()")
    private LocalDate dueDate;

    @XmlPath("cbc:InvoiceTypeCode/text()")
    private String invoiceTypeCode;

    @XmlPath("cbc:InvoiceTypeCode/@listID")
    private String invoiceTypeCode_listID;

    @XmlPath("cbc:Note")
    private List<Note> notes;

    @XmlPath("cbc:DocumentCurrencyCode/text()")
    private String documentCurrencyCode;

    @XmlPath("cac:OrderReference/cbc:ID/text()")
    private String orderReferenceId;

    @XmlPath("cac:DespatchDocumentReference")
    private List<DespatchDocumentReference> despatchDocumentReferences;

    @XmlPath("cac:AdditionalDocumentReference")
    private List<AdditionalDocumentReference> additionalDocumentReferences;

    @XmlPath("cac:Signature")
    private XMLSignature signature;

    @XmlPath("cac:AccountingSupplierParty/cac:Party")
    private XMLSupplier accountingSupplierParty;

    @XmlPath("cac:AccountingCustomerParty/cac:Party")
    private XMLCustomer accountingCustomerParty;

    @XmlPath("cac:Delivery/cac:DeliveryLocation/cac:Address")
    private XMLAddress deliveryLocation;

    @XmlPath("cac:PaymentMeans")
    private PaymentMeans paymentMeans;

    @XmlPath("cac:PaymentTerms")
    private List<PaymentTerms> paymentTerms;

    @XmlPath("cac:PrepaidPayment")
    private List<PrepaidPayment> prepaidPayments;

    @XmlPath("cac:AllowanceCharge")
    private List<AllowanceCharge> allowanceCharges;

    @XmlPath("cac:TaxTotal")
    private TaxTotal taxTotal;

    // When Invoice or CreditNote then @XmlPath("cac:LegalMonetaryTotal")
    // When DebitNote @XmlPath("cac:RequestedMonetaryTotal")
    // Therefore defining it in bindings/*.xml
    private MonetaryTotal monetaryTotal;

    private List<XMLSalesDocumentLine> lines;

    // Note
    @XmlPath("cac:DiscrepancyResponse/cbc:ReferenceID/text()")
    String discrepancyResponse_referenceId;

    @XmlPath("cac:DiscrepancyResponse/cbc:ResponseCode/text()")
    String discrepancyResponse_responseCode;

    @XmlPath("cac:DiscrepancyResponse/cbc:Description/text()")
    String discrepancyResponse_description;

    //

    @Data
    @NoArgsConstructor
    public static class Note {
        @XmlPath("@languageLocaleID")
        private String languageLocaleId;

        @XmlPath("text()")
        private String value;
    }

    @Data
    @NoArgsConstructor
    public static class DespatchDocumentReference {
        @XmlPath("cbc:ID/text()")
        private String id;

        @XmlPath("cbc:DocumentTypeCode/text()")
        private String documentTypeCode;
    }

    @Data
    @NoArgsConstructor
    public static class AdditionalDocumentReference {
        @XmlPath("cbc:ID/text()")
        private String id;

        @XmlPath("cbc:DocumentTypeCode/text()")
        private String documentTypeCode;

        @XmlPath("cbc:DocumentStatusCode/text()")
        private String documentStatusCode;
    }

    @Data
    @NoArgsConstructor
    public static class PaymentMeans {
        @XmlPath("cbc:PaymentMeansCode/text()")
        private String paymentMeansCode;

        @XmlPath("cac:PayeeFinancialAccount/cbc:ID/text()")
        private String payeeFinancialAccount_id;
    }

    @Data
    @NoArgsConstructor
    public static class PaymentTerms {
        @XmlPath("cbc:ID/text()")
        private String id;

        @XmlPath("cbc:PaymentMeansID/text()")
        private String paymentMeansID;

        @XmlPath("cbc:Amount/text()")
        private BigDecimal amount;

        @XmlPath("cbc:PaymentDueDate/text()")
        private LocalDate paymentDueDate;

        @XmlPath("cbc:PaymentPercent/text()")
        private BigDecimal paymentPercent;
    }

    @Data
    @NoArgsConstructor
    public static class PrepaidPayment {
        @XmlPath("cbc:ID/text()")
        private String id;

        @XmlPath("cbc:PaidAmount/text()")
        private BigDecimal paidAmount;
    }

    @Data
    @NoArgsConstructor
    public static class AllowanceCharge {
        @XmlPath("cbc:ChargeIndicator/text()")
        private Boolean chargeIndicator;

        @XmlPath("cbc:AllowanceChargeReasonCode/text()")
        private String allowanceChargeReasonCode;

        @XmlPath("cbc:MultiplierFactorNumeric/text()")
        private BigDecimal multiplierFactorNumeric;

        @XmlPath("cbc:Amount/text()")
        private BigDecimal amount;

        @XmlPath("cbc:BaseAmount/text()")
        private BigDecimal baseAmount;
    }

    @Data
    @NoArgsConstructor
    public static class TaxTotal {
        @XmlPath("cbc:TaxAmount/text()")
        BigDecimal taxAmount;

        @XmlPath("cac:TaxSubtotal")
        List<TaxSubtotal> taxSubtotals;
    }

    @Data
    @NoArgsConstructor
    public static class TaxSubtotal {
        @XmlPath("cbc:TaxableAmount/text()")
        private BigDecimal taxableAmount;

        @XmlPath("cbc:TaxAmount/text()")
        private BigDecimal taxAmount;

        @XmlPath("cac:TaxCategory/cac:TaxScheme/cbc:ID/text()")
        private String code;
    }

    @Data
    @NoArgsConstructor
    public static class MonetaryTotal {
        @XmlPath("cbc:LineExtensionAmount/text()")
        private BigDecimal lineExtensionAmount;

        @XmlPath("cbc:TaxInclusiveAmount/text()")
        private BigDecimal taxInclusiveAmount;

        @XmlPath("cbc:AllowanceTotalAmount/text()")
        private BigDecimal allowanceTotalAmount;

        @XmlPath("cbc:PrepaidAmount/text()")
        private BigDecimal prepaidAmount;

        @XmlPath("cbc:PayableAmount/text()")
        private BigDecimal payableAmount;
    }

}
