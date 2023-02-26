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

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
import com.fasterxml.jackson.dataformat.yaml.YAMLGenerator;
import com.fasterxml.jackson.dataformat.yaml.YAMLMapper;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog1;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog18;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog19;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog1_Invoice;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog20;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog22;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog23;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog6;
import io.github.project.openubl.xbuilder.content.models.common.Cliente;
import io.github.project.openubl.xbuilder.content.models.common.Proveedor;
import io.github.project.openubl.xbuilder.content.models.standard.general.CreditNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DebitNote;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoVentaDetalle;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.content.models.standard.guia.DespatchAdvice;
import io.github.project.openubl.xbuilder.content.models.standard.guia.DespatchAdviceItem;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Destinatario;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Destino;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Envio;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Partida;
import io.github.project.openubl.xbuilder.content.models.standard.guia.Remitente;
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
import io.quarkus.test.junit.QuarkusTest;
import io.restassured.http.ContentType;
import io.vertx.core.json.JsonObject;
import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.math.BigDecimal;
import java.net.URISyntaxException;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.LocalDate;
import java.util.Map;

import static io.restassured.RestAssured.given;
import static org.hamcrest.Matchers.is;

@QuarkusTest
public class QuarkusXbuilderResourceTest {

    public YAMLMapper getYamlMapper() {
        YAMLMapper mapper = new YAMLMapper(new YAMLFactory());
        mapper.registerModule(new JavaTimeModule());
        mapper.configure(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS, false);
        mapper.configure(YAMLGenerator.Feature.LITERAL_BLOCK_STYLE, true);
        mapper.setSerializationInclusion(JsonInclude.Include.NON_NULL);
        return mapper;
    }

    @Test
    public void testAllYamlFilesFromSnapshot() throws URISyntaxException, IOException {
        YAMLMapper yamlMapper = getYamlMapper();

        URL url = getClass().getClassLoader().getResource("e2e");
        Path path = Paths.get(url.toURI());
        Files.walk(path, 5)
                .filter(p -> !p.toFile().isDirectory())
                .forEach(p -> {
                    try {
                        Map jsonObject = yamlMapper.readValue(p.toFile(), Map.class);
                        String kind = (String) jsonObject.get("kind");
                        String snapshot = (String) jsonObject.get("snapshot");
                        Map input = (Map) jsonObject.get("input");

                        given()
                                .when()
                                .contentType(ContentType.JSON)
                                .body(input)
                                .post("/quarkus-xbuilder/" + kind + "/from-json")
                                .then()
                                .statusCode(200)
                                .body(is(snapshot));
                    } catch (IOException e) {
                        throw new RuntimeException(e);
                    }
                });
    }

}
