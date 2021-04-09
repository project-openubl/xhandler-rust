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
package io.github.project.openubl.xmlbuilderlib.integrationtest;

import io.github.project.openubl.xmlbuilderlib.clock.SystemClock;
import io.github.project.openubl.xmlbuilderlib.config.Config;
import io.github.project.openubl.xmlbuilderlib.config.DefaultConfig;
import io.github.project.openubl.xmlbuilderlib.utils.CertificateDetails;
import io.github.project.openubl.xmlbuilderlib.utils.CertificateDetailsFactory;
import io.github.project.openubl.xmlbuilderlib.xml.XMLSigner;
import io.github.project.openubl.xmlbuilderlib.xml.XmlSignatureHelper;
import io.github.project.openubl.xmlsenderws.webservices.managers.smart.SmartBillServiceConfig;
import io.github.project.openubl.xmlsenderws.webservices.managers.smart.SmartBillServiceManager;
import io.github.project.openubl.xmlsenderws.webservices.managers.smart.SmartBillServiceModel;
import io.github.project.openubl.xmlsenderws.webservices.providers.BillServiceModel;
import io.github.project.openubl.xmlsenderws.webservices.xml.DocumentType;
import io.github.project.openubl.xmlsenderws.webservices.xml.XmlContentModel;
import org.w3c.dom.Document;
import org.xml.sax.InputSource;
import org.xml.sax.SAXException;
import org.xmlunit.builder.DiffBuilder;
import org.xmlunit.diff.Diff;

import javax.validation.ConstraintViolation;
import javax.validation.Validation;
import javax.validation.Validator;
import javax.validation.ValidatorFactory;
import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.ParserConfigurationException;
import javax.xml.transform.Transformer;
import javax.xml.transform.TransformerException;
import javax.xml.transform.TransformerFactory;
import javax.xml.transform.dom.DOMSource;
import javax.xml.transform.stream.StreamResult;
import javax.xml.xpath.XPath;
import javax.xml.xpath.XPathFactory;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.*;
import java.util.stream.Collectors;

import static org.junit.jupiter.api.Assertions.*;

public abstract class AbstractUBLTest {

    static final String SUNAT_BETA_USERNAME = "MODDATOS";
    static final String SUNAT_BETA_PASSWORD = "MODDATOS";


    static String SIGN_REFERENCE_ID = "PROJECT-OPENUBL";
    static final String KEYSTORE = "LLAMA-PE-CERTIFICADO-DEMO-10467793549.pfx";
    static final String KEYSTORE_PASSWORD = "password";
    static CertificateDetails CERTIFICATE;


    protected XPath xPath;
    protected Validator validator;


    protected Config config;
    protected TimeZone timeZone;
    protected SystemClock systemClock;

    protected String DOCUMENT_WITHOUT_2007_LEGEND = "4264 - El XML no contiene el codigo de leyenda 2007 para el tipo de operación IVAP";

    public AbstractUBLTest() throws Exception {
        xPath = XPathFactory.newInstance().newXPath();
        xPath.setNamespaceContext(SunatNamespacesSingleton.getInstance());

        ValidatorFactory factory = Validation.buildDefaultValidatorFactory();
        validator = factory.getValidator();

        SmartBillServiceConfig.getInstance()
                .withInvoiceAndNoteDeliveryURL("https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService")
                .withPerceptionAndRetentionDeliveryURL("https://e-beta.sunat.gob.pe/ol-ti-itemision-otroscpe-gem-beta/billService")
                .withDespatchAdviceDeliveryURL("https://e-beta.sunat.gob.pe/ol-ti-itemision-guia-gem-beta/billService");


        InputStream ksInputStream = Thread.currentThread().getContextClassLoader().getResourceAsStream(KEYSTORE);
        CERTIFICATE = CertificateDetailsFactory.create(ksInputStream, KEYSTORE_PASSWORD);

        config = new DefaultConfig();
        timeZone = TimeZone.getTimeZone("America/Lima");
        systemClock = new SystemClock() {
            @Override
            public TimeZone getTimeZone() {
                return timeZone;
            }

            @Override
            public Calendar getCalendarInstance() {
                Calendar calendar = Calendar.getInstance();
                calendar.setTimeZone(timeZone);
                calendar.set(2019, Calendar.DECEMBER, 24, 20, 30, 59);
                return calendar;
            }
        };
    }

    protected static void assertOutputHasNoConstraintViolations(Validator validator, Object output) {
        Set<ConstraintViolation<Object>> violations = validator.validate(output);
        assertTrue(violations.isEmpty());
    }

    public void assertSnapshot(String expected, String snapshotFile) {
        InputStream snapshotInputStream = Thread.currentThread().getContextClassLoader().getResourceAsStream(snapshotFile);
        assertNotNull(snapshotInputStream, "Could not find snapshot file " + snapshotFile);

        Diff myDiff = DiffBuilder
                .compare(snapshotInputStream)
                .withTest(expected)
                .ignoreComments()
                .ignoreWhitespace()
                .build();

        assertFalse(myDiff.hasDifferences(), expected + "\n" + myDiff.toString());
    }

    public void assertSendSunat(String xmlWithoutSignature, String... allowedNotes) throws Exception {
        String skipSunat = System.getProperty("skipSunat", "false");
        if (skipSunat != null && skipSunat.equals("false")) {
            Document signedXML = XMLSigner.signXML(xmlWithoutSignature, SIGN_REFERENCE_ID, CERTIFICATE.getX509Certificate(), CERTIFICATE.getPrivateKey());
            sendFileToSunat(signedXML, xmlWithoutSignature, allowedNotes);
        }
    }

    private void sendFileToSunat(Document document, String xmlWithoutSignature, String... allowedNotes) throws Exception {
        SmartBillServiceModel smartBillServiceModel = SmartBillServiceManager.send(XmlSignatureHelper.getBytesFromDocument(document), SUNAT_BETA_USERNAME, SUNAT_BETA_PASSWORD);
        XmlContentModel xmlContentModel = smartBillServiceModel.getXmlContentModel();
        BillServiceModel billServiceModel = smartBillServiceModel.getBillServiceModel();

        if (billServiceModel.getNotes() != null) {
            List<String> allowedNotesList = Arrays.asList(allowedNotes);

            List<String> notesToCheck = billServiceModel.getNotes().stream().filter(f -> allowedNotesList.stream().noneMatch(f::startsWith)).collect(Collectors.toList());
            notesToCheck.forEach(f -> System.out.println("WARNING:" + f));

            // TODO Fix all warning messages and then apply this validation
            assertTrue(notesToCheck.isEmpty(), "Notes fom SUNAT:\n" + String.join("\n", notesToCheck));
        }

        // Check ticket
        if (!xmlContentModel.getDocumentType().equals(DocumentType.VOIDED_DOCUMENT.getType()) && !xmlContentModel.getDocumentType().equals(DocumentType.SUMMARY_DOCUMENT.getType())) {
            assertEquals(
                    BillServiceModel.Status.ACEPTADO,
                    billServiceModel.getStatus(),
                    xmlWithoutSignature + " \n sunat [codigo=" + billServiceModel.getCode() + "], [descripcion=" + billServiceModel.getDescription() + "]"
            );
        } else {
            assertNotNull(billServiceModel.getTicket());

//            BillServiceModel statusModel = SmartBillServiceManager.getStatus(billServiceModel.getTicket(), xmlContentModel, SUNAT_BETA_USERNAME, SUNAT_BETA_PASSWORD);
//            assertEquals(
//                    BillServiceModel.Status.ACEPTADO,
//                    statusModel.getStatus(),
//                    xmlWithoutSignature + " sunat [status=" + statusModel.getStatus() + "], [descripcion=" + statusModel.getDescription() + "]"
//            );
//            assertNotNull(
//                    statusModel.getCdr(),
//                    xmlWithoutSignature + " sunat [codigo=" + billServiceModel.getCode() + "], [descripcion=" + billServiceModel.getDescription() + "]"
//            );
        }
    }

    private byte[] documentToBytes(Document document) throws TransformerException {
        TransformerFactory transFactory = TransformerFactory.newInstance();
        Transformer transformer = transFactory.newTransformer();

        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        StreamResult result = new StreamResult(bos);
        transformer.transform(new DOMSource(document), result);
        return bos.toByteArray();
    }

    private Document inputStreamToDocument(InputStream in) throws ParserConfigurationException, IOException, SAXException {
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        DocumentBuilder builder = factory.newDocumentBuilder();
        return builder.parse(new InputSource(in));
    }

}
