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
package e2e.renderer.despatchadvice;

import e2e.AbstractTest;
import e2e.renderer.XMLAssertUtils;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog1;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog18;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog20;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog6;
import io.github.project.openubl.xbuilder.content.jaxb.Unmarshall;
import io.github.project.openubl.xbuilder.content.models.standard.guia.DespatchAdvice;
import io.github.project.openubl.xbuilder.content.models.standard.guia.DespatchAdviceItem;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Destinatario;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Destino;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Envio;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Partida;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Remitente;
import io.github.project.openubl.xbuilder.enricher.ContentEnricher;
import io.github.project.openubl.xbuilder.renderer.TemplateProducer;
import io.quarkus.qute.Template;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;

import static e2e.renderer.XMLAssertUtils.assertSendSunat;
import static e2e.renderer.XMLAssertUtils.assertSnapshot;

public class DespatchAdviceTest extends AbstractTest {

    @Test
    public void testBasicMinData() throws Exception {
        // Given
        DespatchAdvice input = DespatchAdvice.builder()
                .serie("T001")
                .numero(1)
                .tipoComprobante(Catalog1.GUIA_REMISION_REMITENTE.getCode())
                .remitente(Remitente.builder()
                        .ruc("12345678912")
                        .razonSocial("Softgreen S.A.C.")
                        .build()
                )
                .destinatario(Destinatario.builder()
                        .tipoDocumentoIdentidad(Catalog6.DNI.getCode())
                        .numeroDocumentoIdentidad("12345678")
                        .nombre("mi cliente")
                        .build()
                )
                .envio(Envio.builder()
                        .tipoTraslado(Catalog20.TRASLADO_EMISOR_ITINERANTE_CP.getCode())
                        .pesoTotal(BigDecimal.ONE)
                        .pesoTotalUnidadMedida("KG")
                        .transbordoProgramado(false)
                        .tipoModalidadTraslado(Catalog18.TRANSPORTE_PRIVADO.getCode())
                        .fechaTraslado(dateProvider.now())
                        .partida(Partida.builder()
                                .direccion("DireccionOrigen")
                                .ubigeo("010101")
                                .build()
                        )
                        .destino(Destino.builder()
                                .direccion("DireccionDestino")
                                .ubigeo("020202")
                                .build()
                        )
                        .build()
                )
                .detalle(DespatchAdviceItem.builder()
                        .cantidad(new BigDecimal("0.5"))
                        .unidadMedida("KG")
                        .codigo("123456")
                        .build()
                )
                .build();

        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // When
        Template template = TemplateProducer.getInstance().getDespatchAdvice();
        String xml = template.data(input).render();

        DespatchAdvice inputFromXml = Unmarshall.unmarshallDespatchAdvice(xml);
        String reconstructedXml = template.data(inputFromXml).render();

        // Then
        assertSnapshot(xml, reconstructedXml, getClass(), "minData.xml");
        assertSendSunat(xml, XMLAssertUtils.DESPATCH_ADVICE_XSD);
    }

}
