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
import java.time.LocalDate;
import java.time.LocalTime;
import java.util.List;

@Data
@NoArgsConstructor
public class XMLDespatchAdvice {

    @XmlPath("cbc:ID/text()")
    private String documentId;

    @XmlPath("cbc:IssueDate/text()")
    private LocalDate issueDate;

    @XmlPath("cbc:IssueTime/text()")
    private LocalTime issueTime;

    @XmlPath("cbc:DespatchAdviceTypeCode/text()")
    private String despatchAdviceTypeCode;

    @XmlPath("cbc:Note/text()")
    private String note;

    @XmlPath("cac:OrderReference")
    private OrderReference orderReference;

    @XmlPath("cac:AdditionalDocumentReference")
    private AdditionalDocumentReference additionalDocumentReference;

    @XmlPath("cac:Signature")
    private XMLSignature signature;

    @XmlPath("cac:DespatchSupplierParty")
    private DespatchSupplierParty despatchSupplierParty;

    @XmlPath("cac:DeliveryCustomerParty")
    private DeliveryCustomerParty deliveryCustomerParty;

    @XmlPath("cac:SellerSupplierParty")
    private SellerSupplierParty sellerSupplierParty;

    @XmlPath("cac:Shipment")
    private Shipment shipment;

    @XmlPath("cac:DespatchLine")
    private List<XMLDespatchAdviceLine> lines;

    @Data
    @NoArgsConstructor
    public static class OrderReference {
        @XmlPath("cbc:ID/text()")
        private String id;

        @XmlPath("cbc:OrderTypeCode/text()")
        private String orderTypeCode;
    }

    @Data
    @NoArgsConstructor
    public static class AdditionalDocumentReference {
        @XmlPath("cbc:ID/text()")
        private String id;

        @XmlPath("cbc:DocumentTypeCode/text()")
        private String documentTypeCode;
    }

    @Data
    @NoArgsConstructor
    public static class DespatchSupplierParty {
        @XmlPath("cac:Party/cac:PartyIdentification/cbc:ID/text()")
        private String customerAssignedAccountID;

        @XmlPath("cac:Party/cac:PartyLegalEntity/cbc:RegistrationName/text()")
        private String registrationName;
    }

    @Data
    @NoArgsConstructor
    public static class DeliveryCustomerParty {
        @XmlPath("cac:Party/cac:PartyIdentification/cbc:ID/text()")
        private String partyIdentification_id;

        @XmlPath("cac:Party/cac:PartyIdentification/cbc:ID/@schemeID")
        private String customerAssignedAccountID_schemeId;

        @XmlPath("cac:Party/cac:PartyLegalEntity/cbc:RegistrationName/text()")
        private String registrationName;
    }

    @Data
    @NoArgsConstructor
    public static class SellerSupplierParty {
        @XmlPath("cbc:CustomerAssignedAccountID/text()")
        private String customerAssignedAccountId;

        @XmlPath("cac:Party/cac:PartyLegalEntity/cbc:RegistrationName/text()")
        private String registrationName;
    }

    @Data
    @NoArgsConstructor
    public static class Shipment {
        @XmlPath("cbc:HandlingCode/text()")
        private String handlingCode;

        @XmlPath("cbc:Information/text()")
        private String information;

        @XmlPath("cbc:GrossWeightMeasure/text()")
        private BigDecimal grossWeightMeasure;

        @XmlPath("cbc:GrossWeightMeasure/@unitCode")
        private String grossWeightMeasure_unitCode;

        @XmlPath("cbc:TotalTransportHandlingUnitQuantity/text()")
        private Integer totalTransportHandlingUnitQuantity;

        @XmlPath("cbc:SplitConsignmentIndicator/text()")
        private Boolean splitConsignmentIndicator;

        @XmlPath("cac:ShipmentStage/cbc:TransportModeCode/text()")
        private String transportModeCode;

        @XmlPath("cac:ShipmentStage/cac:TransitPeriod/cbc:StartDate/text()")
        private LocalDate startDate;

        @XmlPath("cac:ShipmentStage/cac:CarrierParty")
        private CarrierParty carrierParty;

        @XmlPath("cac:ShipmentStage/cac:TransportMeans")
        private TransportMeans transportMeans;

        @XmlPath("cac:ShipmentStage/cac:DriverPerson")
        private DriverPerson driverPerson;

        @XmlPath("cac:Delivery/cac:DeliveryAddress/cbc:ID/text()")
        private String deliveryAddress_id;

        @XmlPath("cac:Delivery/cac:DeliveryAddress/cac:AddressLine/cbc:Line/text()")
        private String deliveryAddress_line;

        @XmlPath("cac:TransportHandlingUnit/cac:TransportEquipment/cbc:ID/text()")
        private String transportEquipment_id;

        @XmlPath("cac:OriginAddress/cbc:ID/text()")
        private String originAddress_id;

        @XmlPath("cac:OriginAddress/cbc:StreetName/text()")
        private String originAddress_streetName;

        @XmlPath("cac:FirstArrivalPortLocation/cbc:ID/text()")
        private String firstArrivalPortLocation_id;
    }

    @Data
    @NoArgsConstructor
    public static class CarrierParty {
        @XmlPath("cac:PartyIdentification/cbc:ID/text()")
        private String partyIdentification_id;

        @XmlPath("cac:PartyIdentification/cbc:ID/@schemeID")
        private String partyIdentification_schemeId;

        @XmlPath("cac:PartyName/cbc:Name/text()")
        private String partyName;
    }

    @Data
    @NoArgsConstructor
    public static class TransportMeans {
        @XmlPath("cac:RoadTransport/cbc:LicensePlateID/text()")
        private String licensePlateId;
    }

    @Data
    @NoArgsConstructor
    public static class DriverPerson {
        @XmlPath("cbc:ID/text()")
        private String id;

        @XmlPath("cbc:ID/@schemeID")
        private String id_schemeId;
    }
}
