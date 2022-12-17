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

import io.github.project.openubl.quarkus.xbuilder.XBuilder;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog1;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog19;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog1_Invoice;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog22;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog23;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog6;
import io.github.project.openubl.xbuilder.content.models.common.Cliente;
import io.github.project.openubl.xbuilder.content.models.common.Proveedor;
import io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoVentaDetalle;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocuments;
import io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocumentsItem;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.PercepcionRetencionOperacion;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Perception;
import io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Retention;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.Comprobante;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteAfectado;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteImpuestos;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteValorVenta;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocuments;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocumentsItem;
import io.github.project.openubl.xbuilder.enricher.ContentEnricher;
import io.quarkus.qute.Template;

import javax.enterprise.context.ApplicationScoped;
import javax.inject.Inject;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import java.math.BigDecimal;
import java.time.LocalDate;

import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.CREDIT_NOTE;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.DEBIT_NOTE;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.INVOICE;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.PERCEPTION;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.RETENTION;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.SUMMARY_DOCUMENTS;
import static io.github.project.openubl.quarkus.xbuilder.XBuilder.Type.VOIDED_DOCUMENTS;

@Path("/quarkus-xbuilder")
@ApplicationScoped
public class QuarkusXbuilderResource {

    @Inject
    XBuilder xBuilder;

    @GET
    @Path("invoice")
    public String createInvoice() {
        Invoice invoice = getBaseInvoice();

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(invoice);

        Template template = xBuilder.getTemplate(INVOICE);
        return template.data(invoice).render();
    }

    @GET
    @Path("credit-note")
    public String createCreditNote() {
        CreditNote creditNote = getBaseCreditNote();

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(creditNote);

        Template template = xBuilder.getTemplate(CREDIT_NOTE);
        return template.data(creditNote).render();
    }

    @GET
    @Path("debit-note")
    public String createDebitNote() {
        DebitNote debitNote = getBaseDebitNote();

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(debitNote);

        Template template = xBuilder.getTemplate(DEBIT_NOTE);
        return template.data(debitNote).render();
    }

    @GET
    @Path("voided-documents")
    public String createVoidedDocuments() {
        VoidedDocuments voidedDocuments = getVoidedDocuments();

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(voidedDocuments);

        Template template = xBuilder.getTemplate(VOIDED_DOCUMENTS);
        return template.data(voidedDocuments).render();
    }

    @GET
    @Path("summary-documents")
    public String createSummaryDocuments() {
        SummaryDocuments summaryDocuments = getSummaryDocuments();

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(summaryDocuments);

        Template template = xBuilder.getTemplate(SUMMARY_DOCUMENTS);
        return template.data(summaryDocuments).render();
    }

    @GET
    @Path("perception")
    public String createPerception() {
        Perception perception = getPerception();

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(perception);

        Template template = xBuilder.getTemplate(PERCEPTION);
        return template.data(perception).render();
    }

    @GET
    @Path("retention")
    public String createRetention() {
        Retention retention = getRetention();

        ContentEnricher enricher = new ContentEnricher(xBuilder.getDefaults(), () -> LocalDate.of(2022, 1, 25));
        enricher.enrich(retention);

        Template template = xBuilder.getTemplate(RETENTION);
        return template.data(retention).render();
    }

    private Invoice getBaseInvoice() {
        return Invoice
                .builder()
                .serie("F001")
                .numero(1)
                .proveedor(Proveedor.builder().ruc("12345678912").razonSocial("Softgreen S.A.C.").build())
                .cliente(
                        Cliente
                                .builder()
                                .nombre("Carlos Feria")
                                .numeroDocumentoIdentidad("12121212121")
                                .tipoDocumentoIdentidad(Catalog6.RUC.toString())
                                .build()
                )
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .descripcion("Item1")
                                .cantidad(new BigDecimal("10"))
                                .precio(new BigDecimal("100"))
                                .build()
                )
                .build();
    }

    private CreditNote getBaseCreditNote() {
        return CreditNote
                .builder()
                .serie("FC01")
                .numero(1)
                .comprobanteAfectadoSerieNumero("F001-1")
                .sustentoDescripcion("mi sustento")
                .proveedor(Proveedor.builder().ruc("12345678912").razonSocial("Softgreen S.A.C.").build())
                .cliente(
                        Cliente
                                .builder()
                                .nombre("Carlos Feria")
                                .numeroDocumentoIdentidad("12121212121")
                                .tipoDocumentoIdentidad(Catalog6.RUC.toString())
                                .build()
                )
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .descripcion("Item1")
                                .cantidad(new BigDecimal("10"))
                                .precio(new BigDecimal("100"))
                                .build()
                )
                .build();
    }

    private DebitNote getBaseDebitNote() {
        return DebitNote
                .builder()
                .serie("FD01")
                .numero(1)
                .comprobanteAfectadoSerieNumero("F001-1")
                .sustentoDescripcion("mi sustento")
                .proveedor(Proveedor.builder().ruc("12345678912").razonSocial("Softgreen S.A.C.").build())
                .cliente(
                        Cliente
                                .builder()
                                .nombre("Carlos Feria")
                                .numeroDocumentoIdentidad("12121212121")
                                .tipoDocumentoIdentidad(Catalog6.RUC.toString())
                                .build()
                )
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .descripcion("Item1")
                                .cantidad(new BigDecimal("10"))
                                .precio(new BigDecimal("100"))
                                .build()
                )
                .build();
    }

    public VoidedDocuments getVoidedDocuments() {
        return VoidedDocuments.builder()
                .numero(1)
                .fechaEmision(LocalDate.of(2022, 01, 31))
                .fechaEmisionComprobantes(LocalDate.of(2022, 01, 29))
                .proveedor(Proveedor.builder()
                        .ruc("12345678912")
                        .razonSocial("Softgreen S.A.C.")
                        .build()
                )
                .comprobante(VoidedDocumentsItem.builder()
                        .serie("F001")
                        .numero(1)
                        .tipoComprobante(Catalog1_Invoice.FACTURA.getCode())
                        .descripcionSustento("Mi sustento1")
                        .build()
                )
                .comprobante(VoidedDocumentsItem.builder()
                        .serie("F001")
                        .numero(2)
                        .tipoComprobante(Catalog1_Invoice.FACTURA.getCode())
                        .descripcionSustento("Mi sustento2")
                        .build()
                )
                .build();
    }

    public SummaryDocuments getSummaryDocuments() {
        return SummaryDocuments.builder()
                .numero(1)
                .fechaEmision(LocalDate.of(2022, 01, 31))
                .fechaEmisionComprobantes(LocalDate.of(2022, 01, 29))
                .proveedor(Proveedor.builder()
                        .ruc("12345678912")
                        .razonSocial("Softgreen S.A.C.")
                        .build()
                )
                .comprobante(SummaryDocumentsItem.builder()
                        .tipoOperacion(Catalog19.ADICIONAR.toString())
                        .comprobante(Comprobante.builder()
                                .tipoComprobante(Catalog1_Invoice.BOLETA.getCode())//
                                .serieNumero("B001-1")
                                .cliente(Cliente.builder()
                                        .nombre("Carlos Feria")
                                        .numeroDocumentoIdentidad("12345678")
                                        .tipoDocumentoIdentidad(Catalog6.DNI.getCode())
                                        .build()
                                )
                                .impuestos(ComprobanteImpuestos.builder()
                                        .igv(new BigDecimal("18"))
                                        .icb(new BigDecimal(2))
                                        .build()
                                )
                                .valorVenta(ComprobanteValorVenta.builder()
                                        .importeTotal(new BigDecimal("120"))
                                        .gravado(new BigDecimal("120"))
                                        .build()
                                )
                                .build()
                        )
                        .build()
                )
                .comprobante(SummaryDocumentsItem.builder()
                        .tipoOperacion(Catalog19.ADICIONAR.toString())
                        .comprobante(Comprobante.builder()
                                .tipoComprobante(Catalog1.NOTA_CREDITO.getCode())
                                .serieNumero("BC02-2")
                                .comprobanteAfectado(ComprobanteAfectado.builder()
                                        .serieNumero("B002-2")
                                        .tipoComprobante(Catalog1.BOLETA.getCode()) //
                                        .build()
                                )
                                .cliente(Cliente.builder()
                                        .nombre("Carlos Feria")
                                        .numeroDocumentoIdentidad("12345678")
                                        .tipoDocumentoIdentidad(Catalog6.DNI.getCode())//
                                        .build()
                                )
                                .impuestos(ComprobanteImpuestos.builder()
                                        .igv(new BigDecimal("18"))
                                        .build()
                                )
                                .valorVenta(ComprobanteValorVenta.builder()
                                        .importeTotal(new BigDecimal("118"))
                                        .gravado(new BigDecimal("118"))
                                        .build()
                                )
                                .build()
                        )
                        .build()
                )
                .build();
    }

    public Perception getPerception() {
        return Perception.builder()
                .serie("P001")
                .numero(1)
                .fechaEmision(LocalDate.of(2022, 01, 31))
                .proveedor(Proveedor.builder()
                        .ruc("12345678912")
                        .razonSocial("Softgreen S.A.C.")
                        .build()
                )
                .cliente(Cliente.builder()
                        .nombre("Carlos Feria")
                        .numeroDocumentoIdentidad("12121212121")
                        .tipoDocumentoIdentidad(Catalog6.RUC.getCode())
                        .build()
                )
                .importeTotalPercibido(new BigDecimal("10"))
                .importeTotalCobrado(new BigDecimal("210"))
                .tipoRegimen(Catalog22.VENTA_INTERNA.getCode())
                .tipoRegimenPorcentaje(Catalog22.VENTA_INTERNA.getPercent()) //
                .operacion(PercepcionRetencionOperacion.builder()
                        .numeroOperacion(1)
                        .fechaOperacion(LocalDate.of(2022, 01, 31))
                        .importeOperacion(new BigDecimal("100"))
                        .comprobante(io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.ComprobanteAfectado.builder()
                                .tipoComprobante(Catalog1.FACTURA.getCode())
                                .serieNumero("F001-1")
                                .fechaEmision(LocalDate.of(2022, 01, 31))
                                .importeTotal(new BigDecimal("200"))
                                .moneda("PEN")
                                .build()
                        )
                        .build()
                )
                .build();
    }

    public Retention getRetention() {
        return Retention.builder()
                .serie("R001")
                .numero(1)
                .fechaEmision(LocalDate.of(2022, 01, 31))
                .proveedor(Proveedor.builder()
                        .ruc("12345678912")
                        .razonSocial("Softgreen S.A.C.")
                        .build()
                )
                .cliente(Cliente.builder()
                        .nombre("Carlos Feria")
                        .numeroDocumentoIdentidad("12121212121")
                        .tipoDocumentoIdentidad(Catalog6.RUC.getCode())
                        .build()
                )
                .importeTotalRetenido(new BigDecimal("10"))
                .importeTotalPagado(new BigDecimal("200"))
                .tipoRegimen(Catalog23.TASA_TRES.getCode())
                .tipoRegimenPorcentaje(Catalog23.TASA_TRES.getPercent()) //
                .operacion(PercepcionRetencionOperacion.builder()
                        .numeroOperacion(1)
                        .fechaOperacion(LocalDate.of(2022, 01, 31))
                        .importeOperacion(new BigDecimal("100"))
                        .comprobante(io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.ComprobanteAfectado.builder()
                                .tipoComprobante(Catalog1.FACTURA.getCode())
                                .serieNumero("F001-1")
                                .fechaEmision(LocalDate.of(2022, 01, 31))
                                .importeTotal(new BigDecimal("210"))
                                .moneda("PEN")
                                .build()
                        )
                        .build()
                )
                .build();
    }
}
