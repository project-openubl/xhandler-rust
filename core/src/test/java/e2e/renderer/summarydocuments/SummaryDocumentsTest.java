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
package e2e.renderer.summarydocuments;

import e2e.AbstractTest;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog1;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog19;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog1_Invoice;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog6;
import io.github.project.openubl.xbuilder.content.models.common.Cliente;
import io.github.project.openubl.xbuilder.content.models.common.Proveedor;
import io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocuments;
import io.github.project.openubl.xbuilder.content.models.sunat.baja.VoidedDocumentsItem;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.Comprobante;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteAfectado;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteImpuestos;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.ComprobanteValorVenta;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocuments;
import io.github.project.openubl.xbuilder.content.models.sunat.resumen.SummaryDocumentsItem;
import io.github.project.openubl.xbuilder.enricher.ContentEnricher;
import io.github.project.openubl.xbuilder.renderer.TemplateProducer;
import io.quarkus.qute.Template;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;
import java.time.LocalDate;

import static e2e.renderer.XMLAssertUtils.assertSendSunat;
import static e2e.renderer.XMLAssertUtils.assertSnapshot;

public class SummaryDocumentsTest extends AbstractTest {

    @Test
    public void testMultipleVoidedDocuments() throws Exception {
        // Given
        SummaryDocuments input = SummaryDocuments.builder()
                .numero(1)
                .fechaEmisionComprobantes(dateProvider.now().minusDays(2))
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

        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getSummaryDocuments();
        String xml = template.data(input).render();

        // Then
        assertSnapshot(xml, getClass(), "summaryDocuments.xml");
        // TODO uncomment following test
        // assertSendSunat(xml);
    }

}
