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
package io.github.project.openubl.quarkus.xbuilder.deployment;

import io.github.project.openubl.quarkus.xbuilder.XBuilder;
import io.github.project.openubl.quarkus.xbuilder.runtime.CustomTemplateLocator;
import io.github.project.openubl.quarkus.xbuilder.runtime.DefaultXBuilder;
import io.github.project.openubl.xbuilder.enricher.kie.RuleFactory;
import io.quarkus.arc.deployment.AdditionalBeanBuildItem;
import io.quarkus.deployment.annotations.BuildProducer;
import io.quarkus.deployment.annotations.BuildStep;
import io.quarkus.deployment.builditem.FeatureBuildItem;
import io.quarkus.deployment.builditem.nativeimage.NativeImageResourceBuildItem;
import io.quarkus.deployment.builditem.nativeimage.ReflectiveClassBuildItem;
import io.quarkus.deployment.builditem.nativeimage.ServiceProviderBuildItem;
import io.quarkus.deployment.util.ServiceUtil;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.Set;

class QuarkusXbuilderProcessor {

    private static final String FEATURE = "quarkus-xbuilder";

    @BuildStep
    FeatureBuildItem feature() {
        return new FeatureBuildItem(FEATURE);
    }

    @BuildStep
    AdditionalBeanBuildItem additionalBeans() {
        return AdditionalBeanBuildItem
                .builder()
                .setUnremovable()
                .addBeanClasses(XBuilder.class, DefaultXBuilder.class, CustomTemplateLocator.class)
                .build();
    }

    @BuildStep
    void registerTemplates(BuildProducer<NativeImageResourceBuildItem> resource) throws URISyntaxException {
        resource.produce(
                new NativeImageResourceBuildItem(
                        "templates/Renderer/creditNote.xml",
                        "templates/Renderer/debitNote.xml",
                        "templates/Renderer/invoice.xml",
                        "templates/Renderer/voidedDocuments.xml",
                        "templates/Renderer/summaryDocuments.xml",
                        "templates/Renderer/perception.xml",
                        "templates/Renderer/retention.xml",

                        "templates/ubl/common/signature.xml",

                        "templates/ubl/standard/include/address.xml",
                        "templates/ubl/standard/include/contact.xml",
                        "templates/ubl/standard/include/customer.xml",
                        "templates/ubl/standard/include/document-line.xml",
                        "templates/ubl/standard/include/documentos-relacionados.xml",
                        "templates/ubl/standard/include/general-data.xml",
                        "templates/ubl/standard/include/guias.xml",
                        "templates/ubl/standard/include/monetary-total.xml",
                        "templates/ubl/standard/include/namespaces.xml",
                        "templates/ubl/standard/include/note/invoice-reference.xml",
                        "templates/ubl/standard/include/payment-terms.xml",
                        "templates/ubl/standard/include/supplier.xml",
                        "templates/ubl/standard/include/tax-total.xml",
                        "templates/ubl/standard/include/ubl-extensions.xml",

                        "templates/ubl/sunat/include/supplier.xml",
                        "templates/ubl/sunat/include/agent-party.xml",
                        "templates/ubl/sunat/include/receiver-party.xml"
                )
        );
        //        resource.produce(new NativeImageResourceDirectoryBuildItem("templates"));
    }

    @BuildStep
    void registerServices(BuildProducer<ServiceProviderBuildItem> services) throws IOException {
        String service = "META-INF/services/" + RuleFactory.class.getName();

        // find out all the implementation classes listed in the service files
        Set<String> implementations = ServiceUtil.classNamesNamedIn(
                Thread.currentThread().getContextClassLoader(),
                service
        );

        // register every listed implementation class so they can be instantiated
        // in native-image at run-time
        services.produce(
                new ServiceProviderBuildItem(RuleFactory.class.getName(), implementations.toArray(new String[0]))
        );
    }

    @BuildStep
    ReflectiveClassBuildItem reflectionLombok() {
        return new ReflectiveClassBuildItem(true, false,
                "io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote$CreditNoteBuilderImpl",
                "io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote$DebitNoteBuilderImpl",
                "io.github.project.openubl.xbuilder.content.models.standard.general.Invoice$InvoiceBuilderImpl",

                "io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocuments$VoidedDocumentsBuilderImpl",
                "io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocuments$SummaryDocumentsBuilderImpl",

                "io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Perception$PerceptionBuilderImpl",
                "io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Retention$RetentionBuilderImpl"
        );
    }

    @BuildStep
    ReflectiveClassBuildItem reflection() {
        return new ReflectiveClassBuildItem(true, false,
                io.github.project.openubl.xbuilder.content.models.common.Cliente.class,
                io.github.project.openubl.xbuilder.content.models.common.Cliente.ClienteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.common.Contacto.class,
                io.github.project.openubl.xbuilder.content.models.common.Contacto.ContactoBuilder.class,
                io.github.project.openubl.xbuilder.content.models.common.Direccion.class,
                io.github.project.openubl.xbuilder.content.models.common.Direccion.DireccionBuilder.class,
                io.github.project.openubl.xbuilder.content.models.common.Document.class,
                io.github.project.openubl.xbuilder.content.models.common.Document.DocumentBuilder.class,
                io.github.project.openubl.xbuilder.content.models.common.Firmante.class,
                io.github.project.openubl.xbuilder.content.models.common.Firmante.FirmanteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.common.Proveedor.class,
                io.github.project.openubl.xbuilder.content.models.common.Proveedor.ProveedorBuilder.class,
                io.github.project.openubl.xbuilder.content.models.common.TipoCambio.class,
                io.github.project.openubl.xbuilder.content.models.common.TipoCambio.TipoCambioBuilder.class,

                io.github.project.openubl.xbuilder.content.models.standard.general.Anticipo.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Anticipo.AnticipoBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.BaseDocumentoTributarioRelacionado.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.BaseDocumentoTributarioRelacionado.BaseDocumentoTributarioRelacionadoBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.CargoDescuento.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.CargoDescuento.CargoDescuentoBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote.CreditNoteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.CuotaDePago.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.CuotaDePago.CuotaDePagoBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote.DebitNoteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Detraccion.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Detraccion.DetraccionBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoRelacionado.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoRelacionado.DocumentoRelacionadoBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoVentaDetalle.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoVentaDetalle.DocumentoVentaDetalleBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.FormaDePago.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.FormaDePago.FormaDePagoBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Guia.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Guia.GuiaBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Invoice.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Invoice.InvoiceBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Note.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Note.NoteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Percepcion.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.Percepcion.PercepcionBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.SalesDocument.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.SalesDocument.SalesDocumentBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporte.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporte.TotalImporteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteInvoice.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteInvoice.TotalImporteInvoiceBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteNote.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImporteNote.TotalImporteNoteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImpuestos.class,
                io.github.project.openubl.xbuilder.content.models.standard.general.TotalImpuestos.TotalImpuestosBuilder.class,

                io.github.project.openubl.xbuilder.content.catalogs.Catalog.class,
                io.github.project.openubl.xbuilder.content.catalogs.CatalogContadoCredito.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog1.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog1_Guia.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog1_Invoice.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog5.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog6.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog7.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog7_1.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog8.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog9.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog10.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog12.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog12_Anticipo.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog12_Doc_Trib_Relacionado_BoletaFactura.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog12_Doc_Trib_Relacionado_NotaCredito.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog12_Doc_Trib_Relacionado_NotaDebito.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog16.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog18.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog19.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog20.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog21.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog22.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog23.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog51.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog52.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog53.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog53_Anticipo.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog54.class,
                io.github.project.openubl.xbuilder.content.catalogs.Catalog59.class,

                io.github.project.openubl.xbuilder.content.models.sunat.SunatDocument.class,
                io.github.project.openubl.xbuilder.content.models.sunat.SunatDocument.SunatDocumentBuilder.class,

                io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocuments.class,
                io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocuments.VoidedDocumentsBuilder.class,
                io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocumentsItem.class,
                io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocumentsItem.VoidedDocumentsItemBuilder.class,

                io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocuments.class,
                io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocuments.SummaryDocumentsBuilder.class,
                io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocumentsItem.class,
                io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocumentsItem.SummaryDocumentsItemBuilder.class,
                io.github.project.openubl.xbuilder.content.models.sunat.resumen.Comprobante.class,
                io.github.project.openubl.xbuilder.content.models.sunat.resumen.Comprobante.ComprobanteBuilder.class,
                io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteAfectado.class,
                io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteAfectado.ComprobanteAfectadoBuilder.class,
                io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteImpuestos.class,
                io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteImpuestos.ComprobanteImpuestosBuilder.class,
                io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteValorVenta.class,
                io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteValorVenta.ComprobanteValorVentaBuilder.class,

                io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Perception.class,
                io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Perception.PerceptionBuilder.class,
                io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Retention.class,
                io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.Retention.RetentionBuilder.class,
                io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.BasePercepcionRetencion.class,
                io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.BasePercepcionRetencion.BasePercepcionRetencionBuilder.class,
                io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.ComprobanteAfectado.class,
                io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.ComprobanteAfectado.ComprobanteAfectadoBuilder.class,
                io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.PercepcionRetencionOperacion.class,
                io.github.project.openubl.xbuilder.content.models.sunat.percepcionretencion.PercepcionRetencionOperacion.PercepcionRetencionOperacionBuilder.class
        );
    }
}
