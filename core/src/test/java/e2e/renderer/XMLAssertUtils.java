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
package e2e.renderer;

import io.github.project.openubl.xbuilder.signature.CertificateDetails;
import io.github.project.openubl.xbuilder.signature.CertificateDetailsFactory;
import io.github.project.openubl.xbuilder.signature.XMLSigner;
import io.github.project.openubl.xbuilder.signature.XmlSignatureHelper;
import io.github.project.openubl.xsender.Constants;
import io.github.project.openubl.xsender.camel.StandaloneCamel;
import io.github.project.openubl.xsender.camel.utils.CamelData;
import io.github.project.openubl.xsender.camel.utils.CamelUtils;
import io.github.project.openubl.xsender.company.CompanyCredentials;
import io.github.project.openubl.xsender.company.CompanyURLs;
import io.github.project.openubl.xsender.files.BillServiceFileAnalyzer;
import io.github.project.openubl.xsender.files.BillServiceXMLFileAnalyzer;
import io.github.project.openubl.xsender.files.ZipFile;
import io.github.project.openubl.xsender.files.xml.DocumentType;
import io.github.project.openubl.xsender.files.xml.XmlContent;
import io.github.project.openubl.xsender.models.Status;
import io.github.project.openubl.xsender.models.SunatResponse;
import io.github.project.openubl.xsender.sunat.BillServiceDestination;
import org.apache.camel.CamelContext;
import org.w3c.dom.Document;
import org.xmlunit.builder.DiffBuilder;
import org.xmlunit.diff.Diff;

import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

import static io.github.project.openubl.xsender.camel.utils.CamelUtils.getBillServiceCamelData;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class XMLAssertUtils {

    public static final CompanyURLs companyURLs = CompanyURLs
            .builder()
            .invoice("https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService")
            .despatch("https://e-beta.sunat.gob.pe/ol-ti-itemision-otroscpe-gem-beta/billService")
            .perceptionRetention("https://e-beta.sunat.gob.pe/ol-ti-itemision-otroscpe-gem-beta/billService")
            .build();

    public static final CompanyCredentials credentials = CompanyCredentials
            .builder()
            .username("12345678959MODDATOS")
            .password("MODDATOS")
            .build();

    private static final String SIGN_REFERENCE_ID = "PROJECT-OPENUBL";
    private static final String KEYSTORE = "LLAMA-PE-CERTIFICADO-DEMO-10467793549.pfx";
    private static final String KEYSTORE_PASSWORD = "password";
    private static final CertificateDetails CERTIFICATE;

    static {
        InputStream ksInputStream = Thread.currentThread().getContextClassLoader().getResourceAsStream(KEYSTORE);
        try {
            CERTIFICATE = CertificateDetailsFactory.create(ksInputStream, KEYSTORE_PASSWORD);
        } catch (Exception e) {
            throw new IllegalStateException(e);
        }
    }

    public static void assertSnapshot(String expected, Class<?> clasz, String snapshotFile) {
        String rootDir = clasz.getName().replaceAll("\\.", "/");

        // Update snapshots and if updated do not verify since it doesn't make sense anymore
        boolean updateSnapshots = Boolean.parseBoolean(System.getProperty("xbuilder.snapshot.update", "false"));
        if (updateSnapshots) {
            try {
                Path directoryPath = Paths.get("src", "test", "resources").resolve(rootDir);
                Files.createDirectories(directoryPath);

                Path filePath = directoryPath.resolve(snapshotFile);
                Files.write(filePath, expected.getBytes());

                return;
            } catch (Exception e) {
                throw new IllegalStateException(e);
            }
        }

        InputStream snapshotInputStream = Thread
                .currentThread()
                .getContextClassLoader()
                .getResourceAsStream(rootDir + "/" + snapshotFile);
        assertNotNull(snapshotInputStream, "Could not find snapshot file " + snapshotFile);

        Diff myDiff = DiffBuilder
                .compare(snapshotInputStream)
                .withTest(expected)
                .ignoreComments()
                .ignoreWhitespace()
                .build();

        assertFalse(myDiff.hasDifferences(), expected + "\n" + myDiff);
    }

    public static void assertSendSunat(String xmlWithoutSignature, String... allowedNotes) throws Exception {
        String skipSunat = System.getProperty("skipSunat", "false");
        if (skipSunat != null && skipSunat.equals("false")) {
            Document signedXML = XMLSigner.signXML(
                    xmlWithoutSignature,
                    SIGN_REFERENCE_ID,
                    CERTIFICATE.getX509Certificate(),
                    CERTIFICATE.getPrivateKey()
            );
            sendFileToSunat(signedXML, xmlWithoutSignature, allowedNotes);
        }
    }

    //

    private static void sendFileToSunat(Document document, String xmlWithoutSignature, String... allowedNotes)
            throws Exception {
        byte[] bytesFromDocument = XmlSignatureHelper.getBytesFromDocument(document);

        CamelContext camelContext = StandaloneCamel.getInstance().getMainCamel().getCamelContext();

        BillServiceFileAnalyzer fileAnalyzer = new BillServiceXMLFileAnalyzer(bytesFromDocument, companyURLs);
        ZipFile zipFile = fileAnalyzer.getZipFile();
        BillServiceDestination fileDestination = fileAnalyzer.getSendFileDestination();
        BillServiceDestination ticketDestination = fileAnalyzer.getVerifyTicketDestination();

        CamelData camelData = getBillServiceCamelData(zipFile, fileDestination, credentials);
        SunatResponse sendFileSunatResponse = camelContext
                .createProducerTemplate()
                .requestBodyAndHeaders(
                        Constants.XSENDER_BILL_SERVICE_URI,
                        camelData.getBody(),
                        camelData.getHeaders(),
                        SunatResponse.class
                );

        if (sendFileSunatResponse.getMetadata() != null && sendFileSunatResponse.getMetadata().getNotes() != null) {
            List<String> allowedNotesList = Arrays.asList(allowedNotes);

            List<String> notesToCheck = sendFileSunatResponse
                    .getMetadata()
                    .getNotes()
                    .stream()
                    .filter(f -> allowedNotesList.stream().noneMatch(f::startsWith))
                    .collect(Collectors.toList());
            notesToCheck.forEach(f -> System.out.println("WARNING:" + f));

            assertTrue(notesToCheck.isEmpty(), "Notes fom SUNAT:\n" + String.join("\n", notesToCheck));
        }

        XmlContent xmlContent = fileAnalyzer.getXmlContent();
        // Check ticket
        if (
                !xmlContent.getDocumentType().equals(DocumentType.VOIDED_DOCUMENT) &&
                        !xmlContent.getDocumentType().equals(DocumentType.SUMMARY_DOCUMENT)
        ) {
            assertEquals(
                    Status.ACEPTADO,
                    sendFileSunatResponse.getStatus(),
                    xmlWithoutSignature +
                            " \n sunat [codigo=" +
                            sendFileSunatResponse.getMetadata().getResponseCode() +
                            "], [descripcion=" +
                            sendFileSunatResponse.getMetadata().getDescription() +
                            "]"
            );
        } else {
            assertNotNull(sendFileSunatResponse.getSunat().getTicket());

            CamelData camelTicketData = CamelUtils.getBillServiceCamelData(
                    sendFileSunatResponse.getSunat().getTicket(),
                    ticketDestination,
                    credentials
            );
            SunatResponse verifyTicketSunatResponse = camelContext
                    .createProducerTemplate()
                    .requestBodyAndHeaders(
                            Constants.XSENDER_BILL_SERVICE_URI,
                            camelTicketData.getBody(),
                            camelTicketData.getHeaders(),
                            SunatResponse.class
                    );

            assertEquals(
                    Status.ACEPTADO,
                    verifyTicketSunatResponse.getStatus(),
                    xmlWithoutSignature +
                            " sunat [status=" +
                            verifyTicketSunatResponse.getStatus() +
                            "], [descripcion=" +
                            verifyTicketSunatResponse.getMetadata().getDescription() +
                            "]"
            );
            assertNotNull(
                    verifyTicketSunatResponse.getSunat().getCdr(),
                    xmlWithoutSignature +
                            " sunat [codigo=" +
                            verifyTicketSunatResponse.getMetadata().getResponseCode() +
                            "], [descripcion=" +
                            verifyTicketSunatResponse.getMetadata().getDescription() +
                            "]"
            );
        }
    }
}
