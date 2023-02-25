package io.github.project.openubl.xbuilder.content.jaxb;

import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSalesDocument;
import io.github.project.openubl.xbuilder.content.models.common.Document;
import io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.content.models.standard.general.Note;
import io.github.project.openubl.xbuilder.content.models.standard.general.SalesDocument;
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
            XMLSalesDocument xmlSalesDocument = unmarshall(documentOXM, new InputSource(reader));
            Invoice.InvoiceBuilder<?, ?> builder = Invoice.builder();

            enrichSalesDocument(xmlSalesDocument, builder);

            // Fecha de vencimiento
            builder.fechaVencimiento(xmlSalesDocument.getDueDate());

            // Tipo de comprobante
            builder.tipoComprobante(xmlSalesDocument.getInvoiceTypeCode());

            // Observaciones
            if (xmlSalesDocument.getNotes() != null) {
                xmlSalesDocument.getNotes().stream()
                        .filter(e -> e.getLanguageLocaleId() == null)
                        .findFirst().ifPresent(n -> {
                            builder.observaciones(n.getValue());
                        });
            }

            // Tipo de operacion
            builder.tipoOperacion(xmlSalesDocument.getInvoiceTypeCode_listID());

            // Forma de pago
            builder.formaDePago(Mapper.mapFormaDePago(xmlSalesDocument.getPaymentTerms()));

            // Total importe
            builder.totalImporte(Mapper.mapTotalImporteInvoice(xmlSalesDocument.getMonetaryTotal()));

            // Direccion entrega
            builder.direccionEntrega(Mapper.mapDireccion(xmlSalesDocument.getDeliveryLocation()));

            // Detraccion
            builder.detraccion(Mapper.mapDetraccion(xmlSalesDocument.getPaymentMeans(), xmlSalesDocument.getPaymentTerms()));

            // Percepcion
            builder.percepcion(Mapper.mapPercepcion(xmlSalesDocument.getPaymentTerms(), xmlSalesDocument.getAllowanceCharges()));

            // Anticipos
            builder.anticipos(Mapper.mapAnticipos(xmlSalesDocument.getAdditionalDocumentReferences(), xmlSalesDocument.getPrepaidPayments(), xmlSalesDocument.getAllowanceCharges()));

            // Descuentos
            builder.descuentos(Mapper.mapDescuentos(xmlSalesDocument.getAllowanceCharges()));

            return builder.build();
        }
    }

    public static CreditNote unmarshallCreditNote(String xml) throws JAXBException, IOException {
        try (
                InputStream documentOXM = Thread.currentThread().getContextClassLoader().getResourceAsStream("jaxb/xml-bindings/credit-note.xml");
                StringReader reader = new StringReader(xml);
        ) {
            XMLSalesDocument xmlSalesDocument = unmarshall(documentOXM, new InputSource(reader));
            CreditNote.CreditNoteBuilder<?, ?> builder = CreditNote.builder();

            enrichSalesDocument(xmlSalesDocument, builder);
            enrichNote(xmlSalesDocument, builder);

            return builder.build();
        }
    }

    public static DebitNote unmarshallDebitNote(String xml) throws JAXBException, IOException {
        try (
                InputStream documentOXM = Thread.currentThread().getContextClassLoader().getResourceAsStream("jaxb/xml-bindings/debit-note.xml");
                StringReader reader = new StringReader(xml);
        ) {
            XMLSalesDocument xmlSalesDocument = unmarshall(documentOXM, new InputSource(reader));
            DebitNote.DebitNoteBuilder<?, ?> builder = DebitNote.builder();

            enrichSalesDocument(xmlSalesDocument, builder);
            enrichNote(xmlSalesDocument, builder);

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

    public static void enrichDocument(XMLSalesDocument xmlSalesDocument, Document.DocumentBuilder<?, ?> builder) {
        builder.moneda(xmlSalesDocument.getDocumentCurrencyCode());
        builder.fechaEmision(xmlSalesDocument.getIssueDate());
        builder.proveedor(Mapper.mapProveedor(xmlSalesDocument.getAccountingSupplierParty()));
        builder.firmante(Mapper.mapFirmante(xmlSalesDocument.getSignature()));
    }

}
