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
package e2e.enricher;

import e2e.AbstractTest;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog16;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog1_Invoice;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog7;
import io.github.project.openubl.xbuilder.content.catalogs.CatalogContadoCredito;
import io.github.project.openubl.xbuilder.content.models.common.Firmante;
import io.github.project.openubl.xbuilder.content.models.common.Proveedor;
import io.github.project.openubl.xbuilder.content.models.standard.general.CuotaDePago;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoDetalle;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.enricher.ContentEnricher;
import io.github.project.openubl.xbuilder.enricher.config.DateProvider;
import io.github.project.openubl.xbuilder.enricher.config.Defaults;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;
import java.time.LocalDate;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class DetalleTest extends AbstractTest {

    @Test
    public void testEnrichUnidadMedida() {
        // Given
        Invoice input = Invoice.builder()
                .detalle(DocumentoDetalle.builder()
                        .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input.getDetalles().forEach(detalle -> {
            assertEquals(defaults.getUnidadMedida(), detalle.getUnidadMedida());
        });
    }

    @Test
    public void testDontEnrichUnidadMedida() {
        // Given
        Invoice input = Invoice.builder()
                .detalle(DocumentoDetalle.builder()
                        .unidadMedida("KG")
                        .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input.getDetalles().forEach(detalle -> {
            assertEquals("KG", detalle.getUnidadMedida());
        });
    }

    @Test
    public void testEnrichIgvTipo() {
        // Given
        Invoice input = Invoice.builder()
                .detalle(DocumentoDetalle.builder()
                        .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input.getDetalles().forEach(detalle -> {
            assertEquals(Catalog7.GRAVADO_OPERACION_ONEROSA.getCode(), detalle.getIgvTipo());
        });
    }

    @Test
    public void testDontEnrichIgvTipo() {
        // Given
        Invoice input = Invoice.builder()
                .detalle(DocumentoDetalle.builder()
                        .igvTipo(Catalog7.INAFECTO_RETIRO.getCode())
                        .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input.getDetalles().forEach(detalle -> {
            assertEquals(Catalog7.INAFECTO_RETIRO.getCode(), detalle.getIgvTipo());
        });
    }

    @Test
    public void testEnrichIcbTasa() {
        // Given
        Invoice input = Invoice.builder()
                .detalle(DocumentoDetalle.builder()
                        .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input.getDetalles().forEach(detalle -> {
            assertEquals(defaults.getIcbTasa(), detalle.getIcbTasa());
        });
    }

    @Test
    public void testDontEnrichIcbTasa() {
        // Given
        Invoice input = Invoice.builder()
                .detalle(DocumentoDetalle.builder()
                        .icbTasa(BigDecimal.TEN)
                        .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input.getDetalles().forEach(detalle -> {
            assertEquals(BigDecimal.TEN, detalle.getIcbTasa());
        });
    }

    @Test
    public void testEnrichIgvTasa() {
        // Given
        Invoice input = Invoice.builder()
                .detalle(DocumentoDetalle.builder()
                        .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input.getDetalles().forEach(detalle -> {
            assertEquals(defaults.getIgvTasa(), detalle.getIgvTasa());
        });
    }

    @Test
    public void testDontEnrichIgvTasa() {
        // Given
        Invoice input = Invoice.builder()
                .detalle(DocumentoDetalle.builder()
                        .igvTasa(BigDecimal.TEN)
                        .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input.getDetalles().forEach(detalle -> {
            assertEquals(BigDecimal.TEN, detalle.getIgvTasa());
        });
    }

    @Test
    public void testEnrichPrecioReferenciaTipo_PrecioConIgv() {
        // Given
        Invoice input = Invoice.builder()
                .detalle(DocumentoDetalle.builder()
                        .igvTipo(Catalog7.GRAVADO_OPERACION_ONEROSA.getCode())
                        .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input.getDetalles().forEach(detalle -> {
            assertEquals(Catalog16.PRECIO_UNITARIO_INCLUYE_IGV.getCode(), detalle.getPrecioReferenciaTipo());
        });
    }

    @Test
    public void testEnrichPrecioReferenciaTipo_ValorReferencial() {
        // Given
        Invoice input = Invoice.builder()
                .detalle(DocumentoDetalle.builder()
                        .igvTipo(Catalog7.GRAVADO_RETIRO_POR_DONACION.getCode())
                        .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input.getDetalles().forEach(detalle -> {
            assertEquals(Catalog16.VALOR_REFERENCIAL_UNITARIO_EN_OPERACIONES_NO_ONEROSAS.getCode(), detalle.getPrecioReferenciaTipo());
        });
    }

}