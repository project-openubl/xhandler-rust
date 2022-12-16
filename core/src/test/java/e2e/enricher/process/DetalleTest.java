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
package e2e.enricher.process;

import e2e.AbstractTest;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog7;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoVentaDetalle;
import io.github.project.openubl.xbuilder.content.models.standard.general.Invoice;
import io.github.project.openubl.xbuilder.enricher.ContentEnricher;
import org.junit.jupiter.api.Test;

import java.math.BigDecimal;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class DetalleTest extends AbstractTest {

    @Test
    public void testEnrichPrecioDeReferencia_precioConImpuestos_OperacionOnerosa() {
        // Given
        Invoice input = Invoice
                .builder()
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .precio(BigDecimal.TEN)
                                .precioConImpuestos(false)
                                .igvTipo(Catalog7.GRAVADO_OPERACION_ONEROSA.getCode())
                                .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(new BigDecimal("11.80"), detalle.getPrecioReferencia());
                });
    }

    @Test
    public void testEnrichPrecioDeReferencia_precioConImpuestos_OperacionNoOnerosa() {
        // Given
        Invoice input = Invoice
                .builder()
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .precio(BigDecimal.TEN)
                                .precioConImpuestos(false)
                                .igvTipo(Catalog7.GRAVADO_RETIRO.getCode())
                                .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(BigDecimal.TEN, detalle.getPrecioReferencia());
                });
    }

    @Test
    public void testEnrichPrecioDeReferencia_precioSinImpuestos_OperacionOnerosa() {
        // Given
        Invoice input = Invoice
                .builder()
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .precio(new BigDecimal("11.80"))
                                .precioConImpuestos(true)
                                .igvTipo(Catalog7.GRAVADO_OPERACION_ONEROSA.getCode())
                                .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(0, BigDecimal.TEN.compareTo(detalle.getPrecioReferencia()));
                });
    }

    @Test
    public void testEnrichPrecioDeReferencia_precioSinImpuestos_OperacionNoOnerosa() {
        // Given
        Invoice input = Invoice
                .builder()
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .precio(new BigDecimal("11.80"))
                                .precioConImpuestos(true)
                                .igvTipo(Catalog7.GRAVADO_RETIRO.getCode())
                                .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(new BigDecimal("11.80"), detalle.getPrecioReferencia());
                });
    }

    @Test
    public void testEnrichIcb() {
        // Given
        Invoice input = Invoice.builder().detalle(DocumentoVentaDetalle.builder().build()).build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(BigDecimal.ZERO, detalle.getIcb());
                });
    }

    @Test
    public void testDontEnrichIcb() {
        // Given
        Invoice input = Invoice
                .builder()
                .detalle(DocumentoVentaDetalle.builder().icb(BigDecimal.TEN).icbAplica(false).build())
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(BigDecimal.TEN, detalle.getIcb());
                });
    }

    @Test
    public void testEnrichIcbAplica() {
        // Given
        Invoice input = Invoice
                .builder()
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .icb(BigDecimal.TEN)
                                .icbAplica(false) // this should be corrected
                                .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertTrue(detalle.isIcbAplica());
                });
    }

    @Test
    public void testEnrichBaseImponible_whenPrecioConImpuestos() {
        // Given
        Invoice input = Invoice
                .builder()
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .cantidad(new BigDecimal(2))
                                .precio(BigDecimal.TEN)
                                .precioConImpuestos(false)
                                .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(0, detalle.getIgvBaseImponible().compareTo(new BigDecimal(20)));
                });
    }

    @Test
    public void testEnrichBaseImponible_whenPrecioSinImpuestos() {
        // Given
        Invoice input = Invoice
                .builder()
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .cantidad(new BigDecimal(2))
                                .precio(new BigDecimal("11.8"))
                                .precioConImpuestos(true)
                                .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(0, detalle.getIgvBaseImponible().compareTo(new BigDecimal(20)));
                });
    }

    @Test
    public void testDontEnrichBaseImponible() {
        // Given
        Invoice input = Invoice
                .builder()
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .cantidad(new BigDecimal(2))
                                .precio(new BigDecimal("10"))
                                .precioConImpuestos(false)
                                .igvBaseImponible(new BigDecimal(999)) // This user defined value should not be altered
                                .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(0, detalle.getIgvBaseImponible().compareTo(new BigDecimal(999)));
                });
    }

    @Test
    public void testEnrichIgv() {
        // Given
        Invoice input = Invoice
                .builder()
                .tasaIgv(new BigDecimal("0.18"))
                .detalle(DocumentoVentaDetalle.builder().igvBaseImponible(new BigDecimal("10")).build())
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(0, detalle.getIgv().compareTo(new BigDecimal("1.8")));
                });
    }

    @Test
    public void testDontEnrichIgv() {
        // Given
        Invoice input = Invoice
                .builder()
                .tasaIgv(new BigDecimal("0.18"))
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .igvBaseImponible(new BigDecimal("10"))
                                .igv(new BigDecimal("999")) // Dont change user defined value
                                .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(0, detalle.getIgv().compareTo(new BigDecimal("999")));
                });
    }

    @Test
    public void testEnrichTotalImpuestos() {
        // Given
        Invoice input = Invoice
                .builder()
                .detalle(DocumentoVentaDetalle.builder().igv(new BigDecimal("10")).icb(new BigDecimal("2")).build())
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(0, detalle.getTotalImpuestos().compareTo(new BigDecimal("12")));
                });
    }

    @Test
    public void testDontEnrichTotalImpuestos() {
        // Given
        Invoice input = Invoice
                .builder()
                .detalle(
                        DocumentoVentaDetalle
                                .builder()
                                .igv(new BigDecimal("10"))
                                .icb(new BigDecimal("2"))
                                .totalImpuestos(new BigDecimal("999")) // Dont change user defined value
                                .build()
                )
                .build();

        // When
        ContentEnricher enricher = new ContentEnricher(defaults, dateProvider);
        enricher.enrich(input);

        // Then
        input
                .getDetalles()
                .forEach(detalle -> {
                    assertEquals(0, detalle.getTotalImpuestos().compareTo(new BigDecimal("999")));
                });
    }
}
