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

import io.github.project.openubl.xbuilder.content.catalogs.Catalog;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog5;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLDespatchAdvice;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLPercepcionRetencion;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLPercepcionRetencionInformation;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLPercepcionRetencionSunatDocumentReference;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSalesDocument;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSunatDocument;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSunatDocumentSummaryDocuments;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSunatDocumentSummaryDocumentsLine;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSunatDocumentVoidedDocuments;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSunatDocumentVoidedDocumentsLine;
import io.github.project.openubl.xbuilder.content.models.common.Cliente;
import io.github.project.openubl.xbuilder.content.models.common.Document;
import io.github.project.openubl.xbuilder.content.models.common.Proveedor;
import io.github.project.openubl.xbuilder.content.models.common.TipoCambio;
import io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.content.models.standard.general.Note;
import io.github.project.openubl.xbuilder.content.models.standard.general.SalesDocument;
import io.github.project.openubl.xbuilder.content.models.standard.guia.DespatchAdvice;
import io.github.project.openubl.xbuilder.content.models.standard.guia.DespatchAdviceItem;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Destinatario;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Destino;
import io.github.project.openubl.xbuilder.content.models.standard.guia.DocumentoBaja;
import io.github.project.openubl.xbuilder.content.models.standard.guia.DocumentoRelacionado;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Envio;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Partida;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Remitente;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Transportista;
import io.github.project.openubl.xbuilder.content.models.sunat.SunatDocument;
import io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocuments;
import io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocumentsItem;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.BasePercepcionRetencion;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.ComprobanteAfectado;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.PercepcionRetencionOperacion;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Perception;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Retention;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.Comprobante;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteImpuestos;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteValorVenta;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocuments;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocumentsItem;
import org.eclipse.persistence.jaxb.JAXBContextFactory;
import org.eclipse.persistence.jaxb.JAXBContextProperties;
import org.xml.sax.InputSource;

import javax.xml.bind.JAXBContext;
import javax.xml.bind.JAXBException;
import javax.xml.bind.Unmarshaller;
import java.io.IOException;
import java.io.InputStream;
import java.io.StringReader;
import java.math.BigDecimal;
import java.util.Collections;
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
            XMLSunatDocumentVoidedDocuments xmlDocument = unmarshall(documentOXM, new InputSource(reader));
            VoidedDocuments.VoidedDocumentsBuilder<?, ?> builder = VoidedDocuments.builder();

            enrichSunatDocument(xmlDocument, builder);

            // Detalles
            List<XMLSunatDocumentVoidedDocumentsLine> lines = xmlDocument.getLines();
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

    public static SummaryDocuments unmarshallSummaryDocuments(String xml) throws JAXBException, IOException {
        try (
                InputStream documentOXM = Thread.currentThread().getContextClassLoader().getResourceAsStream("jaxb/xml-bindings/summary-documents.xml");
                StringReader reader = new StringReader(xml);
        ) {
            XMLSunatDocumentSummaryDocuments xmlDocument = unmarshall(documentOXM, new InputSource(reader));
            SummaryDocuments.SummaryDocumentsBuilder<?, ?> builder = SummaryDocuments.builder();

            enrichSunatDocument(xmlDocument, builder);

            // Detalles
            List<XMLSunatDocumentSummaryDocumentsLine> lines = xmlDocument.getLines();
            if (lines != null) {
                builder.comprobantes(lines.stream()
                        .map(line -> {
                                    Map<String, BigDecimal> billingPayments = Optional.ofNullable(line.getBillingPayments())
                                            .orElse(Collections.emptyList())
                                            .stream()
                                            .collect(Collectors.toMap(
                                                    XMLSunatDocumentSummaryDocumentsLine.BillingPayment::getInstructionId,
                                                    XMLSunatDocumentSummaryDocumentsLine.BillingPayment::getPaidAmount
                                            ));

                                    Map<Catalog5, BigDecimal> taxTotals = Optional.ofNullable(line.getTaxTotals())
                                            .orElse(Collections.emptyList())
                                            .stream()
                                            .collect(Collectors.toMap(
                                                    taxTotal -> {
                                                        String code = taxTotal.getTaxSubtotals() != null ? taxTotal.getTaxSubtotals().getCode() : "";
                                                        return Catalog
                                                                .valueOfCode(Catalog5.class, code)
                                                                .orElse(null);
                                                    },
                                                    XMLSunatDocumentSummaryDocumentsLine.TaxTotalSummaryDocuments::getTaxAmount
                                            ));

                                    io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteAfectado comprobanteAfectado = Optional.ofNullable(line.getBillingReference())
                                            .map(billingReference -> io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteAfectado.builder()
                                                    .serieNumero(billingReference.getInvoiceDocumentReference_id())
                                                    .tipoComprobante(billingReference.getInvoiceDocumentReference_documentTypeCode())
                                                    .build()
                                            )
                                            .orElse(null);

                                    return SummaryDocumentsItem.builder()
                                            .tipoOperacion(line.getStatus_conditionCode())
                                            .comprobante(Comprobante.builder()
                                                    .moneda(line.getTotalAmount_currencyID())
                                                    .tipoComprobante(line.getDocumentTypeCode())
                                                    .serieNumero(line.getDocumentId())
                                                    .cliente(Cliente.builder()
                                                            .numeroDocumentoIdentidad(line.getAccountingCustomerParty_customerAssignedAccountId())
                                                            .tipoDocumentoIdentidad(line.getAccountingCustomerParty_additionalAccountID())
                                                            .build()
                                                    )
                                                    .comprobanteAfectado(comprobanteAfectado)
                                                    .valorVenta(ComprobanteValorVenta.builder()
                                                            .importeTotal(line.getTotalAmount())
                                                            .gravado(billingPayments.get("01"))
                                                            .exonerado(billingPayments.get("02"))
                                                            .inafecto(billingPayments.get("03"))
                                                            .gratuito(billingPayments.get("05"))
                                                            .otrosCargos(line.getAllowanceCharge_amount())
                                                            .build()
                                                    )
                                                    .impuestos(ComprobanteImpuestos.builder()
                                                            .igv(taxTotals.get(Catalog5.IGV))
                                                            .icb(taxTotals.get(Catalog5.ICBPER))
                                                            .build()
                                                    )
                                                    .build()
                                            )
                                            .build();
                                }
                        )
                        .collect(Collectors.toList())
                );
            }

            return builder.build();
        }
    }

    public static DespatchAdvice unmarshallDespatchAdvice(String xml) throws JAXBException, IOException {
        try (
                InputStream documentOXM = Thread.currentThread().getContextClassLoader().getResourceAsStream("jaxb/xml-bindings/despatch-advice.xml");
                StringReader reader = new StringReader(xml);
        ) {
            XMLDespatchAdvice xmlDocument = unmarshall(documentOXM, new InputSource(reader));
            DespatchAdvice.DespatchAdviceBuilder builder = DespatchAdvice.builder();

            // Serie y numero
            String[] split = xmlDocument.getDocumentId().split("-");
            if (split.length == 2) {
                builder.serie(split[0]);
                builder.numero(Integer.parseInt(split[1]));
            }

            builder.fechaEmision(xmlDocument.getIssueDate());
            builder.horaEmision(xmlDocument.getIssueTime());
            builder.tipoComprobante(xmlDocument.getDespatchAdviceTypeCode());
            builder.observaciones(xmlDocument.getNote());
            builder.documentoBaja(Optional.ofNullable(xmlDocument.getOrderReference())
                    .map(elem -> DocumentoBaja.builder()
                            .serieNumero(elem.getId())
                            .tipoDocumento(elem.getOrderTypeCode())
                            .build()
                    )
                    .orElse(null)
            );
            builder.documentoRelacionado(Optional.ofNullable(xmlDocument.getAdditionalDocumentReference())
                    .map(elem -> DocumentoRelacionado.builder()
                            .serieNumero(elem.getId())
                            .tipoDocumento(elem.getDocumentTypeCode())
                            .build()
                    )
                    .orElse(null)
            );
            builder.firmante(Mapper.mapFirmante(xmlDocument.getSignature()));

            builder.remitente(Optional.ofNullable(xmlDocument.getDespatchSupplierParty())
                    .map(elem -> Remitente.builder()
                            .ruc(elem.getCustomerAssignedAccountID())
                            .razonSocial(elem.getRegistrationName())
                            .build()
                    )
                    .orElse(null)
            );

            builder.destinatario(Optional.ofNullable(xmlDocument.getDeliveryCustomerParty())
                    .map(elem -> Destinatario.builder()
                            .numeroDocumentoIdentidad(elem.getPartyIdentification_id())
                            .nombre(elem.getRegistrationName())
                            .tipoDocumentoIdentidad(elem.getCustomerAssignedAccountID_schemeId())
                            .build()
                    )
                    .orElse(null)
            );

            builder.proveedor(Optional.ofNullable(xmlDocument.getSellerSupplierParty())
                    .map(elem -> Proveedor.builder()
                            .ruc(elem.getCustomerAssignedAccountId())
                            .nombreComercial(elem.getRegistrationName())
                            .build()
                    )
                    .orElse(null)
            );

            builder.envio(Optional.ofNullable(xmlDocument.getShipment())
                    .map(elem -> {
                                Transportista transportista = null;
                                if (elem.getCarrierParty() != null || elem.getTransportMeans() != null || elem.getDriverPerson() != null) {
                                    Transportista.TransportistaBuilder transportistaBuilder = Transportista.builder();

                                    Optional.ofNullable(elem.getCarrierParty())
                                            .ifPresent(carrierParty -> transportistaBuilder
                                                    .tipoDocumentoIdentidad(carrierParty.getPartyIdentification_schemeId())
                                                    .numeroDocumentoIdentidad(carrierParty.getPartyIdentification_id())
                                                    .nombre(carrierParty.getPartyName())
                                            );
                                    Optional.ofNullable(elem.getTransportMeans())
                                            .ifPresent(transportMeans -> transportistaBuilder
                                                    .placaDelVehiculo(transportMeans.getLicensePlateId())
                                            );
                                    Optional.ofNullable(elem.getDriverPerson())
                                            .ifPresent(driverPerson -> transportistaBuilder
                                                    .choferTipoDocumentoIdentidad(driverPerson.getId())
                                                    .choferTipoDocumentoIdentidad(driverPerson.getId_schemeId())
                                            );

                                    transportista = transportistaBuilder.build();
                                }

                                return Envio.builder()
                                        .tipoTraslado(elem.getHandlingCode())
                                        .motivoTraslado(elem.getInformation())
                                        .pesoTotal(elem.getGrossWeightMeasure())
                                        .pesoTotalUnidadMedida(elem.getGrossWeightMeasure_unitCode())
                                        .numeroDeBultos(elem.getTotalTransportHandlingUnitQuantity())
                                        .transbordoProgramado(elem.getSplitConsignmentIndicator())
                                        .tipoModalidadTraslado(elem.getTransportModeCode())
                                        .fechaTraslado(elem.getStartDate())
                                        .transportista(transportista)
                                        .destino(Destino.builder()
                                                .ubigeo(elem.getDeliveryAddress_id())
                                                .direccion(elem.getDeliveryAddress_line())
                                                .build()
                                        )
                                        .numeroDeContenedor(elem.getTransportEquipment_id())
                                        .partida(Partida.builder()
                                                .ubigeo(elem.getOriginAddress_id())
                                                .direccion(elem.getOriginAddress_streetName())
                                                .build()
                                        )
                                        .codigoDePuerto(elem.getFirstArrivalPortLocation_id())
                                        .build();
                            }
                    )
                    .orElse(null)
            );

            builder.detalles(Optional.ofNullable(xmlDocument.getLines())
                    .orElse(Collections.emptyList())
                    .stream()
                    .map(elem -> DespatchAdviceItem.builder()
                            .cantidad(elem.getDeliveredQuantity())
                            .unidadMedida(elem.getDeliveredQuantity_unitCode())
                            .descripcion(elem.getItemName())
                            .codigo(elem.getSellersItemIdentification_id())
                            .codigoSunat(elem.getItemClassificationCode())
                            .build()
                    )
                    .collect(Collectors.toList())
            );

            return builder.build();
        }
    }

    private static <T> T unmarshall(InputStream documentOXML, InputSource inputSource) throws JAXBException {
        Map<String, Object> properties = new HashMap<>();
        properties.put(JAXBContextProperties.OXM_METADATA_SOURCE, List.of(
                Objects.requireNonNull(documentOXML)
        ));

        JAXBContext jaxbContext = JAXBContextFactory.createContext(Unmarshall.class.getPackageName(), null, properties);
        Unmarshaller unmarshaller = jaxbContext.createUnmarshaller();
        return (T) unmarshaller.unmarshal(inputSource);
    }

    private static void enrichSalesDocument(XMLSalesDocument xmlSalesDocument, SalesDocument.SalesDocumentBuilder<?, ?> builder) {
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

    private static void enrichNote(XMLSalesDocument xmlSalesDocument, Note.NoteBuilder<?, ?> builder) {
        enrichDocument(xmlSalesDocument, builder);

        // ComprobanteAfectado
        builder.comprobanteAfectadoSerieNumero(xmlSalesDocument.getDiscrepancyResponse_referenceId());
        builder.comprobanteAfectadoTipo(xmlSalesDocument.getDiscrepancyResponse_responseCode());
        builder.sustentoDescripcion(xmlSalesDocument.getDiscrepancyResponse_description());

        // Total importe
        builder.totalImporte(Mapper.mapTotalImporteNote(xmlSalesDocument.getMonetaryTotal()));
    }

    private static void enrichBasePercepcionRetencion(XMLPercepcionRetencion xmlDocument, BasePercepcionRetencion.BasePercepcionRetencionBuilder<?, ?> builder) {
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

    private static void enrichSunatDocument(XMLSunatDocument xmlDocument, SunatDocument.SunatDocumentBuilder<?, ?> builder) {
        enrichDocument(xmlDocument, builder);

        // Numero
        String[] split = xmlDocument.getDocumentId().split("-");
        if (split.length == 3) {
            builder.numero(Integer.parseInt(split[2]));
        }

        // Fecha emision comprobante
        builder.fechaEmisionComprobantes(xmlDocument.getReferenceDate());
    }

    private static void enrichDocument(XMLSalesDocument xmlSalesDocument, Document.DocumentBuilder<?, ?> builder) {
        builder.moneda(xmlSalesDocument.getDocumentCurrencyCode());
        builder.fechaEmision(xmlSalesDocument.getIssueDate());
        builder.proveedor(Mapper.mapProveedor(xmlSalesDocument.getAccountingSupplierParty()));
        builder.firmante(Mapper.mapFirmante(xmlSalesDocument.getSignature()));
    }

    private static void enrichDocument(XMLPercepcionRetencion xmlDocument, Document.DocumentBuilder<?, ?> builder) {
        builder.moneda(xmlDocument.getTotalInvoiceAmount_currencyId());
        builder.fechaEmision(xmlDocument.getIssueDate());
        builder.proveedor(Mapper.mapProveedor(xmlDocument.getAccountingSupplierParty()));
        builder.firmante(Mapper.mapFirmante(xmlDocument.getSignature()));
    }

    private static void enrichDocument(XMLSunatDocument xmlDocument, Document.DocumentBuilder<?, ?> builder) {
        builder.moneda(null);
        builder.fechaEmision(xmlDocument.getIssueDate());
        builder.proveedor(Mapper.mapProveedor(xmlDocument.getAccountingSupplierParty()));
        builder.firmante(Mapper.mapFirmante(xmlDocument.getSignature()));
    }

}
