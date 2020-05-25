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
package io.github.project.openubl.xmlbuilderlib.xml;

import io.github.project.openubl.xmlbuilderlib.integrationtest.utils.CertificateDetails;
import io.github.project.openubl.xmlbuilderlib.integrationtest.utils.CertificateDetailsFactory;
import org.junit.jupiter.api.Test;
import org.w3c.dom.Document;
import org.w3c.dom.NodeList;

import javax.xml.crypto.dsig.Reference;
import javax.xml.crypto.dsig.XMLSignature;
import javax.xml.crypto.dsig.XMLSignatureFactory;
import javax.xml.crypto.dsig.dom.DOMValidateContext;
import javax.xml.parsers.DocumentBuilderFactory;
import java.io.InputStream;
import java.util.Iterator;

import static org.junit.jupiter.api.Assertions.assertTrue;

public class XMLSignerTest {

    @Test
    public void signXML() throws Exception {
        // Given
        InputStream ksInputStream = Thread.currentThread().getContextClassLoader().getResourceAsStream("LLAMA-PE-CERTIFICADO-DEMO-10467793549.pfx");
        CertificateDetails CERTIFICATE = CertificateDetailsFactory.create(ksInputStream, "password");

        InputStream is = Thread.currentThread().getContextClassLoader().getResourceAsStream("xml/invoice_specialCharacters.xml");

        DocumentBuilderFactory dbf = DocumentBuilderFactory.newInstance();
        dbf.setNamespaceAware(true);
        Document documentToSign = dbf.newDocumentBuilder().parse(is);

        // When
        Document signedDocument = XMLSigner.signXML(documentToSign, "PROJECT-OPENUBL", CERTIFICATE.getX509Certificate(), CERTIFICATE.getPrivateKey());

        // Then
        NodeList nodeList = signedDocument.getElementsByTagNameNS(XMLSignature.XMLNS, "Signature");
        if (nodeList.getLength() == 0) {
            throw new Exception("Cannot find Signature element");
        }

        DOMValidateContext valContext = new DOMValidateContext(CERTIFICATE.getX509Certificate().getPublicKey(), nodeList.item(0));
        XMLSignatureFactory fac = XMLSignatureFactory.getInstance("DOM");
        XMLSignature signature = fac.unmarshalXMLSignature(valContext);
        boolean coreValidity = signature.validate(valContext);
        assertTrue(coreValidity);

        boolean signatureValidity = signature.getSignatureValue().validate(valContext);
        assertTrue(signatureValidity);

        Iterator<Reference> i = signature.getSignedInfo().getReferences().iterator();
        for (int j = 0; i.hasNext(); j++) {
            boolean refValid = i.next().validate(valContext);
            assertTrue(refValid);
        }
    }
}
