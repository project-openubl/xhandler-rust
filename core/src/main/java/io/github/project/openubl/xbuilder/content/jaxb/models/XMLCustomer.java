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

import jakarta.xml.bind.annotation.XmlAccessType;
import jakarta.xml.bind.annotation.XmlAccessorType;
import jakarta.xml.bind.annotation.XmlAttribute;
import jakarta.xml.bind.annotation.XmlElement;
import jakarta.xml.bind.annotation.XmlType;
import jakarta.xml.bind.annotation.XmlValue;

@XmlAccessorType(XmlAccessType.NONE)
@XmlType(name = "Customer.Party")
@Data
@NoArgsConstructor
public class XMLCustomer {

    @XmlElement(name = "PartyIdentification", namespace = XMLConstants.CAC)
    private PartyIdentification partyIdentification;

    @XmlElement(name = "PartyLegalEntity", namespace = XMLConstants.CAC)
    private PartyLegalEntity partyLegalEntity;

    @XmlElement(name = "Contact", namespace = XMLConstants.CAC)
    private XMLContact contact;

    @XmlType(name = "Customer.PartyIdentification")
    @XmlAccessorType(XmlAccessType.NONE)
    @Data
    @NoArgsConstructor
    public static class PartyIdentification {
        @XmlElement(name = "ID", namespace = XMLConstants.CBC)
        private PartyIdentification_ID id;
    }

    @XmlType(name = "Customer.ID")
    @XmlAccessorType(XmlAccessType.NONE)
    @Data
    @NoArgsConstructor
    public static class PartyIdentification_ID {
        @XmlValue
        private String value;

        @XmlAttribute(name = "schemeID")
        private String schemeID;
    }

    @XmlType(name = "Customer.PartyLegalEntity")
    @XmlAccessorType(XmlAccessType.NONE)
    @Data
    @NoArgsConstructor
    public static class PartyLegalEntity {
        @XmlElement(name = "RegistrationName", namespace = XMLConstants.CBC)
        private String registrationName;

        @XmlElement(name = "RegistrationAddress", namespace = XMLConstants.CAC)
        private XMLAddress address;
    }
}
