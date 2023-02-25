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
import java.util.List;

@Data
@NoArgsConstructor
public class XMLSalesDocumentLine {
    private BigDecimal quantity;
    private String quantity_unitCode;

    @XmlPath("cbc:LineExtensionAmount/text()")
    private BigDecimal lineExtensionAmount;

    @XmlPath("cac:PricingReference/cac:AlternativeConditionPrice/cbc:PriceAmount/text()")
    private BigDecimal alternativeConditionPrice_priceAmount;

    @XmlPath("cac:PricingReference/cac:AlternativeConditionPrice/cbc:PriceTypeCode/text()")
    private String alternativeConditionPrice_priceTypeCode;

    @XmlPath("cac:TaxTotal")
    private TaxTotalLine taxTotal;

    @XmlPath("cac:Item/cbc:Description/text()")
    private String description;

    @XmlPath("cac:Price/cbc:PriceAmount/text()")
    private BigDecimal priceAmount;

    @Data
    @NoArgsConstructor
    public static class TaxTotalLine {
        @XmlPath("cbc:TaxAmount/text()")
        BigDecimal taxAmount;

        @XmlPath("cac:TaxSubtotal")
        List<TaxSubtotalLine> taxSubtotals;
    }

    @Data
    @NoArgsConstructor
    public static class TaxSubtotalLine {
        @XmlPath("cbc:TaxableAmount/text()")
        private BigDecimal taxableAmount;

        @XmlPath("cbc:TaxAmount/text()")
        private BigDecimal taxAmount;

        @XmlPath("cac:TaxCategory/cbc:Percent/text()")
        private BigDecimal percent;

        @XmlPath("cac:TaxCategory/cbc:TierRange/text()")
        private String tierRange;

        @XmlPath("cac:TaxCategory/cbc:PerUnitAmount/text()")
        private BigDecimal perUnitAmount;

        @XmlPath("cac:TaxCategory/cbc:TaxExemptionReasonCode/text()")
        private String taxExemptionReasonCode;

        @XmlPath("cac:TaxCategory/cac:TaxScheme/cbc:ID/text()")
        private String code;
    }
}
