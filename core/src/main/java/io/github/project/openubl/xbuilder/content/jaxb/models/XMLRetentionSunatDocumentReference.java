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

import io.github.project.openubl.xbuilder.content.jaxb.adapters.LocalDateAdapter;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;

import javax.xml.bind.annotation.XmlAccessType;
import javax.xml.bind.annotation.XmlAccessorType;
import javax.xml.bind.annotation.XmlElement;
import javax.xml.bind.annotation.XmlType;
import javax.xml.bind.annotation.adapters.XmlJavaTypeAdapter;
import java.math.BigDecimal;
import java.time.LocalDate;

@XmlAccessorType(XmlAccessType.NONE)
@XmlType(name = "XMLRetentionSunatDocumentReference")
@EqualsAndHashCode(callSuper = true)
@Data
@NoArgsConstructor
public class XMLRetentionSunatDocumentReference extends XMLPercepcionRetencionSunatDocumentReferenceBase {

    @XmlElement(name = "SUNATRetentionInformation", namespace = XMLConstants.SAC)
    private XMLRetencionInformation sunatInformation;

    @XmlAccessorType(XmlAccessType.NONE)
    @XmlType(name = "XMLRetentionSunatDocumentReference.XMLRetencionInformation")
    @Data
    @NoArgsConstructor
    public static class XMLRetencionInformation {
        @XmlElement(name = "SUNATRetentionAmount", namespace = XMLConstants.SAC)
        private BigDecimal sunatAmount;

        @XmlJavaTypeAdapter(LocalDateAdapter.class)
        @XmlElement(name = "SUNATRetentionDate", namespace = XMLConstants.SAC)
        private LocalDate sunatDate;

        @XmlElement(name = "SUNATNetTotalPaid", namespace = XMLConstants.SAC)
        private BigDecimal sunatNetTotal;

        @XmlElement(name = "ExchangeRate", namespace = XMLConstants.CAC)
        private ExchangeRate exchangeRate;
    }

    @XmlAccessorType(XmlAccessType.NONE)
    @XmlType(name = "XMLRetentionSunatDocumentReference.ExchangeRate")
    @Data
    @NoArgsConstructor
    public static class ExchangeRate {
        @XmlElement(name = "CalculationRate", namespace = XMLConstants.CBC)
        private BigDecimal calculationRate;

        @XmlJavaTypeAdapter(LocalDateAdapter.class)
        @XmlElement(name = "Date", namespace = XMLConstants.CBC)
        private LocalDate date;
    }
}
