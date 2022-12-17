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
package io.github.project.openubl.quarkus.xbuilder.it;

import static io.restassured.RestAssured.given;
import static org.hamcrest.Matchers.is;

import io.quarkus.test.junit.QuarkusTest;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

@QuarkusTest
public class QuarkusXbuilderResourceTest {

    @Test
    public void testInvoice() {
        given()
                .when()
                .get("/quarkus-xbuilder/invoice")
                .then()
                .statusCode(200)
                .body(
                        is(
                                "<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n" +
                                        "<Invoice xmlns=\"urn:oasis:names:specification:ubl:schema:xsd:Invoice-2\"\n" +
                                        "         xmlns:cac=\"urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2\"\n" +
                                        "         xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\"\n" +
                                        "         xmlns:ccts=\"urn:un:unece:uncefact:documentation:2\"\n" +
                                        "         xmlns:cec=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\"\n" +
                                        "         xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\"\n" +
                                        "         xmlns:ext=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\"\n" +
                                        "         xmlns:qdt=\"urn:oasis:names:specification:ubl:schema:xsd:QualifiedDatatypes-2\"\n" +
                                        "         xmlns:sac=\"urn:sunat:names:specification:ubl:peru:schema:xsd:SunatAggregateComponents-1\"\n" +
                                        "         xmlns:udt=\"urn:un:unece:uncefact:data:specification:UnqualifiedDataTypesSchemaModule:2\"\n" +
                                        "         xmlns:xs=\"http://www.w3.org/2001/XMLSchema\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n" +
                                        ">\n" +
                                        "    <ext:UBLExtensions>\n" +
                                        "        <ext:UBLExtension>\n" +
                                        "            <ext:ExtensionContent/>\n" +
                                        "        </ext:UBLExtension>\n" +
                                        "    </ext:UBLExtensions>\n" +
                                        "    <cbc:UBLVersionID>2.1</cbc:UBLVersionID>\n" +
                                        "    <cbc:CustomizationID>2.0</cbc:CustomizationID>\n" +
                                        "    <cbc:ID>F001-1</cbc:ID>\n" +
                                        "    <cbc:IssueDate>2022-01-25</cbc:IssueDate>\n" +
                                        "    <cbc:InvoiceTypeCode listID=\"0101\" listAgencyName=\"PE:SUNAT\" listName=\"Tipo de Documento\" listURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01\">01</cbc:InvoiceTypeCode>\n" +
                                        "    <cbc:DocumentCurrencyCode listID=\"ISO 4217 Alpha\" listAgencyName=\"United Nations Economic Commission for Europe\" listName=\"Currency\">PEN</cbc:DocumentCurrencyCode>\n" +
                                        "    <cac:Signature>\n" +
                                        "        <cbc:ID>12345678912</cbc:ID>\n" +
                                        "        <cac:SignatoryParty>\n" +
                                        "            <cac:PartyIdentification>\n" +
                                        "                <cbc:ID>12345678912</cbc:ID>\n" +
                                        "            </cac:PartyIdentification>\n" +
                                        "            <cac:PartyName>\n" +
                                        "                <cbc:Name><![CDATA[Softgreen S.A.C.]]></cbc:Name>\n" +
                                        "            </cac:PartyName>\n" +
                                        "        </cac:SignatoryParty>\n" +
                                        "        <cac:DigitalSignatureAttachment>\n" +
                                        "            <cac:ExternalReference>\n" +
                                        "                <cbc:URI>#PROJECT-OPENUBL-SIGN</cbc:URI>\n" +
                                        "            </cac:ExternalReference>\n" +
                                        "        </cac:DigitalSignatureAttachment>\n" +
                                        "    </cac:Signature>\n" +
                                        "    <cac:AccountingSupplierParty>\n" +
                                        "        <cac:Party>\n" +
                                        "            <cac:PartyIdentification>\n" +
                                        "                <cbc:ID schemeID=\"6\" schemeAgencyName=\"PE:SUNAT\" schemeName=\"Documento de Identidad\" schemeURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06\">12345678912</cbc:ID>\n" +
                                        "            </cac:PartyIdentification>\n" +
                                        "            <cac:PartyLegalEntity>\n" +
                                        "                <cbc:RegistrationName><![CDATA[Softgreen S.A.C.]]></cbc:RegistrationName>\n" +
                                        "                <cac:RegistrationAddress>\n" +
                                        "                    <cbc:AddressTypeCode>0000</cbc:AddressTypeCode>\n" +
                                        "                </cac:RegistrationAddress>\n" +
                                        "            </cac:PartyLegalEntity>\n" +
                                        "        </cac:Party>\n" +
                                        "    </cac:AccountingSupplierParty>\n" +
                                        "    <cac:AccountingCustomerParty>\n" +
                                        "        <cac:Party>\n" +
                                        "            <cac:PartyIdentification>\n" +
                                        "                <cbc:ID schemeID=\"6\" schemeAgencyName=\"PE:SUNAT\" schemeName=\"Documento de Identidad\" schemeURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06\">12121212121</cbc:ID>\n" +
                                        "            </cac:PartyIdentification>\n" +
                                        "            <cac:PartyLegalEntity>\n" +
                                        "                <cbc:RegistrationName><![CDATA[Carlos Feria]]></cbc:RegistrationName>\n" +
                                        "            </cac:PartyLegalEntity>\n" +
                                        "        </cac:Party>\n" +
                                        "    </cac:AccountingCustomerParty>\n" +
                                        "    <cac:PaymentTerms>\n" +
                                        "        <cbc:ID>FormaPago</cbc:ID>\n" +
                                        "        <cbc:PaymentMeansID>Contado</cbc:PaymentMeansID>\n" +
                                        "    </cac:PaymentTerms>\n" +
                                        "    <cac:TaxTotal>\n" +
                                        "        <cbc:TaxAmount currencyID=\"PEN\">200.00</cbc:TaxAmount>\n" +
                                        "        <cac:TaxSubtotal>\n" +
                                        "            <cbc:TaxableAmount currencyID=\"PEN\">1000.00</cbc:TaxableAmount>\n" +
                                        "            <cbc:TaxAmount currencyID=\"PEN\">200.00</cbc:TaxAmount>\n" +
                                        "            <cac:TaxCategory>\n" +
                                        "                <cbc:ID schemeAgencyName=\"United Nations Economic Commission for Europe\" schemeID=\"UN/ECE 5305\" schemeName=\"Tax Category Identifie\">S</cbc:ID>\n" +
                                        "                <cac:TaxScheme>\n" +
                                        "                    <cbc:ID schemeAgencyName=\"PE:SUNAT\" schemeID=\"UN/ECE 5153\" schemeName=\"Codigo de tributos\">1000</cbc:ID>\n" +
                                        "                    <cbc:Name>IGV</cbc:Name>\n" +
                                        "                    <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n" +
                                        "                </cac:TaxScheme>\n" +
                                        "            </cac:TaxCategory>\n" +
                                        "        </cac:TaxSubtotal>\n" +
                                        "    </cac:TaxTotal>\n" +
                                        "    <cac:LegalMonetaryTotal>\n" +
                                        "        <cbc:LineExtensionAmount currencyID=\"PEN\">1000.00</cbc:LineExtensionAmount>\n" +
                                        "        <cbc:TaxInclusiveAmount currencyID=\"PEN\">1200.00</cbc:TaxInclusiveAmount>\n" +
                                        "        <cbc:PrepaidAmount currencyID=\"PEN\">0</cbc:PrepaidAmount>\n" +
                                        "        <cbc:PayableAmount currencyID=\"PEN\">1200.00</cbc:PayableAmount>\n" +
                                        "    </cac:LegalMonetaryTotal>\n" +
                                        "    <cac:InvoiceLine>\n" +
                                        "        <cbc:ID>1</cbc:ID>\n" +
                                        "        <cbc:InvoicedQuantity unitCode=\"NIU\" unitCodeListAgencyName=\"United Nations Economic Commission for Europe\" unitCodeListID=\"UN/ECE rec 20\">10</cbc:InvoicedQuantity>\n" +
                                        "        <cbc:LineExtensionAmount currencyID=\"PEN\">1000.00</cbc:LineExtensionAmount>\n" +
                                        "        <cac:PricingReference>\n" +
                                        "            <cac:AlternativeConditionPrice>\n" +
                                        "                <cbc:PriceAmount currencyID=\"PEN\">120.00</cbc:PriceAmount>\n" +
                                        "                <cbc:PriceTypeCode listAgencyName=\"PE:SUNAT\" listName=\"Tipo de Precio\" listURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo16\">01</cbc:PriceTypeCode>\n" +
                                        "            </cac:AlternativeConditionPrice>\n" +
                                        "        </cac:PricingReference>\n" +
                                        "        <cac:TaxTotal>\n" +
                                        "            <cbc:TaxAmount currencyID=\"PEN\">200.00</cbc:TaxAmount>\n" +
                                        "            <cac:TaxSubtotal>\n" +
                                        "                <cbc:TaxableAmount currencyID=\"PEN\">1000.00</cbc:TaxableAmount>\n" +
                                        "                <cbc:TaxAmount currencyID=\"PEN\">200.00</cbc:TaxAmount>\n" +
                                        "                <cac:TaxCategory>\n" +
                                        "                    <cbc:ID schemeAgencyName=\"United Nations Economic Commission for Europe\" schemeID=\"UN/ECE 5305\" schemeName=\"Tax Category Identifier\">S</cbc:ID>\n" +
                                        "                    <cbc:Percent>20.00</cbc:Percent>\n" +
                                        "                    <cbc:TaxExemptionReasonCode listAgencyName=\"PE:SUNAT\" listName=\"Afectacion del IGV\" listURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo07\">10</cbc:TaxExemptionReasonCode>\n" +
                                        "                    <cac:TaxScheme>\n" +
                                        "                        <cbc:ID schemeAgencyName=\"PE:SUNAT\" schemeID=\"UN/ECE 5153\" schemeName=\"Codigo de tributos\">1000</cbc:ID>\n" +
                                        "                        <cbc:Name>IGV</cbc:Name>\n" +
                                        "                        <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n" +
                                        "                    </cac:TaxScheme>\n" +
                                        "                </cac:TaxCategory>\n" +
                                        "            </cac:TaxSubtotal>\n" +
                                        "        </cac:TaxTotal>\n" +
                                        "        <cac:Item>\n" +
                                        "            <cbc:Description><![CDATA[Item1]]></cbc:Description>\n" +
                                        "        </cac:Item>\n" +
                                        "        <cac:Price>\n" +
                                        "            <cbc:PriceAmount currencyID=\"PEN\">100.00</cbc:PriceAmount>\n" +
                                        "        </cac:Price>\n" +
                                        "    </cac:InvoiceLine>\n" +
                                        "</Invoice>\n"
                        )
                );
    }

    @Test
    public void testCreditNote() {
        given()
                .when()
                .get("/quarkus-xbuilder/credit-note")
                .then()
                .statusCode(200)
                .body(
                        is(
                                "<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n" +
                                        "<CreditNote xmlns=\"urn:oasis:names:specification:ubl:schema:xsd:CreditNote-2\"\n" +
                                        "         xmlns:cac=\"urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2\"\n" +
                                        "         xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\"\n" +
                                        "         xmlns:ccts=\"urn:un:unece:uncefact:documentation:2\"\n" +
                                        "         xmlns:cec=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\"\n" +
                                        "         xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\"\n" +
                                        "         xmlns:ext=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\"\n" +
                                        "         xmlns:qdt=\"urn:oasis:names:specification:ubl:schema:xsd:QualifiedDatatypes-2\"\n" +
                                        "         xmlns:sac=\"urn:sunat:names:specification:ubl:peru:schema:xsd:SunatAggregateComponents-1\"\n" +
                                        "         xmlns:udt=\"urn:un:unece:uncefact:data:specification:UnqualifiedDataTypesSchemaModule:2\"\n" +
                                        "         xmlns:xs=\"http://www.w3.org/2001/XMLSchema\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n" +
                                        ">\n" +
                                        "    <ext:UBLExtensions>\n" +
                                        "        <ext:UBLExtension>\n" +
                                        "            <ext:ExtensionContent/>\n" +
                                        "        </ext:UBLExtension>\n" +
                                        "    </ext:UBLExtensions>\n" +
                                        "    <cbc:UBLVersionID>2.1</cbc:UBLVersionID>\n" +
                                        "    <cbc:CustomizationID>2.0</cbc:CustomizationID>\n" +
                                        "    <cbc:ID>FC01-1</cbc:ID>\n" +
                                        "    <cbc:IssueDate>2022-01-25</cbc:IssueDate>\n" +
                                        "    <cbc:DocumentCurrencyCode listID=\"ISO 4217 Alpha\" listAgencyName=\"United Nations Economic Commission for Europe\" listName=\"Currency\">PEN</cbc:DocumentCurrencyCode>\n" +
                                        "    <cac:DiscrepancyResponse>\n" +
                                        "        <cbc:ReferenceID>F001-1</cbc:ReferenceID>\n" +
                                        "        <cbc:ResponseCode>01</cbc:ResponseCode>\n" +
                                        "        <cbc:Description><![CDATA[mi sustento]]></cbc:Description>\n" +
                                        "    </cac:DiscrepancyResponse>\n" +
                                        "    <cac:BillingReference>\n" +
                                        "        <cac:InvoiceDocumentReference>\n" +
                                        "            <cbc:ID>F001-1</cbc:ID>\n" +
                                        "            <cbc:DocumentTypeCode listAgencyName=\"PE:SUNAT\" listName=\"Tipo de Documento\" listURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01\">01</cbc:DocumentTypeCode>\n" +
                                        "        </cac:InvoiceDocumentReference>\n" +
                                        "    </cac:BillingReference>\n" +
                                        "    <cac:Signature>\n" +
                                        "        <cbc:ID>12345678912</cbc:ID>\n" +
                                        "        <cac:SignatoryParty>\n" +
                                        "            <cac:PartyIdentification>\n" +
                                        "                <cbc:ID>12345678912</cbc:ID>\n" +
                                        "            </cac:PartyIdentification>\n" +
                                        "            <cac:PartyName>\n" +
                                        "                <cbc:Name><![CDATA[Softgreen S.A.C.]]></cbc:Name>\n" +
                                        "            </cac:PartyName>\n" +
                                        "        </cac:SignatoryParty>\n" +
                                        "        <cac:DigitalSignatureAttachment>\n" +
                                        "            <cac:ExternalReference>\n" +
                                        "                <cbc:URI>#PROJECT-OPENUBL-SIGN</cbc:URI>\n" +
                                        "            </cac:ExternalReference>\n" +
                                        "        </cac:DigitalSignatureAttachment>\n" +
                                        "    </cac:Signature>\n" +
                                        "    <cac:AccountingSupplierParty>\n" +
                                        "        <cac:Party>\n" +
                                        "            <cac:PartyIdentification>\n" +
                                        "                <cbc:ID schemeID=\"6\" schemeAgencyName=\"PE:SUNAT\" schemeName=\"Documento de Identidad\" schemeURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06\">12345678912</cbc:ID>\n" +
                                        "            </cac:PartyIdentification>\n" +
                                        "            <cac:PartyLegalEntity>\n" +
                                        "                <cbc:RegistrationName><![CDATA[Softgreen S.A.C.]]></cbc:RegistrationName>\n" +
                                        "                <cac:RegistrationAddress>\n" +
                                        "                    <cbc:AddressTypeCode>0000</cbc:AddressTypeCode>\n" +
                                        "                </cac:RegistrationAddress>\n" +
                                        "            </cac:PartyLegalEntity>\n" +
                                        "        </cac:Party>\n" +
                                        "    </cac:AccountingSupplierParty>\n" +
                                        "    <cac:AccountingCustomerParty>\n" +
                                        "        <cac:Party>\n" +
                                        "            <cac:PartyIdentification>\n" +
                                        "                <cbc:ID schemeID=\"6\" schemeAgencyName=\"PE:SUNAT\" schemeName=\"Documento de Identidad\" schemeURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06\">12121212121</cbc:ID>\n" +
                                        "            </cac:PartyIdentification>\n" +
                                        "            <cac:PartyLegalEntity>\n" +
                                        "                <cbc:RegistrationName><![CDATA[Carlos Feria]]></cbc:RegistrationName>\n" +
                                        "            </cac:PartyLegalEntity>\n" +
                                        "        </cac:Party>\n" +
                                        "    </cac:AccountingCustomerParty>\n" +
                                        "    <cac:TaxTotal>\n" +
                                        "        <cbc:TaxAmount currencyID=\"PEN\">200.00</cbc:TaxAmount>\n" +
                                        "        <cac:TaxSubtotal>\n" +
                                        "            <cbc:TaxableAmount currencyID=\"PEN\">1000.00</cbc:TaxableAmount>\n" +
                                        "            <cbc:TaxAmount currencyID=\"PEN\">200.00</cbc:TaxAmount>\n" +
                                        "            <cac:TaxCategory>\n" +
                                        "                <cbc:ID schemeAgencyName=\"United Nations Economic Commission for Europe\" schemeID=\"UN/ECE 5305\" schemeName=\"Tax Category Identifie\">S</cbc:ID>\n" +
                                        "                <cac:TaxScheme>\n" +
                                        "                    <cbc:ID schemeAgencyName=\"PE:SUNAT\" schemeID=\"UN/ECE 5153\" schemeName=\"Codigo de tributos\">1000</cbc:ID>\n" +
                                        "                    <cbc:Name>IGV</cbc:Name>\n" +
                                        "                    <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n" +
                                        "                </cac:TaxScheme>\n" +
                                        "            </cac:TaxCategory>\n" +
                                        "        </cac:TaxSubtotal>\n" +
                                        "    </cac:TaxTotal>\n" +
                                        "    <cac:LegalMonetaryTotal>\n" +
                                        "        <cbc:LineExtensionAmount currencyID=\"PEN\">1000.00</cbc:LineExtensionAmount>\n" +
                                        "        <cbc:TaxInclusiveAmount currencyID=\"PEN\">1200.00</cbc:TaxInclusiveAmount>\n" +
                                        "        <cbc:PayableAmount currencyID=\"PEN\">1200.00</cbc:PayableAmount>\n" +
                                        "    </cac:LegalMonetaryTotal>\n" +
                                        "    <cac:CreditNoteLine>\n" +
                                        "        <cbc:ID>1</cbc:ID>\n" +
                                        "        <cbc:CreditedQuantity unitCode=\"NIU\" unitCodeListAgencyName=\"United Nations Economic Commission for Europe\" unitCodeListID=\"UN/ECE rec 20\">10</cbc:CreditedQuantity>\n" +
                                        "        <cbc:LineExtensionAmount currencyID=\"PEN\">1000.00</cbc:LineExtensionAmount>\n" +
                                        "        <cac:PricingReference>\n" +
                                        "            <cac:AlternativeConditionPrice>\n" +
                                        "                <cbc:PriceAmount currencyID=\"PEN\">120.00</cbc:PriceAmount>\n" +
                                        "                <cbc:PriceTypeCode listAgencyName=\"PE:SUNAT\" listName=\"Tipo de Precio\" listURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo16\">01</cbc:PriceTypeCode>\n" +
                                        "            </cac:AlternativeConditionPrice>\n" +
                                        "        </cac:PricingReference>\n" +
                                        "        <cac:TaxTotal>\n" +
                                        "            <cbc:TaxAmount currencyID=\"PEN\">200.00</cbc:TaxAmount>\n" +
                                        "            <cac:TaxSubtotal>\n" +
                                        "                <cbc:TaxableAmount currencyID=\"PEN\">1000.00</cbc:TaxableAmount>\n" +
                                        "                <cbc:TaxAmount currencyID=\"PEN\">200.00</cbc:TaxAmount>\n" +
                                        "                <cac:TaxCategory>\n" +
                                        "                    <cbc:ID schemeAgencyName=\"United Nations Economic Commission for Europe\" schemeID=\"UN/ECE 5305\" schemeName=\"Tax Category Identifier\">S</cbc:ID>\n" +
                                        "                    <cbc:Percent>20.00</cbc:Percent>\n" +
                                        "                    <cbc:TaxExemptionReasonCode listAgencyName=\"PE:SUNAT\" listName=\"Afectacion del IGV\" listURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo07\">10</cbc:TaxExemptionReasonCode>\n" +
                                        "                    <cac:TaxScheme>\n" +
                                        "                        <cbc:ID schemeAgencyName=\"PE:SUNAT\" schemeID=\"UN/ECE 5153\" schemeName=\"Codigo de tributos\">1000</cbc:ID>\n" +
                                        "                        <cbc:Name>IGV</cbc:Name>\n" +
                                        "                        <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n" +
                                        "                    </cac:TaxScheme>\n" +
                                        "                </cac:TaxCategory>\n" +
                                        "            </cac:TaxSubtotal>\n" +
                                        "        </cac:TaxTotal>\n" +
                                        "        <cac:Item>\n" +
                                        "            <cbc:Description><![CDATA[Item1]]></cbc:Description>\n" +
                                        "        </cac:Item>\n" +
                                        "        <cac:Price>\n" +
                                        "            <cbc:PriceAmount currencyID=\"PEN\">100.00</cbc:PriceAmount>\n" +
                                        "        </cac:Price>\n" +
                                        "    </cac:CreditNoteLine>\n" +
                                        "</CreditNote>\n"
                        )
                );
    }

    @Test
    public void testDebitNote() {
        given()
                .when()
                .get("/quarkus-xbuilder/debit-note")
                .then()
                .statusCode(200)
                .body(
                        is(
                                "<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n" +
                                        "<DebitNote xmlns=\"urn:oasis:names:specification:ubl:schema:xsd:DebitNote-2\"\n" +
                                        "         xmlns:cac=\"urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2\"\n" +
                                        "         xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\"\n" +
                                        "         xmlns:ccts=\"urn:un:unece:uncefact:documentation:2\"\n" +
                                        "         xmlns:cec=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\"\n" +
                                        "         xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\"\n" +
                                        "         xmlns:ext=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\"\n" +
                                        "         xmlns:qdt=\"urn:oasis:names:specification:ubl:schema:xsd:QualifiedDatatypes-2\"\n" +
                                        "         xmlns:sac=\"urn:sunat:names:specification:ubl:peru:schema:xsd:SunatAggregateComponents-1\"\n" +
                                        "         xmlns:udt=\"urn:un:unece:uncefact:data:specification:UnqualifiedDataTypesSchemaModule:2\"\n" +
                                        "         xmlns:xs=\"http://www.w3.org/2001/XMLSchema\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n" +
                                        ">\n" +
                                        "    <ext:UBLExtensions>\n" +
                                        "        <ext:UBLExtension>\n" +
                                        "            <ext:ExtensionContent/>\n" +
                                        "        </ext:UBLExtension>\n" +
                                        "    </ext:UBLExtensions>\n" +
                                        "    <cbc:UBLVersionID>2.1</cbc:UBLVersionID>\n" +
                                        "    <cbc:CustomizationID>2.0</cbc:CustomizationID>\n" +
                                        "    <cbc:ID>FD01-1</cbc:ID>\n" +
                                        "    <cbc:IssueDate>2022-01-25</cbc:IssueDate>\n" +
                                        "    <cbc:DocumentCurrencyCode listID=\"ISO 4217 Alpha\" listAgencyName=\"United Nations Economic Commission for Europe\" listName=\"Currency\">PEN</cbc:DocumentCurrencyCode>\n" +
                                        "    <cac:DiscrepancyResponse>\n" +
                                        "        <cbc:ReferenceID>F001-1</cbc:ReferenceID>\n" +
                                        "        <cbc:ResponseCode>01</cbc:ResponseCode>\n" +
                                        "        <cbc:Description><![CDATA[mi sustento]]></cbc:Description>\n" +
                                        "    </cac:DiscrepancyResponse>\n" +
                                        "    <cac:BillingReference>\n" +
                                        "        <cac:InvoiceDocumentReference>\n" +
                                        "            <cbc:ID>F001-1</cbc:ID>\n" +
                                        "            <cbc:DocumentTypeCode listAgencyName=\"PE:SUNAT\" listName=\"Tipo de Documento\" listURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo01\">01</cbc:DocumentTypeCode>\n" +
                                        "        </cac:InvoiceDocumentReference>\n" +
                                        "    </cac:BillingReference>\n" +
                                        "    <cac:Signature>\n" +
                                        "        <cbc:ID>12345678912</cbc:ID>\n" +
                                        "        <cac:SignatoryParty>\n" +
                                        "            <cac:PartyIdentification>\n" +
                                        "                <cbc:ID>12345678912</cbc:ID>\n" +
                                        "            </cac:PartyIdentification>\n" +
                                        "            <cac:PartyName>\n" +
                                        "                <cbc:Name><![CDATA[Softgreen S.A.C.]]></cbc:Name>\n" +
                                        "            </cac:PartyName>\n" +
                                        "        </cac:SignatoryParty>\n" +
                                        "        <cac:DigitalSignatureAttachment>\n" +
                                        "            <cac:ExternalReference>\n" +
                                        "                <cbc:URI>#PROJECT-OPENUBL-SIGN</cbc:URI>\n" +
                                        "            </cac:ExternalReference>\n" +
                                        "        </cac:DigitalSignatureAttachment>\n" +
                                        "    </cac:Signature>\n" +
                                        "    <cac:AccountingSupplierParty>\n" +
                                        "        <cac:Party>\n" +
                                        "            <cac:PartyIdentification>\n" +
                                        "                <cbc:ID schemeID=\"6\" schemeAgencyName=\"PE:SUNAT\" schemeName=\"Documento de Identidad\" schemeURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06\">12345678912</cbc:ID>\n" +
                                        "            </cac:PartyIdentification>\n" +
                                        "            <cac:PartyLegalEntity>\n" +
                                        "                <cbc:RegistrationName><![CDATA[Softgreen S.A.C.]]></cbc:RegistrationName>\n" +
                                        "                <cac:RegistrationAddress>\n" +
                                        "                    <cbc:AddressTypeCode>0000</cbc:AddressTypeCode>\n" +
                                        "                </cac:RegistrationAddress>\n" +
                                        "            </cac:PartyLegalEntity>\n" +
                                        "        </cac:Party>\n" +
                                        "    </cac:AccountingSupplierParty>\n" +
                                        "    <cac:AccountingCustomerParty>\n" +
                                        "        <cac:Party>\n" +
                                        "            <cac:PartyIdentification>\n" +
                                        "                <cbc:ID schemeID=\"6\" schemeAgencyName=\"PE:SUNAT\" schemeName=\"Documento de Identidad\" schemeURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo06\">12121212121</cbc:ID>\n" +
                                        "            </cac:PartyIdentification>\n" +
                                        "            <cac:PartyLegalEntity>\n" +
                                        "                <cbc:RegistrationName><![CDATA[Carlos Feria]]></cbc:RegistrationName>\n" +
                                        "            </cac:PartyLegalEntity>\n" +
                                        "        </cac:Party>\n" +
                                        "    </cac:AccountingCustomerParty>\n" +
                                        "    <cac:TaxTotal>\n" +
                                        "        <cbc:TaxAmount currencyID=\"PEN\">200.00</cbc:TaxAmount>\n" +
                                        "        <cac:TaxSubtotal>\n" +
                                        "            <cbc:TaxableAmount currencyID=\"PEN\">1000.00</cbc:TaxableAmount>\n" +
                                        "            <cbc:TaxAmount currencyID=\"PEN\">200.00</cbc:TaxAmount>\n" +
                                        "            <cac:TaxCategory>\n" +
                                        "                <cbc:ID schemeAgencyName=\"United Nations Economic Commission for Europe\" schemeID=\"UN/ECE 5305\" schemeName=\"Tax Category Identifie\">S</cbc:ID>\n" +
                                        "                <cac:TaxScheme>\n" +
                                        "                    <cbc:ID schemeAgencyName=\"PE:SUNAT\" schemeID=\"UN/ECE 5153\" schemeName=\"Codigo de tributos\">1000</cbc:ID>\n" +
                                        "                    <cbc:Name>IGV</cbc:Name>\n" +
                                        "                    <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n" +
                                        "                </cac:TaxScheme>\n" +
                                        "            </cac:TaxCategory>\n" +
                                        "        </cac:TaxSubtotal>\n" +
                                        "    </cac:TaxTotal>\n" +
                                        "    <cac:RequestedMonetaryTotal>\n" +
                                        "        <cbc:LineExtensionAmount currencyID=\"PEN\">1000.00</cbc:LineExtensionAmount>\n" +
                                        "        <cbc:TaxInclusiveAmount currencyID=\"PEN\">1200.00</cbc:TaxInclusiveAmount>\n" +
                                        "        <cbc:PayableAmount currencyID=\"PEN\">1200.00</cbc:PayableAmount>\n" +
                                        "    </cac:RequestedMonetaryTotal>\n" +
                                        "    <cac:DebitNoteLine>\n" +
                                        "        <cbc:ID>1</cbc:ID>\n" +
                                        "        <cbc:DebitedQuantity unitCode=\"NIU\" unitCodeListAgencyName=\"United Nations Economic Commission for Europe\" unitCodeListID=\"UN/ECE rec 20\">10</cbc:DebitedQuantity>\n" +
                                        "        <cbc:LineExtensionAmount currencyID=\"PEN\">1000.00</cbc:LineExtensionAmount>\n" +
                                        "        <cac:PricingReference>\n" +
                                        "            <cac:AlternativeConditionPrice>\n" +
                                        "                <cbc:PriceAmount currencyID=\"PEN\">120.00</cbc:PriceAmount>\n" +
                                        "                <cbc:PriceTypeCode listAgencyName=\"PE:SUNAT\" listName=\"Tipo de Precio\" listURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo16\">01</cbc:PriceTypeCode>\n" +
                                        "            </cac:AlternativeConditionPrice>\n" +
                                        "        </cac:PricingReference>\n" +
                                        "        <cac:TaxTotal>\n" +
                                        "            <cbc:TaxAmount currencyID=\"PEN\">200.00</cbc:TaxAmount>\n" +
                                        "            <cac:TaxSubtotal>\n" +
                                        "                <cbc:TaxableAmount currencyID=\"PEN\">1000.00</cbc:TaxableAmount>\n" +
                                        "                <cbc:TaxAmount currencyID=\"PEN\">200.00</cbc:TaxAmount>\n" +
                                        "                <cac:TaxCategory>\n" +
                                        "                    <cbc:ID schemeAgencyName=\"United Nations Economic Commission for Europe\" schemeID=\"UN/ECE 5305\" schemeName=\"Tax Category Identifier\">S</cbc:ID>\n" +
                                        "                    <cbc:Percent>20.00</cbc:Percent>\n" +
                                        "                    <cbc:TaxExemptionReasonCode listAgencyName=\"PE:SUNAT\" listName=\"Afectacion del IGV\" listURI=\"urn:pe:gob:sunat:cpe:see:gem:catalogos:catalogo07\">10</cbc:TaxExemptionReasonCode>\n" +
                                        "                    <cac:TaxScheme>\n" +
                                        "                        <cbc:ID schemeAgencyName=\"PE:SUNAT\" schemeID=\"UN/ECE 5153\" schemeName=\"Codigo de tributos\">1000</cbc:ID>\n" +
                                        "                        <cbc:Name>IGV</cbc:Name>\n" +
                                        "                        <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n" +
                                        "                    </cac:TaxScheme>\n" +
                                        "                </cac:TaxCategory>\n" +
                                        "            </cac:TaxSubtotal>\n" +
                                        "        </cac:TaxTotal>\n" +
                                        "        <cac:Item>\n" +
                                        "            <cbc:Description><![CDATA[Item1]]></cbc:Description>\n" +
                                        "        </cac:Item>\n" +
                                        "        <cac:Price>\n" +
                                        "            <cbc:PriceAmount currencyID=\"PEN\">100.00</cbc:PriceAmount>\n" +
                                        "        </cac:Price>\n" +
                                        "    </cac:DebitNoteLine>\n" +
                                        "</DebitNote>\n"
                        )
                );
    }

    @Test
    public void testVoidedDocuments() {
        given()
                .when().get("/quarkus-xbuilder/voided-documents")
                .then()
                .statusCode(200)
                .body(is("<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n" +
                        "<VoidedDocuments xmlns=\"urn:sunat:names:specification:ubl:peru:schema:xsd:VoidedDocuments-1\"\n" +
                        "                 xmlns:cac=\"urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2\"\n" +
                        "                 xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\"\n" +
                        "                 xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\"\n" +
                        "                 xmlns:ext=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\"\n" +
                        "                 xmlns:sac=\"urn:sunat:names:specification:ubl:peru:schema:xsd:SunatAggregateComponents-1\"\n" +
                        "                 xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\">\n" +
                        "    <ext:UBLExtensions>\n" +
                        "        <ext:UBLExtension>\n" +
                        "            <ext:ExtensionContent/>\n" +
                        "        </ext:UBLExtension>\n" +
                        "    </ext:UBLExtensions>\n" +
                        "    <cbc:UBLVersionID>2.0</cbc:UBLVersionID>\n" +
                        "    <cbc:CustomizationID>1.0</cbc:CustomizationID>\n" +
                        "    <cbc:ID>RA-20220131-1</cbc:ID>\n" +
                        "    <cbc:ReferenceDate>2022-01-29</cbc:ReferenceDate>\n" +
                        "    <cbc:IssueDate>2022-01-31</cbc:IssueDate>\n" +
                        "    <cac:Signature>\n" +
                        "        <cbc:ID>12345678912</cbc:ID>\n" +
                        "        <cac:SignatoryParty>\n" +
                        "            <cac:PartyIdentification>\n" +
                        "                <cbc:ID>12345678912</cbc:ID>\n" +
                        "            </cac:PartyIdentification>\n" +
                        "            <cac:PartyName>\n" +
                        "                <cbc:Name><![CDATA[Softgreen S.A.C.]]></cbc:Name>\n" +
                        "            </cac:PartyName>\n" +
                        "        </cac:SignatoryParty>\n" +
                        "        <cac:DigitalSignatureAttachment>\n" +
                        "            <cac:ExternalReference>\n" +
                        "                <cbc:URI>#PROJECT-OPENUBL-SIGN</cbc:URI>\n" +
                        "            </cac:ExternalReference>\n" +
                        "        </cac:DigitalSignatureAttachment>\n" +
                        "    </cac:Signature>\n" +
                        "    <cac:AccountingSupplierParty>\n" +
                        "        <cbc:CustomerAssignedAccountID>12345678912</cbc:CustomerAssignedAccountID>\n" +
                        "        <cbc:AdditionalAccountID>6</cbc:AdditionalAccountID>\n" +
                        "        <cac:Party>\n" +
                        "            <cac:PartyLegalEntity>\n" +
                        "                <cbc:RegistrationName><![CDATA[Softgreen S.A.C.]]></cbc:RegistrationName>\n" +
                        "            </cac:PartyLegalEntity>\n" +
                        "        </cac:Party>\n" +
                        "    </cac:AccountingSupplierParty>\n" +
                        "    <sac:VoidedDocumentsLine>\n" +
                        "        <cbc:LineID>1</cbc:LineID>\n" +
                        "        <cbc:DocumentTypeCode>01</cbc:DocumentTypeCode>\n" +
                        "        <sac:DocumentSerialID>F001</sac:DocumentSerialID>\n" +
                        "        <sac:DocumentNumberID>1</sac:DocumentNumberID>\n" +
                        "        <sac:VoidReasonDescription>Mi sustento1</sac:VoidReasonDescription>\n" +
                        "    </sac:VoidedDocumentsLine>\n" +
                        "    <sac:VoidedDocumentsLine>\n" +
                        "        <cbc:LineID>2</cbc:LineID>\n" +
                        "        <cbc:DocumentTypeCode>01</cbc:DocumentTypeCode>\n" +
                        "        <sac:DocumentSerialID>F001</sac:DocumentSerialID>\n" +
                        "        <sac:DocumentNumberID>2</sac:DocumentNumberID>\n" +
                        "        <sac:VoidReasonDescription>Mi sustento2</sac:VoidReasonDescription>\n" +
                        "    </sac:VoidedDocumentsLine>\n" +
                        "</VoidedDocuments>\n"));
    }

    @Test
    public void testSummaryDocuments() {
        given()
                .when().get("/quarkus-xbuilder/summary-documents")
                .then()
                .statusCode(200)
                .body(is("<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n" +
                        "<SummaryDocuments xmlns=\"urn:sunat:names:specification:ubl:peru:schema:xsd:SummaryDocuments-1\"\n" +
                        "                  xmlns:cac=\"urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2\"\n" +
                        "                  xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\"\n" +
                        "                  xmlns:ccts=\"urn:un:unece:uncefact:documentation:2\" xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\"\n" +
                        "                  xmlns:ext=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\"\n" +
                        "                  xmlns:ns11=\"urn:sunat:names:specification:ubl:peru:schema:xsd:Perception-1\"\n" +
                        "                  xmlns:qdt=\"urn:oasis:names:specification:ubl:schema:xsd:QualifiedDatatypes-2\"\n" +
                        "                  xmlns:sac=\"urn:sunat:names:specification:ubl:peru:schema:xsd:SunatAggregateComponents-1\"\n" +
                        "                  xmlns:udt=\"urn:un:unece:uncefact:data:specification:UnqualifiedDataTypesSchemaModule:2\"\n" +
                        "                  xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\">\n" +
                        "    <ext:UBLExtensions>\n" +
                        "        <ext:UBLExtension>\n" +
                        "            <ext:ExtensionContent />\n" +
                        "        </ext:UBLExtension>\n" +
                        "    </ext:UBLExtensions>\n" +
                        "    <cbc:UBLVersionID>2.0</cbc:UBLVersionID>\n" +
                        "    <cbc:CustomizationID>1.1</cbc:CustomizationID>\n" +
                        "    <cbc:ID>RC-20220131-1</cbc:ID>\n" +
                        "    <cbc:ReferenceDate>2022-01-29</cbc:ReferenceDate>\n" +
                        "    <cbc:IssueDate>2022-01-31</cbc:IssueDate>\n" +
                        "    <cac:Signature>\n" +
                        "        <cbc:ID>12345678912</cbc:ID>\n" +
                        "        <cac:SignatoryParty>\n" +
                        "            <cac:PartyIdentification>\n" +
                        "                <cbc:ID>12345678912</cbc:ID>\n" +
                        "            </cac:PartyIdentification>\n" +
                        "            <cac:PartyName>\n" +
                        "                <cbc:Name><![CDATA[Softgreen S.A.C.]]></cbc:Name>\n" +
                        "            </cac:PartyName>\n" +
                        "        </cac:SignatoryParty>\n" +
                        "        <cac:DigitalSignatureAttachment>\n" +
                        "            <cac:ExternalReference>\n" +
                        "                <cbc:URI>#PROJECT-OPENUBL-SIGN</cbc:URI>\n" +
                        "            </cac:ExternalReference>\n" +
                        "        </cac:DigitalSignatureAttachment>\n" +
                        "    </cac:Signature>\n" +
                        "    <cac:AccountingSupplierParty>\n" +
                        "        <cbc:CustomerAssignedAccountID>12345678912</cbc:CustomerAssignedAccountID>\n" +
                        "        <cbc:AdditionalAccountID>6</cbc:AdditionalAccountID>\n" +
                        "        <cac:Party>\n" +
                        "            <cac:PartyLegalEntity>\n" +
                        "                <cbc:RegistrationName><![CDATA[Softgreen S.A.C.]]></cbc:RegistrationName>\n" +
                        "            </cac:PartyLegalEntity>\n" +
                        "        </cac:Party>\n" +
                        "    </cac:AccountingSupplierParty>\n" +
                        "    <sac:SummaryDocumentsLine>\n" +
                        "        <cbc:LineID>1</cbc:LineID>\n" +
                        "        <cbc:DocumentTypeCode>03</cbc:DocumentTypeCode>\n" +
                        "        <cbc:ID>B001-1</cbc:ID>\n" +
                        "        <cac:AccountingCustomerParty>\n" +
                        "            <cbc:CustomerAssignedAccountID>12345678</cbc:CustomerAssignedAccountID>\n" +
                        "            <cbc:AdditionalAccountID>1</cbc:AdditionalAccountID>\n" +
                        "        </cac:AccountingCustomerParty>\n" +
                        "        <cac:Status>\n" +
                        "            <cbc:ConditionCode>1</cbc:ConditionCode>\n" +
                        "        </cac:Status>\n" +
                        "        <sac:TotalAmount currencyID=\"PEN\">120</sac:TotalAmount>\n" +
                        "        <sac:BillingPayment>\n" +
                        "            <cbc:PaidAmount currencyID=\"PEN\">120</cbc:PaidAmount>\n" +
                        "            <cbc:InstructionID>01</cbc:InstructionID>\n" +
                        "        </sac:BillingPayment>\n" +
                        "        <cac:TaxTotal>\n" +
                        "            <cbc:TaxAmount currencyID=\"PEN\">18</cbc:TaxAmount>\n" +
                        "            <cac:TaxSubtotal>\n" +
                        "                <cbc:TaxAmount currencyID=\"PEN\">18</cbc:TaxAmount>\n" +
                        "                <cac:TaxCategory>\n" +
                        "                    <cac:TaxScheme>\n" +
                        "                        <cbc:ID>1000</cbc:ID>\n" +
                        "                        <cbc:Name>IGV</cbc:Name>\n" +
                        "                        <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n" +
                        "                    </cac:TaxScheme>\n" +
                        "                </cac:TaxCategory>\n" +
                        "            </cac:TaxSubtotal>\n" +
                        "        </cac:TaxTotal>\n" +
                        "        <cac:TaxTotal>\n" +
                        "            <cbc:TaxAmount currencyID=\"PEN\">2</cbc:TaxAmount>\n" +
                        "            <cac:TaxSubtotal>\n" +
                        "                <cbc:TaxAmount currencyID=\"PEN\">2</cbc:TaxAmount>\n" +
                        "                <cac:TaxCategory>\n" +
                        "                    <cac:TaxScheme>\n" +
                        "                        <cbc:ID>7152</cbc:ID>\n" +
                        "                        <cbc:Name>ICBPER</cbc:Name>\n" +
                        "                        <cbc:TaxTypeCode>OTH</cbc:TaxTypeCode>\n" +
                        "                    </cac:TaxScheme>\n" +
                        "                </cac:TaxCategory>\n" +
                        "            </cac:TaxSubtotal>\n" +
                        "        </cac:TaxTotal>\n" +
                        "    </sac:SummaryDocumentsLine>\n" +
                        "    <sac:SummaryDocumentsLine>\n" +
                        "        <cbc:LineID>2</cbc:LineID>\n" +
                        "        <cbc:DocumentTypeCode>07</cbc:DocumentTypeCode>\n" +
                        "        <cbc:ID>BC02-2</cbc:ID>\n" +
                        "        <cac:AccountingCustomerParty>\n" +
                        "            <cbc:CustomerAssignedAccountID>12345678</cbc:CustomerAssignedAccountID>\n" +
                        "            <cbc:AdditionalAccountID>1</cbc:AdditionalAccountID>\n" +
                        "        </cac:AccountingCustomerParty>\n" +
                        "        <cac:BillingReference>\n" +
                        "            <cac:InvoiceDocumentReference>\n" +
                        "                <cbc:ID>B002-2</cbc:ID>\n" +
                        "                <cbc:DocumentTypeCode>03</cbc:DocumentTypeCode>\n" +
                        "            </cac:InvoiceDocumentReference>\n" +
                        "        </cac:BillingReference>\n" +
                        "        <cac:Status>\n" +
                        "            <cbc:ConditionCode>1</cbc:ConditionCode>\n" +
                        "        </cac:Status>\n" +
                        "        <sac:TotalAmount currencyID=\"PEN\">118</sac:TotalAmount>\n" +
                        "        <sac:BillingPayment>\n" +
                        "            <cbc:PaidAmount currencyID=\"PEN\">118</cbc:PaidAmount>\n" +
                        "            <cbc:InstructionID>01</cbc:InstructionID>\n" +
                        "        </sac:BillingPayment>\n" +
                        "        <cac:TaxTotal>\n" +
                        "            <cbc:TaxAmount currencyID=\"PEN\">18</cbc:TaxAmount>\n" +
                        "            <cac:TaxSubtotal>\n" +
                        "                <cbc:TaxAmount currencyID=\"PEN\">18</cbc:TaxAmount>\n" +
                        "                <cac:TaxCategory>\n" +
                        "                    <cac:TaxScheme>\n" +
                        "                        <cbc:ID>1000</cbc:ID>\n" +
                        "                        <cbc:Name>IGV</cbc:Name>\n" +
                        "                        <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n" +
                        "                    </cac:TaxScheme>\n" +
                        "                </cac:TaxCategory>\n" +
                        "            </cac:TaxSubtotal>\n" +
                        "        </cac:TaxTotal>\n" +
                        "    </sac:SummaryDocumentsLine>\n" +
                        "</SummaryDocuments>\n"));
    }

    @Test
    public void testPerception() {
        given()
                .when().get("/quarkus-xbuilder/perception")
                .then()
                .statusCode(200)
                .body(is("<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n" +
                        "<Perception xmlns=\"urn:sunat:names:specification:ubl:peru:schema:xsd:Perception-1\"\n" +
                        "            xmlns:cac=\"urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2\"\n" +
                        "            xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\"\n" +
                        "            xmlns:ccts=\"urn:un:unece:uncefact:documentation:2\"\n" +
                        "            xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\"\n" +
                        "            xmlns:ext=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\"\n" +
                        "            xmlns:qdt=\"urn:oasis:names:specification:ubl:schema:xsd:QualifiedDatatypes-2\"\n" +
                        "            xmlns:sac=\"urn:sunat:names:specification:ubl:peru:schema:xsd:SunatAggregateComponents-1\"\n" +
                        "            xmlns:udt=\"urn:un:unece:uncefact:data:specification:UnqualifiedDataTypesSchemaModule:2\"\n" +
                        "            xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\">\n" +
                        "    <ext:UBLExtensions>\n" +
                        "        <ext:UBLExtension>\n" +
                        "            <ext:ExtensionContent />\n" +
                        "        </ext:UBLExtension>\n" +
                        "    </ext:UBLExtensions>\n" +
                        "    <cbc:UBLVersionID>2.0</cbc:UBLVersionID>\n" +
                        "    <cbc:CustomizationID>1.0</cbc:CustomizationID>\n" +
                        "    <cac:Signature>\n" +
                        "        <cbc:ID>12345678912</cbc:ID>\n" +
                        "        <cac:SignatoryParty>\n" +
                        "            <cac:PartyIdentification>\n" +
                        "                <cbc:ID>12345678912</cbc:ID>\n" +
                        "            </cac:PartyIdentification>\n" +
                        "            <cac:PartyName>\n" +
                        "                <cbc:Name><![CDATA[Softgreen S.A.C.]]></cbc:Name>\n" +
                        "            </cac:PartyName>\n" +
                        "        </cac:SignatoryParty>\n" +
                        "        <cac:DigitalSignatureAttachment>\n" +
                        "            <cac:ExternalReference>\n" +
                        "                <cbc:URI>#PROJECT-OPENUBL-SIGN</cbc:URI>\n" +
                        "            </cac:ExternalReference>\n" +
                        "        </cac:DigitalSignatureAttachment>\n" +
                        "    </cac:Signature>\n" +
                        "    <cbc:ID>P001-1</cbc:ID>\n" +
                        "    <cbc:IssueDate>2022-01-31</cbc:IssueDate>\n" +
                        "    <cac:AgentParty>\n" +
                        "        <cac:PartyIdentification>\n" +
                        "            <cbc:ID schemeID=\"6\">12345678912</cbc:ID>\n" +
                        "        </cac:PartyIdentification>\n" +
                        "        <cac:PartyLegalEntity>\n" +
                        "            <cbc:RegistrationName><![CDATA[Softgreen S.A.C.]]></cbc:RegistrationName>\n" +
                        "        </cac:PartyLegalEntity>\n" +
                        "    </cac:AgentParty>\n" +
                        "    <cac:ReceiverParty>\n" +
                        "        <cac:PartyIdentification>\n" +
                        "            <cbc:ID schemeID=\"6\">12121212121</cbc:ID>\n" +
                        "        </cac:PartyIdentification>\n" +
                        "        <cac:PartyLegalEntity>\n" +
                        "            <cbc:RegistrationName><![CDATA[Carlos Feria]]></cbc:RegistrationName>\n" +
                        "        </cac:PartyLegalEntity>\n" +
                        "    </cac:ReceiverParty>\n" +
                        "    <sac:SUNATPerceptionSystemCode>01</sac:SUNATPerceptionSystemCode>\n" +
                        "    <sac:SUNATPerceptionPercent>2</sac:SUNATPerceptionPercent>\n" +
                        "    <cbc:TotalInvoiceAmount currencyID=\"PEN\">10</cbc:TotalInvoiceAmount>\n" +
                        "    <sac:SUNATTotalCashed currencyID=\"PEN\">210</sac:SUNATTotalCashed>\n" +
                        "    <sac:SUNATPerceptionDocumentReference>\n" +
                        "        <cbc:ID schemeID=\"01\">F001-1</cbc:ID>\n" +
                        "        <cbc:IssueDate>2022-01-31</cbc:IssueDate>\n" +
                        "        <cbc:TotalInvoiceAmount currencyID=\"PEN\">200</cbc:TotalInvoiceAmount>\n" +
                        "        <cac:Payment>\n" +
                        "            <cbc:ID>1</cbc:ID>\n" +
                        "            <cbc:PaidAmount currencyID=\"PEN\">100</cbc:PaidAmount>\n" +
                        "            <cbc:PaidDate>2022-01-31</cbc:PaidDate>\n" +
                        "        </cac:Payment>\n" +
                        "        <sac:SUNATPerceptionInformation>\n" +
                        "            <sac:SUNATPerceptionAmount currencyID=\"PEN\">10</sac:SUNATPerceptionAmount>\n" +
                        "            <sac:SUNATPerceptionDate>2022-01-31</sac:SUNATPerceptionDate>\n" +
                        "            <sac:SUNATNetTotalCashed currencyID=\"PEN\">210</sac:SUNATNetTotalCashed>\n" +
                        "        </sac:SUNATPerceptionInformation>\n" +
                        "    </sac:SUNATPerceptionDocumentReference>\n" +
                        "</Perception>\n"));
    }

    @Test
    public void testRetention() {
        given()
                .when().get("/quarkus-xbuilder/retention")
                .then()
                .statusCode(200)
                .body(is("<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n" +
                        "<Retention xmlns=\"urn:sunat:names:specification:ubl:peru:schema:xsd:Retention-1\"\n" +
                        "           xmlns:cac=\"urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2\"\n" +
                        "           xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\"\n" +
                        "           xmlns:ccts=\"urn:un:unece:uncefact:documentation:2\"\n" +
                        "           xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\"\n" +
                        "           xmlns:ext=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\"\n" +
                        "           xmlns:qdt=\"urn:oasis:names:specification:ubl:schema:xsd:QualifiedDatatypes-2\"\n" +
                        "           xmlns:sac=\"urn:sunat:names:specification:ubl:peru:schema:xsd:SunatAggregateComponents-1\"\n" +
                        "           xmlns:udt=\"urn:un:unece:uncefact:data:specification:UnqualifiedDataTypesSchemaModule:2\"\n" +
                        "           xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\">\n" +
                        "    <ext:UBLExtensions>\n" +
                        "        <ext:UBLExtension>\n" +
                        "            <ext:ExtensionContent />\n" +
                        "        </ext:UBLExtension>\n" +
                        "    </ext:UBLExtensions>\n" +
                        "    <cbc:UBLVersionID>2.0</cbc:UBLVersionID>\n" +
                        "    <cbc:CustomizationID>1.0</cbc:CustomizationID>\n" +
                        "    <cac:Signature>\n" +
                        "        <cbc:ID>12345678912</cbc:ID>\n" +
                        "        <cac:SignatoryParty>\n" +
                        "            <cac:PartyIdentification>\n" +
                        "                <cbc:ID>12345678912</cbc:ID>\n" +
                        "            </cac:PartyIdentification>\n" +
                        "            <cac:PartyName>\n" +
                        "                <cbc:Name><![CDATA[Softgreen S.A.C.]]></cbc:Name>\n" +
                        "            </cac:PartyName>\n" +
                        "        </cac:SignatoryParty>\n" +
                        "        <cac:DigitalSignatureAttachment>\n" +
                        "            <cac:ExternalReference>\n" +
                        "                <cbc:URI>#PROJECT-OPENUBL-SIGN</cbc:URI>\n" +
                        "            </cac:ExternalReference>\n" +
                        "        </cac:DigitalSignatureAttachment>\n" +
                        "    </cac:Signature>\n" +
                        "    <cbc:ID>R001-1</cbc:ID>\n" +
                        "    <cbc:IssueDate>2022-01-31</cbc:IssueDate>\n" +
                        "    <cac:AgentParty>\n" +
                        "        <cac:PartyIdentification>\n" +
                        "            <cbc:ID schemeID=\"6\">12345678912</cbc:ID>\n" +
                        "        </cac:PartyIdentification>\n" +
                        "        <cac:PartyLegalEntity>\n" +
                        "            <cbc:RegistrationName><![CDATA[Softgreen S.A.C.]]></cbc:RegistrationName>\n" +
                        "        </cac:PartyLegalEntity>\n" +
                        "    </cac:AgentParty>\n" +
                        "    <cac:ReceiverParty>\n" +
                        "        <cac:PartyIdentification>\n" +
                        "            <cbc:ID schemeID=\"6\">12121212121</cbc:ID>\n" +
                        "        </cac:PartyIdentification>\n" +
                        "        <cac:PartyLegalEntity>\n" +
                        "            <cbc:RegistrationName><![CDATA[Carlos Feria]]></cbc:RegistrationName>\n" +
                        "        </cac:PartyLegalEntity>\n" +
                        "    </cac:ReceiverParty>\n" +
                        "    <sac:SUNATRetentionSystemCode>01</sac:SUNATRetentionSystemCode>\n" +
                        "    <sac:SUNATRetentionPercent>3</sac:SUNATRetentionPercent>\n" +
                        "    <cbc:TotalInvoiceAmount currencyID=\"PEN\">10</cbc:TotalInvoiceAmount>\n" +
                        "    <sac:SUNATTotalPaid currencyID=\"PEN\">200</sac:SUNATTotalPaid>\n" +
                        "    <sac:SUNATRetentionDocumentReference>\n" +
                        "        <cbc:ID schemeID=\"01\">F001-1</cbc:ID>\n" +
                        "        <cbc:IssueDate>2022-01-31</cbc:IssueDate>\n" +
                        "        <cbc:TotalInvoiceAmount currencyID=\"PEN\">210</cbc:TotalInvoiceAmount>\n" +
                        "        <cac:Payment>\n" +
                        "            <cbc:ID>1</cbc:ID>\n" +
                        "            <cbc:PaidAmount currencyID=\"PEN\">100</cbc:PaidAmount>\n" +
                        "            <cbc:PaidDate>2022-01-31</cbc:PaidDate>\n" +
                        "        </cac:Payment>\n" +
                        "        <sac:SUNATRetentionInformation>\n" +
                        "            <sac:SUNATRetentionAmount currencyID=\"PEN\">10</sac:SUNATRetentionAmount>\n" +
                        "            <sac:SUNATRetentionDate>2022-01-31</sac:SUNATRetentionDate>\n" +
                        "            <sac:SUNATNetTotalPaid currencyID=\"PEN\">200</sac:SUNATNetTotalPaid>\n" +
                        "        </sac:SUNATRetentionInformation>\n" +
                        "    </sac:SUNATRetentionDocumentReference>\n" +
                        "</Retention>\n"));
    }
}
