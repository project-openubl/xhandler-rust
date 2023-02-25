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
package io.github.project.openubl.xbuilder.content.jaxb;

import io.github.project.openubl.xbuilder.content.jaxb.models.XMLPercepcionRetencion;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLPercepcionRetencionInformation;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLPercepcionRetencionSunatDocumentReference;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSalesDocument;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSunatDocument;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSunatDocumentVoidedDocument;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSunatDocumentVoidedDocumentLine;
import io.github.project.openubl.xbuilder.content.models.common.Document;
import io.github.project.openubl.xbuilder.content.models.common.TipoCambio;
import io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.content.models.standard.general.Note;
import io.github.project.openubl.xbuilder.content.models.standard.general.SalesDocument;
import io.github.project.openubl.xbuilder.content.models.sunat.SunatDocument;
import io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocuments;
import io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocumentsItem;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.BasePercepcionRetencion;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.ComprobanteAfectado;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.PercepcionRetencionOperacion;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Perception;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Retention;
import org.eclipse.persistence.jaxb.JAXBContextFactory;
import org.eclipse.persistence.jaxb.JAXBContextProperties;
import org.xml.sax.InputSource;

import javax.xml.bind.JAXBContext;
import javax.xml.bind.JAXBException;
import javax.xml.bind.Unmarshaller;
import java.io.IOException;
import java.io.InputStream;
import java.io.StringReader;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.stream.Collectors;

public class Unmarshall {

    public static Invoice unmarshallInvoice(String xml) throws JAXBException, IOException {
        try (
                InputStream documentOXM = Thread.currentThread().getContextClassLoader().getResourceAsStream("jaxb/xml-bindings/invoice.xml");
                StringReader reader = new StringReader(xml);
        ) {
            XMLSalesDocument xmlDocument = unmarshall(documentOXM, new InputSource(reader));
            Invoice.InvoiceBuilder<?, ?> builder = Invoice.builder();

            enrichSalesDocument(xmlDocument, builder);

            // Fecha de vencimiento
            builder.fechaVencimiento(xmlDocument.getDueDate());

            // Tipo de comprobante
            builder.tipoComprobante(xmlDocument.getInvoiceTypeCode());

            // Observaciones
            if (xmlDocument.getNotes() != null) {
                xmlDocument.getNotes().stream()
                        .filter(e -> e.getLanguageLocaleId() == null)
                        .findFirst().ifPresent(n -> {
                            builder.observaciones(n.getValue());
                        });
            }

            // Tipo de operacion
            builder.tipoOperacion(xmlDocument.getInvoiceTypeCode_listID());

            // Forma de pago
            builder.formaDePago(Mapper.mapFormaDePago(xmlDocument.getPaymentTerms()));

            // Total importe
            builder.totalImporte(Mapper.mapTotalImporteInvoice(xmlDocument.getMonetaryTotal()));

            // Direccion entrega
            builder.direccionEntrega(Mapper.mapDireccion(xmlDocument.getDeliveryLocation()));

            // Detraccion
            builder.detraccion(Mapper.mapDetraccion(xmlDocument.getPaymentMeans(), xmlDocument.getPaymentTerms()));

            // Percepcion
            builder.percepcion(Mapper.mapPercepcion(xmlDocument.getPaymentTerms(), xmlDocument.getAllowanceCharges()));

            // Anticipos
            builder.anticipos(Mapper.mapAnticipos(xmlDocument.getAdditionalDocumentReferences(), xmlDocument.getPrepaidPayments(), xmlDocument.getAllowanceCharges()));

            // Descuentos
            builder.descuentos(Mapper.mapDescuentos(xmlDocument.getAllowanceCharges()));

            return builder.build();
        }
    }

    public static CreditNote unmarshallCreditNote(String xml) throws JAXBException, IOException {
        try (
                InputStream documentOXM = Thread.currentThread().getContextClassLoader().getResourceAsStream("jaxb/xml-bindings/credit-note.xml");
                StringReader reader = new StringReader(xml);
        ) {
            XMLSalesDocument xmlDocument = unmarshall(documentOXM, new InputSource(reader));
            CreditNote.CreditNoteBuilder<?, ?> builder = CreditNote.builder();

            enrichSalesDocument(xmlDocument, builder);
            enrichNote(xmlDocument, builder);

            return builder.build();
        }
    }

    public static DebitNote unmarshallDebitNote(String xml) throws JAXBException, IOException {
        try (
                InputStream documentOXM = Thread.currentThread().getContextClassLoader().getResourceAsStream("jaxb/xml-bindings/debit-note.xml");
                StringReader reader = new StringReader(xml);
        ) {
            XMLSalesDocument xmlDocument = unmarshall(documentOXM, new InputSource(reader));
            DebitNote.DebitNoteBuilder<?, ?> builder = DebitNote.builder();

            enrichSalesDocument(xmlDocument, builder);
            enrichNote(xmlDocument, builder);

            return builder.build();
        }
    }

    public static Perception unmarshallPerception(String xml) throws JAXBException, IOException {
        try (
                InputStream documentOXM = Thread.currentThread().getContextClassLoader().getResourceAsStream("jaxb/xml-bindings/perception.xml");
                StringReader reader = new StringReader(xml);
        ) {
            XMLPercepcionRetencion xmlDocument = unmarshall(documentOXM, new InputSource(reader));
            Perception.PerceptionBuilder<?, ?> builder = Perception.builder();

            enrichBasePercepcionRetencion(xmlDocument, builder);

            // Serie
            String[] split = xmlDocument.getDocumentId().split("-");
            if (split.length == 2) {
                builder.serie(split[0]);
            }

            // Tipo de regimen
            builder.tipoRegimen(xmlDocument.getSunatSystemCode());

            // ImporteTotalPercibido, ImporteTotalCobrado
            XMLPercepcionRetencionSunatDocumentReference sunatDocumentReference = xmlDocument.getSunatDocumentReference();
            if (sunatDocumentReference != null) {
                XMLPercepcionRetencionInformation sunatInformation = sunatDocumentReference.getSunatInformation();
                if (sunatInformation != null) {
                    builder.importeTotalPercibido(sunatInformation.getSunatAmount());
                    builder.importeTotalCobrado(sunatInformation.getSunatNetTotal());
                }
            }

            return builder.build();
        }
    }

    public static Retention unmarshallRetention(String xml) throws JAXBException, IOException {
        try (
                InputStream documentOXM = Thread.currentThread().getContextClassLoader().getResourceAsStream("jaxb/xml-bindings/retention.xml");
                StringReader reader = new StringReader(xml);
        ) {
            XMLPercepcionRetencion xmlDocument = unmarshall(documentOXM, new InputSource(reader));
            Retention.RetentionBuilder<?, ?> builder = Retention.builder();

            enrichBasePercepcionRetencion(xmlDocument, builder);

            // Serie
            String[] split = xmlDocument.getDocumentId().split("-");
            if (split.length == 2) {
                builder.serie(split[0]);
            }

            // Tipo de regimen
            builder.tipoRegimen(xmlDocument.getSunatSystemCode());

            // ImporteTotalPercibido, ImporteTotalCobrado
            XMLPercepcionRetencionSunatDocumentReference sunatDocumentReference = xmlDocument.getSunatDocumentReference();
            if (sunatDocumentReference != null) {
                XMLPercepcionRetencionInformation sunatInformation = sunatDocumentReference.getSunatInformation();
                if (sunatInformation != null) {
                    builder.importeTotalRetenido(sunatInformation.getSunatAmount());
                    builder.importeTotalPagado(sunatInformation.getSunatNetTotal());
                }
            }

            return builder.build();
        }
    }

    public static VoidedDocuments unmarshallVoidedDocuments(String xml) throws JAXBException, IOException {
        try (
                InputStream documentOXM = Thread.currentThread().getContextClassLoader().getResourceAsStream("jaxb/xml-bindings/voided-documents.xml");
                StringReader reader = new StringReader(xml);
        ) {
            XMLSunatDocumentVoidedDocument xmlDocument = unmarshall(documentOXM, new InputSource(reader));
            VoidedDocuments.VoidedDocumentsBuilder<?, ?> builder = VoidedDocuments.builder();

            enrichSunatDocument(xmlDocument, builder);

            // Detalles
            List<XMLSunatDocumentVoidedDocumentLine> lines = xmlDocument.getLines();
            if (lines != null) {
                builder.comprobantes(lines.stream()
                        .map(line -> VoidedDocumentsItem.builder()
                                .tipoComprobante(line.getDocumentTypeCode())
                                .serie(line.getDocumentSerialID())
                                .numero(line.getDocumentNumberID())
                                .descripcionSustento(line.getVoidReasonDescription())
                                .build()
                        )
                        .collect(Collectors.toList())
                );
            }

            return builder.build();
        }
    }

    public static <T> T unmarshall(InputStream documentOXML, InputSource inputSource) throws JAXBException {
        Map<String, Object> properties = new HashMap<>();
        properties.put(JAXBContextProperties.OXM_METADATA_SOURCE, List.of(
                Objects.requireNonNull(documentOXML)
        ));

        JAXBContext jaxbContext = JAXBContextFactory.createContext(Unmarshall.class.getPackageName(), null, properties);
        Unmarshaller unmarshaller = jaxbContext.createUnmarshaller();
        return (T) unmarshaller.unmarshal(inputSource);
    }

    public static void enrichSalesDocument(XMLSalesDocument xmlSalesDocument, SalesDocument.SalesDocumentBuilder<?, ?> builder) {
        enrichDocument(xmlSalesDocument, builder);

        // Leyendas
        Optional<Map<String, String>> notes = Optional
                .ofNullable(xmlSalesDocument.getNotes())
                .map(o -> o.stream()
                        .filter(note -> note.getLanguageLocaleId() != null)
                        .collect(Collectors.toMap(XMLSalesDocument.Note::getLanguageLocaleId, XMLSalesDocument.Note::getValue))
                );
        notes.ifPresent(builder::leyendas);

        // Serie y numero
        String[] split = xmlSalesDocument.getDocumentId().split("-");
        if (split.length == 2) {
            builder.serie(split[0]);
            builder.numero(Integer.parseInt(split[1]));
        }

        //
        builder.horaEmision(xmlSalesDocument.getIssueTime());
        builder.ordenDeCompra(xmlSalesDocument.getOrderReferenceId());

        // Cliente
        builder.cliente(Mapper.mapCliente(xmlSalesDocument.getAccountingCustomerParty()));

        // Total impuestos
        builder.totalImpuestos(Mapper.mapTotalImpuestos(xmlSalesDocument.getTaxTotal()));

        // Guias
        builder.guias(Mapper.mapGuias(xmlSalesDocument.getDespatchDocumentReferences()));

        // Documentos relacionados
        builder.documentosRelacionados(Mapper.mapDocumentosRelacionados(xmlSalesDocument.getAdditionalDocumentReferences()));

        // Detalles
        builder.detalles(Mapper.mapDetalles(xmlSalesDocument.getLines()));
    }

    public static void enrichNote(XMLSalesDocument xmlSalesDocument, Note.NoteBuilder<?, ?> builder) {
        enrichDocument(xmlSalesDocument, builder);

        // ComprobanteAfectado
        builder.comprobanteAfectadoSerieNumero(xmlSalesDocument.getDiscrepancyResponse_referenceId());
        builder.comprobanteAfectadoTipo(xmlSalesDocument.getDiscrepancyResponse_responseCode());
        builder.sustentoDescripcion(xmlSalesDocument.getDiscrepancyResponse_description());

        // Total importe
        builder.totalImporte(Mapper.mapTotalImporteNote(xmlSalesDocument.getMonetaryTotal()));
    }

    public static void enrichBasePercepcionRetencion(XMLPercepcionRetencion xmlDocument, BasePercepcionRetencion.BasePercepcionRetencionBuilder<?, ?> builder) {
        enrichDocument(xmlDocument, builder);

        // Numero
        String[] split = xmlDocument.getDocumentId().split("-");
        if (split.length == 2) {
            builder.numero(Integer.parseInt(split[1]));
        }

        // Tipo regimen porcentaje
        builder.tipoRegimenPorcentaje(xmlDocument.getSunatPercent());

        // Observacion
        builder.observacion(xmlDocument.getNote());

        // Cliente
        builder.cliente(Mapper.mapCliente(xmlDocument.getAccountingCustomerParty()));

        // Operacion
        XMLPercepcionRetencionSunatDocumentReference sunatDocumentReference = xmlDocument.getSunatDocumentReference();

        if (sunatDocumentReference != null) {
            XMLPercepcionRetencionInformation sunatInformation = sunatDocumentReference.getSunatInformation();

            TipoCambio tipoCambio = null;
            if (sunatInformation != null && sunatInformation.getExchangeRate() != null) {
                tipoCambio = TipoCambio.builder()
                        .fecha(sunatInformation.getExchangeRate().getDate())
                        .valor(sunatInformation.getExchangeRate().getCanculationRate())
                        .build();
            }

            builder.operacion(PercepcionRetencionOperacion.builder()
                    .numeroOperacion(sunatDocumentReference.getPayment_id())
                    .fechaOperacion(sunatDocumentReference.getPayment_paidDate())
                    .importeOperacion(sunatDocumentReference.getPayment_paidAmount())
                    .comprobante(ComprobanteAfectado.builder()
                            .moneda(sunatDocumentReference.getTotalInvoiceAmount_currencyId())
                            .tipoComprobante(sunatDocumentReference.getId_schemeId())
                            .serieNumero(sunatDocumentReference.getId())
                            .fechaEmision(sunatDocumentReference.getIssueDate())
                            .importeTotal(sunatDocumentReference.getTotalInvoiceAmount())
                            .build()
                    )
                    .tipoCambio(tipoCambio)
                    .build()
            );
        }

    }

    public static void enrichSunatDocument(XMLSunatDocument xmlDocument, SunatDocument.SunatDocumentBuilder<?, ?> builder) {
        enrichDocument(xmlDocument, builder);

        // Numero
        String[] split = xmlDocument.getDocumentId().split("-");
        if (split.length == 3) {
            builder.numero(Integer.parseInt(split[2]));
        }

        // Fecha emision comprobante
        builder.fechaEmisionComprobantes(xmlDocument.getReferenceDate());
    }

    public static void enrichDocument(XMLSalesDocument xmlSalesDocument, Document.DocumentBuilder<?, ?> builder) {
        builder.moneda(xmlSalesDocument.getDocumentCurrencyCode());
        builder.fechaEmision(xmlSalesDocument.getIssueDate());
        builder.proveedor(Mapper.mapProveedor(xmlSalesDocument.getAccountingSupplierParty()));
        builder.firmante(Mapper.mapFirmante(xmlSalesDocument.getSignature()));
    }

    public static void enrichDocument(XMLPercepcionRetencion xmlDocument, Document.DocumentBuilder<?, ?> builder) {
        builder.moneda(xmlDocument.getTotalInvoiceAmount_currencyId());
        builder.fechaEmision(xmlDocument.getIssueDate());
        builder.proveedor(Mapper.mapProveedor(xmlDocument.getAccountingSupplierParty()));
        builder.firmante(Mapper.mapFirmante(xmlDocument.getSignature()));
    }

    public static void enrichDocument(XMLSunatDocument xmlDocument, Document.DocumentBuilder<?, ?> builder) {
        builder.moneda(null);
        builder.fechaEmision(xmlDocument.getIssueDate());
        builder.proveedor(Mapper.mapProveedor(xmlDocument.getAccountingSupplierParty()));
        builder.firmante(Mapper.mapFirmante(xmlDocument.getSignature()));
    }

}
