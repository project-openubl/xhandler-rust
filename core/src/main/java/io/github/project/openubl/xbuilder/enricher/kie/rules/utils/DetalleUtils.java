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
package io.github.project.openubl.xbuilder.enricher.kie.rules.utils;

import io.github.project.openubl.xbuilder.content.catalogs.Catalog;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog5;
import io.github.project.openubl.xbuilder.content.catalogs.Catalog7;
import io.github.project.openubl.xbuilder.content.models.standard.general.DocumentoVentaDetalle;

import java.math.BigDecimal;
import java.util.List;
import java.util.Objects;
import java.util.function.Supplier;
import java.util.stream.Stream;

public class DetalleUtils {

    public static BigDecimal getImporteSinImpuestos(List<DocumentoVentaDetalle> detalles) {
        return detalles
                .stream()
                .filter(item -> {
                    Catalog7 catalog7 = Catalog
                            .valueOfCode(Catalog7.class, item.getIgvTipo())
                            .orElseThrow(Catalog.invalidCatalogValue);
                    return !catalog7.getTaxCategory().equals(Catalog5.GRATUITO);
                })
                .map(DocumentoVentaDetalle::getIgvBaseImponible)
                .filter(Objects::nonNull)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
    }

    public static BigDecimal getTotalImpuestos(List<DocumentoVentaDetalle> detalles) {
        return detalles
                .stream()
                .filter(detalle -> {
                    Catalog7 catalog7 = Catalog
                            .valueOfCode(Catalog7.class, detalle.getIgvTipo())
                            .orElseThrow(Catalog.invalidCatalogValue);
                    return !catalog7.getTaxCategory().equals(Catalog5.GRATUITO);
                })
                .map(DocumentoVentaDetalle::getTotalImpuestos)
                .filter(Objects::nonNull)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
    }

    public static Impuesto calImpuestoByTipo(List<DocumentoVentaDetalle> detalle, Catalog5 categoria) {
        Supplier<Stream<DocumentoVentaDetalle>> stream = () ->
                detalle
                        .stream()
                        .filter($il -> {
                            Catalog7 catalog7 = Catalog
                                    .valueOfCode(Catalog7.class, $il.getIgvTipo())
                                    .orElseThrow(Catalog.invalidCatalogValue);
                            return catalog7.getTaxCategory().equals(categoria);
                        });

        BigDecimal baseImponible = stream
                .get()
                .map(DocumentoVentaDetalle::getIgvBaseImponible)
                .filter(Objects::nonNull)
                .reduce(BigDecimal.ZERO, BigDecimal::add);
        BigDecimal importe = stream
                .get()
                .map(DocumentoVentaDetalle::getIgv)
                .filter(Objects::nonNull)
                .reduce(BigDecimal.ZERO, BigDecimal::add);

        return Impuesto.builder().importe(importe).baseImponible(baseImponible).build();
    }
}
