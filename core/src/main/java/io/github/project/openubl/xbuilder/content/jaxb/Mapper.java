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
import io.github.project.openubl.xbuilder.content.catalogs.Catalog12;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog12_Anticipo;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog5;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog53_Anticipo;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog53_DescuentoGlobal;
import io.github.project.openubl.xbuilder.content.catalogs.CatalogContadoCredito;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLAddress;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLContact;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLCustomer;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSalesDocument;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSalesDocumentLine;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSignature;
import io.github.project.openubl.xbuilder.content.jaxb.models.XMLSupplier;
import io.github.project.openubl.xbuilder.content.models.common.Cliente;
import io.github.project.openubl.xbuilder.content.models.common.Contacto;
import io.github.project.openubl.xbuilder.content.models.common.Direccion;
import io.github.project.openubl.xbuilder.content.models.common.Firmante;
import io.github.project.openubl.xbuilder.content.models.common.Proveedor;
import io.github.project.openubl.xbuilder.content.models.standard.general.Anticipo;
import io.github.project.openubl.xbuilder.content.models.standard.general.CuotaDePago;
import io.github.project.openubl.xbuilder.content.models.standard.general.Descuento;
import io.github.project.openubl.xbuilder.content.models.standard.general.Detraccion;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoRelacionado;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoVentaDetalle;
import io.github.project.openubl.xbuilder.content.models.standard.general.FormaDePago;
import io.github.project.openubl.xbuilder.content.models.standard.general.Guia;
import io.github.project.openubl.xbuilder.content.models.standard.general.Percepcion;
import io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteInvoice;
import io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.TotalImpuestos;

import java.math.BigDecimal;
import java.math.RoundingMode;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.stream.Collectors;

public class Mapper {

    public static BigDecimal mapPorcentaje(BigDecimal number) {
        return Optional.ofNullable(number)
                .map(bigDecimal -> bigDecimal.divide(new BigDecimal("100"), 10, RoundingMode.HALF_EVEN))
                .orElse(null);
    }

    public static Proveedor mapProveedor(XMLSupplier supplier) {
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

    public static Cliente mapCliente(XMLCustomer customer) {
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

    public static Direccion mapDireccion(XMLAddress address) {
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

    public static Contacto mapContacto(XMLContact contact) {
        if (contact == null) {
            return null;
        }

        return Contacto.builder()
                .telefono(contact.getTelephone())
                .email(contact.getElectronicMail())
                .build();
    }

    public static Firmante mapFirmante(XMLSignature signature) {
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

    public static List<DocumentoRelacionado> mapDocumentosRelacionados(List<XMLSalesDocument.AdditionalDocumentReference> additionalDocumentReferences) {
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

    public static List<DocumentoVentaDetalle> mapDetalles(List<XMLSalesDocumentLine> lines) {
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

    public static FormaDePago mapFormaDePago(List<XMLSalesDocument.PaymentTerms> paymentTerms) {
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

    public static TotalImporteInvoice mapTotalImporteInvoice(XMLSalesDocument.MonetaryTotal monetaryTotal) {
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

    public static TotalImporteNote mapTotalImporteNote(XMLSalesDocument.MonetaryTotal monetaryTotal) {
        if (monetaryTotal == null) {
            return null;
        }

        return TotalImporteNote.builder()
                .importe(monetaryTotal.getPayableAmount())
                .importeSinImpuestos(monetaryTotal.getLineExtensionAmount())
                .importeConImpuestos(monetaryTotal.getTaxInclusiveAmount())
                .build();
    }

    public static Detraccion mapDetraccion(XMLSalesDocument.PaymentMeans paymentMeans, List<XMLSalesDocument.PaymentTerms> paymentTerms) {
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

    public static Percepcion mapPercepcion(List<XMLSalesDocument.PaymentTerms> paymentTerms, List<XMLSalesDocument.AllowanceCharge> allowanceCharges) {
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

    public static List<Anticipo> mapAnticipos(List<XMLSalesDocument.AdditionalDocumentReference> additionalDocumentReferences, List<XMLSalesDocument.PrepaidPayment> prepaidPayments, List<XMLSalesDocument.AllowanceCharge> allowanceCharges) {
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

    public static List<Descuento> mapDescuentos(List<XMLSalesDocument.AllowanceCharge> allowanceCharges) {
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
