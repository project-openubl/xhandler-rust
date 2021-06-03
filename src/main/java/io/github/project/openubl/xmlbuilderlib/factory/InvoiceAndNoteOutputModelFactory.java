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
package io.github.project.openubl.xmlbuilderlib.factory;

import io.github.project.openubl.xmlbuilderlib.clock.SystemClock;
import io.github.project.openubl.xmlbuilderlib.config.Config;
import io.github.project.openubl.xmlbuilderlib.factory.common.ClienteOutputModelFactory;
import io.github.project.openubl.xmlbuilderlib.factory.common.FirmanteOutputModelFactory;
import io.github.project.openubl.xmlbuilderlib.factory.common.FormaPagoOutputModelFactory;
import io.github.project.openubl.xmlbuilderlib.factory.common.ProveedorOutputModelFactory;
import io.github.project.openubl.xmlbuilderlib.models.catalogs.*;
import io.github.project.openubl.xmlbuilderlib.models.input.common.CuotaDePagoInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.DocumentInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.invoice.InvoiceInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.note.NoteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.note.creditNote.CreditNoteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.input.standard.note.debitNote.DebitNoteInputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.common.FormaPagoOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.*;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.invoice.InvoiceOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.note.NoteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.note.creditNote.CreditNoteOutputModel;
import io.github.project.openubl.xmlbuilderlib.models.output.standard.note.debitNote.DebitNoteOutputModel;

import java.math.BigDecimal;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import static io.github.project.openubl.xmlbuilderlib.utils.DateUtils.toGregorianCalendarDate;
import static io.github.project.openubl.xmlbuilderlib.utils.DateUtils.toGregorianCalendarTime;

public class InvoiceAndNoteOutputModelFactory {

    private InvoiceAndNoteOutputModelFactory() {
        // Only static methods
    }

    public static InvoiceOutputModel getInvoiceOutput(InvoiceInputModel input, Config config, SystemClock systemClock) {
        InvoiceOutputModel.Builder builder = InvoiceOutputModel.Builder.anInvoiceOutputModel();

        if (input.getSerie().matches("^[F|b].*$")) {
            builder.withTipoInvoice(Catalog1.FACTURA);
        } else if (input.getSerie().matches("^[B|b].*$")) {
            builder.withTipoInvoice(Catalog1.BOLETA);
        } else {
            throw new IllegalStateException("Invalid Serie");
        }

        enrichDocument(input, builder, config, systemClock);

        // Forma de pago
        InvoiceOutputModel tmpOutput = builder.build();
        builder.withFormaPago(getFormaPago(
                input.getCuotasDePago(), tmpOutput.getTotales(), systemClock.getTimeZone()
        ));

        return builder.build();
    }

    public static CreditNoteOutputModel getCreditNoteOutput(CreditNoteInputModel input, Config config, SystemClock systemClock) {
        CreditNoteOutputModel.Builder builder = CreditNoteOutputModel.Builder.aCreditNoteOutputModel()
                .withTipoNota(
                        input.getTipoNota() != null
                                ? Catalog.valueOfCode(Catalog9.class, input.getTipoNota()).orElseThrow(Catalog.invalidCatalogValue)
                                : config.getDefaultTipoNotaCredito()
                );

        enrichNote(input, builder);
        enrichDocument(input, builder, config, systemClock);

        // Forma de pago
        CreditNoteOutputModel tmpOutput = builder.build();
        builder.withFormaPago(getFormaPago(
                input.getCuotasDePago(), tmpOutput.getTotales(), systemClock.getTimeZone()
        ));

        return builder.build();
    }

    public static DebitNoteOutputModel getDebitNoteOutput(DebitNoteInputModel input, Config config, SystemClock systemClock) {
        DebitNoteOutputModel.Builder builder = DebitNoteOutputModel.Builder.aDebitNoteOutputModel()
                .withTipoNota(
                        input.getTipoNota() != null
                                ? Catalog.valueOfCode(Catalog10.class, input.getTipoNota()).orElseThrow(Catalog.invalidCatalogValue)
                                : config.getDefaultTipoNotaDebito()
                );

        enrichNote(input, builder);
        enrichDocument(input, builder, config, systemClock);
        return builder.build();
    }

    // Enrich

    private static void enrichDocument(DocumentInputModel input, DocumentOutputModel.Builder builder, Config config, SystemClock systemClock) {
        builder.withMoneda(config.getDefaultMoneda())
                .withSerieNumero(input.getSerie().toUpperCase() + "-" + input.getNumero());

        // Fecha y hora de emision
        long fechaEmision = input.getFechaEmision() != null ? input.getFechaEmision() : systemClock.getCalendarInstance().getTimeInMillis();
        builder.withFechaEmision(toGregorianCalendarDate(fechaEmision, systemClock.getTimeZone()))
                .withHoraEmision(toGregorianCalendarTime(fechaEmision, systemClock.getTimeZone()));

        // Proveedor
        builder.withProveedor(ProveedorOutputModelFactory.getProveedor(input.getProveedor()));

        // Cliente
        builder.withCliente(ClienteOutputModelFactory.getCliente(input.getCliente()));

        // Firmante
        builder.withFirmante(
                input.getFirmante() != null
                        ? FirmanteOutputModelFactory.getFirmante(input.getFirmante())
                        : FirmanteOutputModelFactory.getFirmante(input.getProveedor())
        );

        // Guias de remisión relacionadas
        builder.withGuiasRemisionRelacionadas(input.getGuiasRemisionRelacionadas() != null ?
                input.getGuiasRemisionRelacionadas().stream()
                        .map(guiaRemisionInput -> {
                            GuiaRemisionRelacionadaOutputModel guiaRemisionOutput = new GuiaRemisionRelacionadaOutputModel();
                            guiaRemisionOutput.setSerieNumero(guiaRemisionInput.getSerieNumero());
                            guiaRemisionOutput.setTipoDocumento(Catalog.valueOfCode(Catalog1_Guia.class, guiaRemisionInput.getTipoDocumento()).orElseThrow(Catalog.invalidCatalogValue));
                            return guiaRemisionOutput;
                        })
                        .collect(Collectors.toList())
                : Collections.emptyList()
        );

        // Detalle
        List<DocumentLineOutputModel> lineOutput = input.getDetalle().stream()
                .map(f -> DocumentLineOutputModelFactory.getDocumentLineOutput(f, config, systemClock))
                .collect(Collectors.toList());
        builder.withDetalle(lineOutput);


        DocumentImpuestosOutputModel.Builder impuestosBuilder = DocumentImpuestosOutputModel.Builder.aDocumentImpuestosOutputModel();

        // Importe total de impuestos
        BigDecimal importeTotalImpuestosIgv = lineOutput.stream()
                .map(DocumentLineOutputModel::getImpuestos)
                .filter(p -> p.getIgv().getCategoria().equals(Catalog5.IGV) ||
                        p.getIgv().getCategoria().equals(Catalog5.IMPUESTO_ARROZ_PILADO)
                )
                .map(m -> m.getIgv().getImporte())
                .reduce(BigDecimal.ZERO, BigDecimal::add);
        BigDecimal importeTotalImpuestosIcb = lineOutput.stream()
                .map(DocumentLineOutputModel::getImpuestos)
                .map(DocumentLineImpuestosOutputModel::getIcb)
                .filter(Objects::nonNull)
                .map(ImpuestoOutputModel::getImporte)
                .reduce(BigDecimal.ZERO, BigDecimal::add);

        impuestosBuilder.withImporteTotal(
                importeTotalImpuestosIgv.add(importeTotalImpuestosIcb)
        );

        // Gravado
        ImpuestoTotalOutputModel gravado = getImpuestoTotal(lineOutput, Catalog5.IGV);
        if (gravado.getBaseImponible().compareTo(BigDecimal.ZERO) > 0) {
            impuestosBuilder.withGravadas(gravado);
        }

        // Exonerado
        ImpuestoTotalOutputModel exonerado = getImpuestoTotal(lineOutput, Catalog5.EXONERADO);
        if (exonerado.getBaseImponible().compareTo(BigDecimal.ZERO) > 0) {
            impuestosBuilder.withExoneradas(exonerado);
        }

        // Inafecto
        ImpuestoTotalOutputModel inafecto = getImpuestoTotal(lineOutput, Catalog5.INAFECTO);
        if (inafecto.getBaseImponible().compareTo(BigDecimal.ZERO) > 0) {
            impuestosBuilder.withInafectas(inafecto);
        }

        // Gratuito
        ImpuestoTotalOutputModel gratuito = getImpuestoTotal(lineOutput, Catalog5.GRATUITO);
        if (gratuito.getBaseImponible().compareTo(BigDecimal.ZERO) > 0) {
            impuestosBuilder.withInafectas(gratuito);
        }

        // IVAP
        java.util.function.Supplier<Stream<DocumentLineOutputModel>> ivapStream = () -> lineOutput.stream()
                .filter(i -> i.getImpuestos().getIgv().getTipo().getTaxCategory().equals(Catalog5.IMPUESTO_ARROZ_PILADO));

        BigDecimal ivapImporte = ivapStream.get()
                .map(DocumentLineOutputModel::getImpuestos)
                .map(DocumentLineImpuestosOutputModel::getIgv)
                .map(ImpuestoOutputModel::getImporte)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
        BigDecimal ivapBaseImponible = ivapStream.get()
                .map(DocumentLineOutputModel::getImpuestos)
                .map(DocumentLineImpuestosOutputModel::getIgv)
                .map(ImpuestoDetalladoOutputModel::getBaseImponible)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
        if (ivapImporte.compareTo(BigDecimal.ZERO) > 0) {
            impuestosBuilder.withIvap(ImpuestoTotalOutputModel.Builder.anImpuestoTotalOutputModel()
                    .withCategoria(Catalog5.IMPUESTO_ARROZ_PILADO)
                    .withImporte(ivapImporte)
                    .withBaseImponible(ivapBaseImponible)
                    .build());
        }

        // ICB
        BigDecimal icbImporte = lineOutput.stream()
                .map(DocumentLineOutputModel::getImpuestos)
                .map(DocumentLineImpuestosOutputModel::getIcb)
                .filter(Objects::nonNull)
                .map(ImpuestoOutputModel::getImporte)
                .reduce(BigDecimal.ZERO, BigDecimal::add);

        if (icbImporte.compareTo(BigDecimal.ZERO) > 0) {
            impuestosBuilder.withIcb(ImpuestoTotalICBOutputModel.Builder.anImpuestoTotalICBOutputModel()
                    .withCategoria(Catalog5.ICBPER)
                    .withImporte(icbImporte)
                    .build()
            );
        }


        builder.withImpuestos(impuestosBuilder.build());


        // Importe total
        BigDecimal valorVentaSinImpuestos = lineOutput.stream()
                .filter(p -> !p.getImpuestos().getIgv().getTipo().getTaxCategory().equals(Catalog5.GRATUITO))
                .map(DocumentLineOutputModel::getValorVentaSinImpuestos)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
        BigDecimal valorVentaConImpuestos = lineOutput.stream()
                .filter(p -> !p.getImpuestos().getIgv().getTipo().getTaxCategory().equals(Catalog5.GRATUITO))
                .map(f -> f.getImpuestos().getImporteTotal())
                .reduce(BigDecimal.ZERO, BigDecimal::add)
                .add(valorVentaSinImpuestos);

        builder.withTotales(DocumentMonetaryTotalOutputModel.Builder.aDocumentMonetaryTotalOutputModel()
                .withValorVentaSinImpuestos(valorVentaSinImpuestos)
                .withValorVentaConImpuestos(valorVentaConImpuestos)
                .withImporteTotal(valorVentaConImpuestos)
                .build()
        );
    }

    private static void enrichNote(NoteInputModel input, NoteOutputModel.Builder builder) {
        builder.withSerieNumeroComprobanteAfectado(input.getSerieNumeroComprobanteAfectado())
                .withDescripcionSustentoDeNota(input.getDescripcionSustentoDeNota());

        if (input.getSerieNumeroComprobanteAfectado().matches("^[F|b].*$")) {
            builder.withTipoDocumentoComprobanteAfectado(Catalog1.FACTURA);
        } else if (input.getSerie().matches("^[B|b].*$")) {
            builder.withTipoDocumentoComprobanteAfectado(Catalog1.BOLETA);
        } else {
            throw new IllegalStateException("Invalid Serie");
        }
    }

    private static ImpuestoTotalOutputModel getImpuestoTotal(List<DocumentLineOutputModel> lineOutput, Catalog5 categoria) {
        java.util.function.Supplier<Stream<DocumentLineOutputModel>> stream = () -> lineOutput.stream()
                .filter(i -> i.getImpuestos().getIgv().getTipo().getTaxCategory().equals(categoria));

        BigDecimal importe = stream.get()
                .map(DocumentLineOutputModel::getImpuestos)
                .map(DocumentLineImpuestosOutputModel::getIgv)
                .map(ImpuestoOutputModel::getImporte)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
        BigDecimal baseImponible = stream.get()
                .map(DocumentLineOutputModel::getImpuestos)
                .map(DocumentLineImpuestosOutputModel::getIgv)
                .map(ImpuestoDetalladoOutputModel::getBaseImponible)
                .reduce(BigDecimal.ZERO, BigDecimal::add);

        return ImpuestoTotalOutputModel.Builder.anImpuestoTotalOutputModel()
                .withCategoria(categoria)
                .withImporte(importe)
                .withBaseImponible(baseImponible)
                .build();
    }

    private static FormaPagoOutputModel getFormaPago(List<CuotaDePagoInputModel> cuotas, DocumentMonetaryTotalOutputModel totales, TimeZone timeZone) {
        return FormaPagoOutputModelFactory.getFormaPago(
                cuotas, totales.getValorVentaConImpuestos(), timeZone
        );
    }
}
