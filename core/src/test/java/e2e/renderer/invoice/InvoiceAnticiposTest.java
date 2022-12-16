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
package e2e.renderer.invoice;

import e2e.AbstractTest;
import e2e.renderer.XMLAssertUtils;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog6;
import io.github.project.openubl.xbuilder.content.models.common.Cliente;
import io.github.project.openubl.xbuilder.content.models.common.Proveedor;
import io.github.project.openubl.xbuilder.content.models.standard.general.Anticipo;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoVentaDetalle;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.enricher.ContentEnricher;
import io.github.project.openubl.xbuilder.renderer.TemplateProducer;
import io.quarkus.qute.Template;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;

public class InvoiceAnticiposTest extends AbstractTest {

    @Test
    public void testFechaVencimiento() throws Exception {
        // Given
        Invoice input = Invoice
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
                                .cantidad(new BigDecimal("2"))
                                .precio(new BigDecimal("100"))
                                .build()
                )
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .descripcion("Item2")
                                .cantidad(new BigDecimal("2"))
                                .precio(new BigDecimal("100"))
                                .build()
                )
                .anticipo(Anticipo.builder().comprobanteSerieNumero("F001-2").monto(new BigDecimal("100")).build())
                .build();

        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getInvoice();
        String xml = template.data(input).render();

        // Then
        XMLAssertUtils.assertSnapshot(xml, getClass(), "minAnticipos.xml");
        XMLAssertUtils.assertSendSunat(xml);
    }
}
