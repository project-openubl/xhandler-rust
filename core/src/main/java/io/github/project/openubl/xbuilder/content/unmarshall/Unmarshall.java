package io.github.project.openubl.xbuilder.content.unmarshall;

import io.github.project.openubl.xbuilder.content.catalogs.Catalog;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog12;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog12_Anticipo;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog5;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog53_Anticipo;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog53_DescuentoGlobal;
import io.github.project.openubl.xbuilder.content.catalogs.CatalogContadoCredito;
import io.github.project.openubl.xbuilder.content.models.common.Cliente;
import io.github.project.openubl.xbuilder.content.models.common.Contacto;
import io.github.project.openubl.xbuilder.content.models.common.Direccion;
import io.github.project.openubl.xbuilder.content.models.common.Document;
import io.github.project.openubl.xbuilder.content.models.common.Firmante;
import io.github.project.openubl.xbuilder.content.models.common.Proveedor;
import io.github.project.openubl.xbuilder.content.models.standard.general.Anticipo;
import io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.CuotaDePago;
import io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.Descuento;
import io.github.project.openubl.xbuilder.content.models.standard.general.Detraccion;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoRelacionado;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoVentaDetalle;
import io.github.project.openubl.xbuilder.content.models.standard.general.FormaDePago;
import io.github.project.openubl.xbuilder.content.models.standard.general.Guia;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.content.models.standard.general.Note;
import io.github.project.openubl.xbuilder.content.models.standard.general.Percepcion;
import io.github.project.openubl.xbuilder.content.models.standard.general.SalesDocument;
import io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteInvoice;
import io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.TotalImpuestos;
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
import java.math.RoundingMode;
import java.util.ArrayList;
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
            builder.formaDePago(mapFormaDePago(xmlSalesDocument.getPaymentTerms()));

            // Total importe
            builder.totalImporte(mapTotalImporteInvoice(xmlSalesDocument.getMonetaryTotal()));

            // Direccion entrega
            builder.direccionEntrega(mapDireccion(xmlSalesDocument.getDeliveryLocation()));

            // Detraccion
            builder.detraccion(mapDetraccion(xmlSalesDocument.getPaymentMeans(), xmlSalesDocument.getPaymentTerms()));

            // Percepcion
            builder.percepcion(mapPercepcion(xmlSalesDocument.getPaymentTerms(), xmlSalesDocument.getAllowanceCharges()));

            // Anticipos
            builder.anticipos(mapAnticipos(xmlSalesDocument.getAdditionalDocumentReferences(), xmlSalesDocument.getPrepaidPayments(), xmlSalesDocument.getAllowanceCharges()));

            // Descuentos
            builder.descuentos(mapDescuentos(xmlSalesDocument.getAllowanceCharges()));

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

    public static void enrichNote(XMLSalesDocument xmlSalesDocument, Note.NoteBuilder<?, ?> builder) {
        enrichDocument(xmlSalesDocument, builder);

        // ComprobanteAfectado
        builder.comprobanteAfectadoSerieNumero(xmlSalesDocument.getDiscrepancyResponse_referenceId());
        builder.comprobanteAfectadoTipo(xmlSalesDocument.getDiscrepancyResponse_responseCode());
        builder.sustentoDescripcion(xmlSalesDocument.getDiscrepancyResponse_description());

        // Total importe
        builder.totalImporte(mapTotalImporteNote(xmlSalesDocument.getMonetaryTotal()));
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
        builder.cliente(mapCliente(xmlSalesDocument.getAccountingCustomerParty()));

        // Total impuestos
        builder.totalImpuestos(mapTotalImpuestos(xmlSalesDocument.getTaxTotal()));

        // Guias
        builder.guias(mapGuias(xmlSalesDocument.getDespatchDocumentReferences()));

        // Documentos relacionados
        builder.documentosRelacionados(mapDocumentosRelacionados(xmlSalesDocument.getAdditionalDocumentReferences()));

        // Detalles
        builder.detalles(mapDetalles(xmlSalesDocument.getLines()));
    }

    public static void enrichDocument(XMLSalesDocument xmlSalesDocument, Document.DocumentBuilder<?, ?> builder) {
        builder.moneda(xmlSalesDocument.getDocumentCurrencyCode());
        builder.fechaEmision(xmlSalesDocument.getIssueDate());
        builder.proveedor(mapProveedor(xmlSalesDocument.getAccountingSupplierParty()));
        builder.firmante(mapFirmante(xmlSalesDocument.getSignature()));
    }

    public static BigDecimal mapPorcentaje(BigDecimal number) {
        return Optional.ofNullable(number)
                .map(bigDecimal -> bigDecimal.divide(new BigDecimal("100"), 10, RoundingMode.HALF_EVEN))
                .orElse(null);
    }

    public static Proveedor mapProveedor(XMLSalesDocument.Supplier supplier) {
        if (supplier == null) {
            return null;
        }

        return Proveedor.builder()
                .ruc(supplier.getPartyIdentification_id())
                .nombreComercial(supplier.getPartyName())
                .razonSocial(supplier.getRegistrationName())
                .direccion(mapDireccion(supplier.getAddress()))
                .contacto(mapContacto(supplier.getContact()))
                .build();
    }

    public static Cliente mapCliente(XMLSalesDocument.Customer customer) {
        if (customer == null) {
            return null;
        }

        return Cliente.builder()
                .tipoDocumentoIdentidad(customer.getPartyIdentification_id_schemeId())
                .numeroDocumentoIdentidad(customer.getPartyIdentification_id())
                .nombre(customer.getRegistrationName())
                .direccion(mapDireccion(customer.getAddress()))
                .contacto(mapContacto(customer.getContact()))
                .build();
    }

    public static Direccion mapDireccion(XMLSalesDocument.Address address) {
        if (address == null) {
            return null;
        }

        return Direccion.builder()
                .ubigeo(address.getId())
                .codigoLocal(address.getAddressTypeCode())
                .urbanizacion(address.getCitySubdivisionName())
                .provincia(address.getCityName())
                .departamento(address.getCountrySubEntity())
                .distrito(address.getDistrict())
                .direccion(address.getAddressLine())
                .codigoPais(address.getIdentificationCode())
                .build();
    }

    public static Contacto mapContacto(XMLSalesDocument.Contact contact) {
        if (contact == null) {
            return null;
        }

        return Contacto.builder()
                .telefono(contact.getTelephone())
                .email(contact.getElectronicMail())
                .build();
    }

    public static Firmante mapFirmante(XMLSalesDocument.Signature signature) {
        if (signature == null) {
            return null;
        }

        return Firmante.builder()
                .ruc(signature.getId())
                .razonSocial(signature.getPartyName())
                .build();
    }

    public static TotalImpuestos mapTotalImpuestos(XMLSalesDocument.TaxTotal taxTotal) {
        if (taxTotal == null) {
            return null;
        }

        TotalImpuestos.TotalImpuestosBuilder builder = TotalImpuestos.builder()
                .total(taxTotal.getTaxAmount());

        for (XMLSalesDocument.TaxSubtotal taxSubtotal : taxTotal.getTaxSubtotals()) {
            Catalog5 catalog5 = Catalog
                    .valueOfCode(Catalog5.class, taxSubtotal.getCode())
                    .orElseThrow(Catalog.invalidCatalogValue);
            switch (catalog5) {
                case IGV:
                    builder = builder
                            .gravadoBaseImponible(taxSubtotal.getTaxableAmount())
                            .gravadoImporte(taxSubtotal.getTaxAmount());
                    break;
                case IMPUESTO_ARROZ_PILADO:
                    builder = builder
                            .ivapBaseImponible(taxSubtotal.getTaxableAmount())
                            .ivapImporte(taxSubtotal.getTaxAmount());
                    break;
                case ISC:
                    builder = builder
                            .iscBaseImponible(taxSubtotal.getTaxableAmount())
                            .iscImporte(taxSubtotal.getTaxAmount());
                    break;
                case EXPORTACION:
                    builder = builder
                            .exportacionBaseImponible(taxSubtotal.getTaxableAmount())
                            .exportacionImporte(taxSubtotal.getTaxAmount());
                    break;
                case GRATUITO:
                    builder = builder
                            .gratuitoBaseImponible(taxSubtotal.getTaxableAmount())
                            .gratuitoImporte(taxSubtotal.getTaxAmount());
                    break;
                case EXONERADO:
                    builder = builder
                            .exoneradoBaseImponible(taxSubtotal.getTaxableAmount())
                            .exoneradoImporte(taxSubtotal.getTaxAmount());
                    break;
                case INAFECTO:
                    builder = builder
                            .inafectoBaseImponible(taxSubtotal.getTaxableAmount())
                            .inafectoImporte(taxSubtotal.getTaxAmount());
                    break;
                case ICBPER:
                    builder = builder
                            .icbImporte(taxSubtotal.getTaxAmount());
                    break;
                case OTROS:
                    break;
            }
        }

        return builder.build();
    }

    public static List<Guia> mapGuias(List<XMLSalesDocument.DespatchDocumentReference> despatchDocumentReferences) {
        if (despatchDocumentReferences == null) {
            return Collections.emptyList();
        }

        return despatchDocumentReferences.stream()
                .map(despatchDocumentReference -> Guia.builder()
                        .serieNumero(despatchDocumentReference.getId())
                        .tipoDocumento(despatchDocumentReference.getDocumentTypeCode())
                        .build())
                .collect(Collectors.toList());
    }

    private static List<DocumentoRelacionado> mapDocumentosRelacionados(List<XMLSalesDocument.AdditionalDocumentReference> additionalDocumentReferences) {
        if (additionalDocumentReferences == null) {
            return Collections.emptyList();
        }

        return additionalDocumentReferences.stream()
                .filter(additionalDocumentReference -> Objects.nonNull(additionalDocumentReference.getDocumentTypeCode()))
                .filter(additionalDocumentReference -> {
                    Optional<Catalog12> catalog12 = Catalog
                            .valueOfCode(Catalog12.class, additionalDocumentReference.getDocumentTypeCode());
                    Optional<Catalog12_Anticipo> catalog12_anticipo = Catalog
                            .valueOfCode(Catalog12_Anticipo.class, additionalDocumentReference.getDocumentTypeCode());
                    return catalog12.isPresent() && catalog12_anticipo.isEmpty();
                })
                .map(despatchDocumentReference -> DocumentoRelacionado.builder()
                        .serieNumero(despatchDocumentReference.getId())
                        .tipoDocumento(despatchDocumentReference.getDocumentTypeCode())
                        .build())
                .collect(Collectors.toList());
    }

    private static List<DocumentoVentaDetalle> mapDetalles(List<XMLSalesDocumentLine> lines) {
        if (lines == null) {
            return Collections.emptyList();
        }

        return lines.stream()
                .map(documentLine -> {
                    DocumentoVentaDetalle.DocumentoVentaDetalleBuilder builder = DocumentoVentaDetalle.builder();

                    // Extract taxes
                    XMLSalesDocumentLine.TaxTotalLine taxTotal = documentLine.getTaxTotal();
                    Map<String, Optional<XMLSalesDocumentLine.TaxSubtotalLine>> subTotals = taxTotal.getTaxSubtotals().stream()
                            .collect(Collectors.groupingBy(
                                    XMLSalesDocumentLine.TaxSubtotalLine::getCode,
                                    Collectors.reducing((o, o2) -> o) // Only one element per type is expected
                            ));

                    // ISC
                    Optional<XMLSalesDocumentLine.TaxSubtotalLine> iscTaxSubtotal = subTotals.getOrDefault(Catalog5.ISC.getCode(), Optional.empty());
                    iscTaxSubtotal.ifPresent(taxSubtotalLine -> {
                        builder.iscBaseImponible(taxSubtotalLine.getTaxableAmount());
                        builder.isc(taxSubtotalLine.getTaxAmount());
                        builder.tasaIsc(mapPorcentaje(taxSubtotalLine.getPercent()));
                        builder.iscTipo(taxSubtotalLine.getTierRange());
                    });

                    // IGV
                    Optional<XMLSalesDocumentLine.TaxSubtotalLine> igvTaxSubtotal = taxTotal.getTaxSubtotals().stream()
                            .filter(line -> !Objects.equals(line.getCode(), Catalog5.ISC.getCode()) && !Objects.equals(line.getCode(), Catalog5.ICBPER.getCode()))
                            .findFirst();
                    igvTaxSubtotal.ifPresent(taxSubtotalLine -> {
                        builder.igvBaseImponible(taxSubtotalLine.getTaxableAmount());
                        builder.igv(taxSubtotalLine.getTaxAmount());
                        builder.tasaIgv(mapPorcentaje(taxSubtotalLine.getPercent()));
                        builder.igvTipo(taxSubtotalLine.getTaxExemptionReasonCode());
                    });

                    // ICB
                    Optional<XMLSalesDocumentLine.TaxSubtotalLine> icbTaxSubtotal = subTotals.getOrDefault(Catalog5.ICBPER.getCode(), Optional.empty());
                    icbTaxSubtotal.ifPresent(taxSubtotalLine -> {
                        builder.icb(taxSubtotalLine.getTaxAmount());
                        builder.tasaIcb(taxSubtotalLine.getPerUnitAmount());
                        builder.icbAplica(taxSubtotalLine.getTaxAmount() != null && taxSubtotalLine.getTaxAmount().compareTo(BigDecimal.ZERO) > 0);
                    });

                    return builder
                            .descripcion(documentLine.getDescription())
                            .unidadMedida(documentLine.getQuantity_unitCode())
                            .cantidad(documentLine.getQuantity())
                            .precio(documentLine.getPriceAmount())
                            .precioReferencia(documentLine.getAlternativeConditionPrice_priceAmount())
                            .precioReferenciaTipo(documentLine.getAlternativeConditionPrice_priceTypeCode())
                            .totalImpuestos(taxTotal.getTaxAmount())
                            .build();
                })
                .collect(Collectors.toList());
    }

    private static FormaDePago mapFormaDePago(List<XMLSalesDocument.PaymentTerms> paymentTerms) {
        if (paymentTerms == null) {
            return null;
        }

        List<XMLSalesDocument.PaymentTerms> formasDePago = paymentTerms.stream()
                .filter(elem -> Objects.equals(elem.getId(), "FormaPago"))
                .collect(Collectors.toList());
        if (formasDePago.isEmpty()) {
            return null;
        }

        XMLSalesDocument.PaymentTerms formaDePago = formasDePago.stream()
                .filter(elem -> Objects.equals(elem.getPaymentMeansID(), CatalogContadoCredito.CREDITO.getCode()) ||
                        Objects.equals(elem.getPaymentMeansID(), CatalogContadoCredito.CONTADO.getCode())
                )
                .findFirst()
                .orElse(new XMLSalesDocument.PaymentTerms());

        List<XMLSalesDocument.PaymentTerms> cuotas = formasDePago.stream()
                .filter(elem -> elem.getPaymentMeansID().startsWith("Cuota"))
                .collect(Collectors.toList());

        return FormaDePago.builder()
                .tipo(formaDePago.getPaymentMeansID())
                .total(formaDePago.getAmount())
                .cuotas(cuotas.stream()
                        .map(elem -> CuotaDePago.builder()
                                .importe(elem.getAmount())
                                .fechaPago(elem.getPaymentDueDate())
                                .build())
                        .collect(Collectors.toList())
                )
                .build();
    }

    private static TotalImporteInvoice mapTotalImporteInvoice(XMLSalesDocument.MonetaryTotal monetaryTotal) {
        if (monetaryTotal == null) {
            return null;
        }

        return TotalImporteInvoice.builder()
                .importe(monetaryTotal.getPayableAmount())
                .importeSinImpuestos(monetaryTotal.getLineExtensionAmount())
                .importeConImpuestos(monetaryTotal.getTaxInclusiveAmount())
                .anticipos(monetaryTotal.getPrepaidAmount())
                .descuentos(monetaryTotal.getAllowanceTotalAmount())
                .build();
    }

    private static TotalImporteNote mapTotalImporteNote(XMLSalesDocument.MonetaryTotal monetaryTotal) {
        if (monetaryTotal == null) {
            return null;
        }

        return TotalImporteNote.builder()
                .importe(monetaryTotal.getPayableAmount())
                .importeSinImpuestos(monetaryTotal.getLineExtensionAmount())
                .importeConImpuestos(monetaryTotal.getTaxInclusiveAmount())
                .build();
    }

    private static Detraccion mapDetraccion(XMLSalesDocument.PaymentMeans paymentMeans, List<XMLSalesDocument.PaymentTerms> paymentTerms) {
        if (paymentMeans == null || paymentTerms == null) {
            return null;
        }

        Detraccion.DetraccionBuilder builder = Detraccion.builder()
                .medioDePago(paymentMeans.getPaymentMeansCode())
                .cuentaBancaria(paymentMeans.getPayeeFinancialAccount_id());

        paymentTerms.stream()
                .filter(elem -> Objects.equals(elem.getId(), "Detraccion"))
                .findFirst()
                .ifPresent(el -> builder
                        .tipoBienDetraido(el.getPaymentMeansID())
                        .porcentaje(mapPorcentaje(el.getPaymentPercent()))
                        .monto(el.getAmount())
                );

        return builder.build();
    }

    private static Percepcion mapPercepcion(List<XMLSalesDocument.PaymentTerms> paymentTerms, List<XMLSalesDocument.AllowanceCharge> allowanceCharges) {
        if (paymentTerms == null || allowanceCharges == null) {
            return null;
        }

        Optional<XMLSalesDocument.PaymentTerms> paymentTerm = paymentTerms.stream()
                .filter(elem -> Objects.equals(elem.getId(), "Percepcion"))
                .findFirst();
        Optional<XMLSalesDocument.AllowanceCharge> allowanceCharge = allowanceCharges.stream()
                .filter(XMLSalesDocument.AllowanceCharge::getChargeIndicator)
                .findFirst();

        if (paymentTerm.isEmpty() || allowanceCharge.isEmpty()) {
            return null;
        }

        Percepcion.PercepcionBuilder builder = Percepcion.builder();
        paymentTerm.ifPresent(elem -> builder
                .montoTotal(elem.getAmount())
        );
        allowanceCharge.ifPresent(elem -> builder
                .tipo(elem.getAllowanceChargeReasonCode())
                .porcentaje(elem.getMultiplierFactorNumeric())
                .monto(elem.getAmount())
                .montoBase(elem.getBaseAmount())
        );

        return builder.build();
    }

    private static List<Anticipo> mapAnticipos(List<XMLSalesDocument.AdditionalDocumentReference> additionalDocumentReferences, List<XMLSalesDocument.PrepaidPayment> prepaidPayments, List<XMLSalesDocument.AllowanceCharge> allowanceCharges) {
        if (additionalDocumentReferences == null || prepaidPayments == null || allowanceCharges == null) {
            return Collections.emptyList();
        }

        additionalDocumentReferences = additionalDocumentReferences.stream()
                .filter(additionalDocumentReference -> additionalDocumentReference.getDocumentStatusCode() != null)
                .collect(Collectors.toList());

        allowanceCharges = allowanceCharges.stream()
                .filter(allowanceCharge -> !allowanceCharge.getChargeIndicator())
                .filter(allowanceCharge -> Catalog
                        .valueOfCode(Catalog53_Anticipo.class, allowanceCharge.getAllowanceChargeReasonCode())
                        .isPresent()
                )
                .collect(Collectors.toList());

        if (additionalDocumentReferences.size() != prepaidPayments.size() || additionalDocumentReferences.size() != allowanceCharges.size()) {
            return Collections.emptyList();
        }

        List<Anticipo> result = new ArrayList<>();
        for (int i = 0; i < additionalDocumentReferences.size(); i++) {
            XMLSalesDocument.AdditionalDocumentReference additionalDocumentReference = additionalDocumentReferences.get(i);
            XMLSalesDocument.PrepaidPayment prepaidPayment = prepaidPayments.get(i);
            XMLSalesDocument.AllowanceCharge allowanceCharge = allowanceCharges.get(i);

            result.add(Anticipo.builder()
                    .comprobanteSerieNumero(additionalDocumentReference.getId())
                    .comprobanteTipo(additionalDocumentReference.getDocumentTypeCode())
                    .monto(prepaidPayment.getPaidAmount())
                    .tipo(allowanceCharge.getAllowanceChargeReasonCode())
                    .build()
            );
        }

        return result;
    }

    private static List<Descuento> mapDescuentos(List<XMLSalesDocument.AllowanceCharge> allowanceCharges) {
        if (allowanceCharges == null) {
            return Collections.emptyList();
        }

        return allowanceCharges.stream()
                .filter(allowanceCharge -> !allowanceCharge.getChargeIndicator())
                .filter(allowanceCharge -> Catalog
                        .valueOfCode(Catalog53_DescuentoGlobal.class, allowanceCharge.getAllowanceChargeReasonCode())
                        .isPresent()
                )
                .map(allowanceCharge -> Descuento.builder()
                        .tipoDescuento(allowanceCharge.getAllowanceChargeReasonCode())
                        .factor(allowanceCharge.getMultiplierFactorNumeric())
                        .monto(allowanceCharge.getAmount())
                        .montoBase(allowanceCharge.getBaseAmount())
                        .build()
                )
                .collect(Collectors.toList());
    }

}
